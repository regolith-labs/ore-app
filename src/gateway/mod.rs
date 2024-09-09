mod error;
mod pfee;
mod pubkey;

use async_std::future::{timeout, Future};
use cached::proc_macro::cached;
pub use error::*;
use ore_api::{
    consts::CONFIG_ADDRESS,
    state::{Config, Proof},
};
use ore_relayer_api::state::Escrow;
use ore_types::{response::ListTransfersResponse, Transfer};
use ore_utils::AccountDeserialize;
use solana_client_wasm::{
    solana_sdk::{clock::Clock, hash::Hash, pubkey::Pubkey, signature::Signature, sysvar},
    WasmClient,
};
use solana_extra_wasm::{
    account_decoder::parse_token::UiTokenAccount,
    program::spl_associated_token_account::get_associated_token_address,
    transaction_status::TransactionConfirmationStatus,
};
use web_time::Duration;

pub use pfee::*;
pub use pubkey::*;

pub const API_URL: &str = "https://ore-api-lthm.onrender.com";

// pub const RPC_URL: &str = "https://emelia-3g4m0w-fast-devnet.helius-rpc.com"; // Devnet

// pub const RPC_URL: &str = "https://amaleta-5y8tse-fast-mainnet.helius-rpc.com"; // Mainnet

pub const RPC_URL: &str = "https://rpc.ironforge.network/mainnet?apiKey=01J4NJDYJXSGJYE3AN6VXEB5VR";

// const GATEWAY_RETRIES: usize = 128;
const CONFIRM_RETRIES: usize = 20;
const CONFIRM_DELAY: u64 = 500;

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
        retry(|| self.try_get_clock()).await
    }

    pub async fn try_get_clock(&self) -> GatewayResult<Clock> {
        let data = self
            .rpc
            .get_account_data(&sysvar::clock::ID)
            .await
            .map_err(GatewayError::from)?;
        bincode::deserialize::<Clock>(&data).or(Err(GatewayError::FailedDeserialization))
    }

    pub async fn get_config(&self) -> GatewayResult<Config> {
        retry(|| self.try_get_config()).await
    }

    pub async fn try_get_config(&self) -> GatewayResult<Config> {
        let data = self
            .rpc
            .get_account_data(&CONFIG_ADDRESS)
            .await
            .map_err(GatewayError::from)?;
        Ok(*Config::try_from_bytes(&data).expect("Failed to parse config account"))
    }

    pub async fn get_proof(&self, authority: Pubkey) -> GatewayResult<Proof> {
        retry(|| self.try_get_proof(authority)).await
    }

    pub async fn get_proof_update(
        &self,
        authority: Pubkey,
        challenge: [u8; 32],
    ) -> GatewayResult<Proof> {
        loop {
            match retry(|| self.try_get_proof(authority)).await {
                Err(err) => return Err(err),
                Ok(proof) => {
                    if proof.challenge.ne(&challenge) {
                        return Ok(proof);
                    }
                }
            }
            async_std::task::sleep(Duration::from_millis(1000)).await;
        }
    }

    pub async fn try_get_proof(&self, authority: Pubkey) -> GatewayResult<Proof> {
        let data = self
            .rpc
            .get_account_data(&proof_pubkey(authority))
            .await
            .map_err(GatewayError::from)?;
        Ok(*Proof::try_from_bytes(&data).expect("Failed to parse proof"))
    }

    pub async fn get_escrow(&self, authority: Pubkey) -> GatewayResult<Escrow> {
        retry(|| self.try_get_escrow(authority)).await
    }

    pub async fn try_get_escrow(&self, authority: Pubkey) -> GatewayResult<Escrow> {
        let data = self
            .rpc
            .get_account_data(&escrow_pubkey(authority))
            .await
            .map_err(GatewayError::from)?;
        Ok(*Escrow::try_from_bytes(&data).expect("Failed to parse escrow"))
    }

    pub async fn get_token_account(
        &self,
        pubkey: &Pubkey,
    ) -> GatewayResult<Option<UiTokenAccount>> {
        retry(|| self.try_get_token_account(pubkey)).await
    }

    pub async fn try_get_token_account(
        &self,
        pubkey: &Pubkey,
    ) -> GatewayResult<Option<UiTokenAccount>> {
        self.rpc
            .get_token_account(pubkey)
            .await
            .map_err(GatewayError::from)
    }

    pub async fn get_proof_v1(&self, authority: Pubkey) -> GatewayResult<ore_api_v1::state::Proof> {
        retry(|| self.try_get_proof_v1(authority)).await
    }

    pub async fn try_get_proof_v1(
        &self,
        authority: Pubkey,
    ) -> GatewayResult<ore_api_v1::state::Proof> {
        let data = self
            .rpc
            .get_account_data(&proof_v1_pubkey(authority))
            .await
            .map_err(GatewayError::from)?;
        Ok(
            *<ore_api_v1::state::Proof as ore_api_v1::utils::AccountDeserialize>::try_from_bytes(
                &data,
            )
            .expect("Failed to parse proof"),
        )
    }

    pub async fn get_latest_blockhash(&self) -> GatewayResult<Hash> {
        retry(|| self.try_get_latest_blockhash()).await
    }

    pub async fn try_get_latest_blockhash(&self) -> GatewayResult<Hash> {
        self.rpc
            .get_latest_blockhash()
            .await
            .map_err(GatewayError::from)
    }

    pub async fn confirm_signature(&self, sig: Signature) -> GatewayResult<Signature> {
        // Confirm tx
        for _ in 0..CONFIRM_RETRIES {
            // Delay before confirming
            async_std::task::sleep(Duration::from_millis(CONFIRM_DELAY)).await;

            // Fetch transaction status
            match self.rpc.get_signature_statuses(&[sig]).await {
                Ok(signature_statuses) => {
                    for signature_status in signature_statuses {
                        if let Some(signature_status) = signature_status.as_ref() {
                            if signature_status.confirmation_status.is_some() {
                                if let Some(current_commitment) =
                                    signature_status.confirmation_status.as_ref()
                                {
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
        log::info!("Query: {:?}", query);
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

pub async fn retry<F, Fut, T>(f: F) -> GatewayResult<T>
where
    F: Fn() -> Fut,
    Fut: Future<Output = GatewayResult<T>>,
{
    const MAX_RETRIES: u32 = 8;
    const INITIAL_BACKOFF: Duration = Duration::from_millis(200);
    const TIMEOUT: Duration = Duration::from_secs(8);
    let mut backoff = INITIAL_BACKOFF;
    for attempt in 0..MAX_RETRIES {
        match timeout(TIMEOUT, f()).await {
            Ok(Ok(result)) => return Ok(result),
            Ok(Err(e)) if attempt < MAX_RETRIES - 1 => {
                match e {
                    GatewayError::AccountNotFound => return Err(e),
                    _ => {
                        async_std::task::sleep(backoff).await;
                        backoff *= 2; // Exponential backoff
                    }
                }
            }
            Ok(Err(e)) => return Err(e),
            Err(_) if attempt < MAX_RETRIES - 1 => {
                async_std::task::sleep(backoff).await;
                backoff *= 2; // Exponential backoff
            }
            Err(_) => return Err(GatewayError::RetryFailed),
        }
    }

    Err(GatewayError::AccountNotFound)
}

#[cached]
pub fn ore_token_account_address(pubkey: Pubkey) -> Pubkey {
    get_associated_token_address(&pubkey, &ore_api::consts::MINT_ADDRESS)
}

#[cached]
pub fn ore_token_account_address_v1(pubkey: Pubkey) -> Pubkey {
    get_associated_token_address(&pubkey, &ore_api::consts::MINT_V1_ADDRESS)
}
