mod error;
mod pubkey;

use cached::proc_macro::cached;
pub use error::*;
use gloo_storage::{LocalStorage, Storage};
use ore_api::{
    consts::{BUS_ADDRESSES, CONFIG_ADDRESS, TREASURY_ADDRESS},
    state::{Bus, Config, Proof, Treasury},
};
use ore_relayer_api::{consts::ESCROW, state::Escrow};
use ore_types::{response::ListTransfersResponse, Transfer};
use ore_utils::AccountDeserialize;
pub use pubkey::*;
use solana_client_wasm::{
    solana_sdk::{
        clock::Clock,
        commitment_config::{CommitmentConfig, CommitmentLevel},
        compute_budget::ComputeBudgetInstruction,
        instruction::Instruction,
        pubkey::Pubkey,
        signature::{Keypair, Signature},
        signer::Signer,
        sysvar,
        transaction::Transaction,
    },
    utils::rpc_config::RpcSendTransactionConfig,
    WasmClient,
};
use solana_extra_wasm::{
    account_decoder::parse_token::UiTokenAccount,
    program::{
        spl_associated_token_account::{
            get_associated_token_address, instruction::create_associated_token_account,
        },
        spl_memo, spl_token,
    },
    transaction_status::{TransactionConfirmationStatus, UiTransactionEncoding},
};
use web_time::Duration;

pub const API_URL: &str = "https://ore-api-lthm.onrender.com";
pub const RPC_URL: &str = "https://emelia-3g4m0w-fast-devnet.helius-rpc.com";
// pub const RPC_URL: &str = "http://localhost:8899";

pub const CU_LIMIT_CLAIM: u32 = 11_000;
pub const CU_LIMIT_MINE: u32 = 500_000;
pub const CU_LIMIT_UPGRADE: u32 = 17_985 + 300;

const RPC_RETRIES: usize = 0;
const GATEWAY_RETRIES: usize = 4;
const CONFIRM_RETRIES: usize = 8;

pub struct Gateway {
    pub rpc: WasmClient,
    api_url: String,
}

impl Gateway {
    pub fn new(api_url: String, rpc_url: String) -> Self {
        Gateway {
            api_url,
            rpc: WasmClient::new(&rpc_url),
        }
    }

    pub async fn get_clock(&self) -> GatewayResult<Clock> {
        let data = self
            .rpc
            .get_account_data(&sysvar::clock::ID)
            .await
            .map_err(GatewayError::from)?;
        bincode::deserialize::<Clock>(&data).or(Err(GatewayError::FailedDeserialization))
    }

    pub async fn get_proof(&self, authority: Pubkey) -> GatewayResult<Proof> {
        let data = self
            .rpc
            .get_account_data(&proof_pubkey(authority))
            .await
            .map_err(GatewayError::from)?;
        Ok(*Proof::try_from_bytes(&data).expect("Failed to parse proof"))
    }

    pub async fn get_config(&self) -> GatewayResult<Config> {
        let data = self
            .rpc
            .get_account_data(&CONFIG_ADDRESS)
            .await
            .map_err(GatewayError::from)?;
        Ok(*Config::try_from_bytes(&data).expect("Failed to parse config account"))
    }

    pub async fn get_escrow(&self, authority: Pubkey) -> GatewayResult<Escrow> {
        let escrow_pubkey =
            Pubkey::find_program_address(&[ESCROW, authority.as_ref()], &ore_relayer_api::id()).0;
        let data = self
            .rpc
            .get_account_data(&escrow_pubkey)
            .await
            .map_err(GatewayError::from)?;
        Ok(*Escrow::try_from_bytes(&data).expect("Failed to parse escrow account"))
    }

    pub async fn get_token_account(
        &self,
        pubkey: &Pubkey,
    ) -> GatewayResult<Option<UiTokenAccount>> {
        self.rpc
            .get_token_account(pubkey)
            .await
            .map_err(GatewayError::from)
    }

    pub async fn send_via_relayer(
        &self,
        ixs: &[Instruction],
        skip_confirm: bool,
    ) -> GatewayResult<Signature> {
        Ok(Signature::new_unique())
        // let mut tx = Transaction::new_with_payer(ixs, Some(&ore_relayer_api::consts::MINER_PUBKEY));
        // let client = reqwest::Client::new();
        // let mut attempts = 0;
        // loop {
        //     log::info!("Attempt: {:?}", attempts);
        //     match client
        //         .post("http://api.ore.supply/relay")
        //         .body(tx)
        //         .send()
        //         .await
        //     {
        //         Ok(res) => {
        //             // Parse sig
        //             let sig: Signature = res.json().await.unwrap();

        //             // Confirm tx
        //             if skip_confirm {
        //                 return Ok(sig);
        //             }
        //             let confirmed = self.confirm_signature(sig).await;
        //             if confirmed.is_ok() {
        //                 return confirmed;
        //             }
        //         }
        //         Err(_) => {
        //             // TODO
        //         }
        //     }

        //     // Retry
        //     async_std::task::sleep(Duration::from_millis(2000)).await;
        //     attempts += 1;
        //     if attempts > GATEWAY_RETRIES {
        //         return Err(GatewayError::TransactionTimeout);
        //     }
        // }
    }

    async fn confirm_signature(&self, sig: Signature) -> GatewayResult<Signature> {
        // Confirm tx
        for _ in 0..CONFIRM_RETRIES {
            // Delay before confirming
            async_std::task::sleep(Duration::from_millis(2000)).await;

            // Fetch transaction status
            match self.rpc.get_signature_statuses(&[sig]).await {
                Ok(signature_statuses) => {
                    for signature_status in signature_statuses {
                        if let Some(signature_status) = signature_status.as_ref() {
                            if signature_status.confirmation_status.is_some() {
                                if let Some(current_commitment) =
                                    signature_status.confirmation_status.as_ref()
                                {
                                    log::info!("Commitment: {:?}", current_commitment);
                                    match current_commitment {
                                        TransactionConfirmationStatus::Processed => {}
                                        TransactionConfirmationStatus::Confirmed
                                        | TransactionConfirmationStatus::Finalized => {
                                            log::info!("Confirmed: true");
                                            return Ok(sig);
                                        }
                                    }
                                }
                            } else {
                                log::info!("No status");
                            }
                        }
                    }
                }

                // Handle confirmation errors
                Err(err) => {
                    log::error!("Error confirming: {:?}", err);
                }
            }
        }

        return Err(GatewayError::TransactionTimeout);
    }

    // asserts that the token account is already initialized
    pub async fn get_token_account_ore_from_pubkey_v1(
        &self,
        pubkey: Pubkey,
    ) -> GatewayResult<Pubkey> {
        let token_account_address = ore_token_account_address_v1(pubkey);
        self.assert_token_account_ore_exists(token_account_address)
            .await
    }

    // asserts that the token account is already initialized
    pub async fn get_token_account_ore_from_pubkey(&self, pubkey: Pubkey) -> GatewayResult<Pubkey> {
        let token_account_address = ore_token_account_address(pubkey);
        self.assert_token_account_ore_exists(token_account_address)
            .await
    }

    // asserts that the token account is already initialized
    async fn assert_token_account_ore_exists(&self, ata: Pubkey) -> GatewayResult<Pubkey> {
        self.rpc
            .get_token_account(&ata)
            .await
            .map_err(GatewayError::from)
            .and_then(|maybe_some_token_account| {
                // assert that ok(none) was not returned
                maybe_some_token_account.ok_or(GatewayError::FailedAta)
            })
            .map(|_| ata)
    }

    // API
    pub async fn get_transfer(&self, sig: String) -> GatewayResult<Transfer> {
        match reqwest::Client::new()
            .get(format!("{}/transfers/{}", self.api_url, sig))
            .send()
            .await
        {
            Ok(res) => res.json::<Transfer>().await.map_err(GatewayError::from),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn list_transfers(
        &self,
        user: Option<Pubkey>,
        offset: u64,
        limit: usize,
    ) -> GatewayResult<ListTransfersResponse> {
        let offset = offset.to_string();
        let limit = limit.to_string();
        let mut query = vec![("offset", offset.as_str()), ("limit", limit.as_str())];
        let user_str = user.map(|u| u.to_string());
        let user_ref = user_str.as_deref();
        if let Some(user_str) = user_ref {
            query.push(("user", user_str));
        };
        match reqwest::Client::new()
            .get(format!("{}/transfers", &self.api_url))
            .query(&query)
            .send()
            .await
        {
            Ok(res) => res
                .json::<ListTransfersResponse>()
                .await
                .map_err(GatewayError::from),
            Err(e) => Err(e.into()),
        }
    }
}

#[cached]
pub fn ore_token_account_address(pubkey: Pubkey) -> Pubkey {
    get_associated_token_address(&pubkey, &ore_api::consts::MINT_ADDRESS)
}

#[cached]
pub fn ore_token_account_address_v1(pubkey: Pubkey) -> Pubkey {
    get_associated_token_address(&pubkey, &ore_api::consts::MINT_V1_ADDRESS)
}
