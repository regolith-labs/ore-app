mod async_result;
mod error;
mod pubkey;

pub use async_result::*;
use cached::proc_macro::cached;
pub use error::*;
#[cfg(feature = "web")]
use gloo_storage::{LocalStorage, Storage};
use ore::{
    state::{Proof, Treasury},
    utils::AccountDeserialize,
    TREASURY_ADDRESS,
};
use ore_types::{response::GetTransfersResponse, Transfer};
pub use pubkey::*;
#[cfg(feature = "desktop")]
use solana_account_decoder::parse_token::UiTokenAccount;
#[cfg(feature = "desktop")]
use solana_client::{nonblocking::rpc_client::RpcClient, rpc_response::RpcTokenAccountBalance};
#[cfg(feature = "web")]
use solana_client_wasm::{
    solana_sdk::{
        clock::Clock,
        instruction::Instruction,
        pubkey::Pubkey,
        signature::{Keypair, Signature},
        signer::Signer,
        sysvar,
        transaction::Transaction,
    },
    utils::rpc_response::RpcTokenAccountBalance,
    WasmClient,
};
#[cfg(feature = "web")]
use solana_extra_wasm::{
    account_decoder::parse_token::UiTokenAccount,
    program::{
        spl_associated_token_account::{
            get_associated_token_address, instruction::create_associated_token_account,
        },
        spl_memo, spl_token,
    },
};
#[cfg(feature = "desktop")]
use solana_sdk::{
    clock::Clock,
    instruction::Instruction,
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signer::Signer,
    sysvar,
    transaction::Transaction,
};
#[cfg(feature = "desktop")]
use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account,
};

use crate::metrics::{track, AppEvent};

pub const API_URL: &str = "https://ore-api-lthm.onrender.com";
pub const RPC_URL: &str =
    "https://devnet.helius-rpc.com/?api-key=bb9df66a-8cba-404d-b17a-e739fe6a480c";
// "https://emelia-3g4m0w-fast-devnet.helius-rpc.com/";

#[cfg(feature = "desktop")]
pub const WSS_URL: &str = "wss://ore-websockets.onrender.com/ws";

pub struct Gateway {
    #[cfg(feature = "web")]
    pub rpc: WasmClient,
    #[cfg(feature = "desktop")]
    pub rpc: RpcClient,
    api_url: String,
}

impl Gateway {
    pub fn new() -> Self {
        Gateway {
            api_url: API_URL.to_string(),
            #[cfg(feature = "web")]
            rpc: WasmClient::new(RPC_URL),
            #[cfg(feature = "desktop")]
            rpc: RpcClient::new(RPC_URL.to_string()),
        }
    }

    pub async fn get_clock(&self) -> GatewayResult<Clock> {
        let data = self
            .rpc
            .get_account_data(&sysvar::clock::ID)
            .await
            .or(Err(GatewayError::NetworkUnavailable))?;
        bincode::deserialize::<Clock>(&data).or(Err(GatewayError::FailedDeserialization))
    }

    pub async fn get_proof(&self, authority: Pubkey) -> GatewayResult<Proof> {
        let data = self
            .rpc
            .get_account_data(&proof_pubkey(authority))
            .await
            .or(Err(GatewayError::NetworkUnavailable))?;
        Ok(*Proof::try_from_bytes(&data).expect("Failed to parse proof"))
    }

    pub async fn get_treasury(&self) -> GatewayResult<Treasury> {
        let data = self
            .rpc
            .get_account_data(&TREASURY_ADDRESS)
            .await
            .or(Err(GatewayError::NetworkUnavailable))?;
        Ok(*Treasury::try_from_bytes(&data).expect("Failed to parse treasury account"))
    }

    pub async fn get_token_account(
        &self,
        pubkey: &Pubkey,
    ) -> GatewayResult<Option<UiTokenAccount>> {
        self.rpc
            .get_token_account(pubkey)
            .await
            .or(Err(GatewayError::NetworkUnavailable))
    }

    pub async fn get_token_largest_accounts(
        &self,
        pubkey: &Pubkey,
    ) -> GatewayResult<Vec<RpcTokenAccountBalance>> {
        self.rpc
            .get_token_largest_accounts(pubkey)
            .await
            .or(Err(GatewayError::NetworkUnavailable))
    }

    pub async fn send_and_confirm(&self, ixs: &[Instruction]) -> GatewayResult<Signature> {
        let signer = signer();
        let mut tx = Transaction::new_with_payer(ixs, Some(&signer.pubkey()));
        let hash = self.rpc.get_latest_blockhash().await.unwrap();
        log::info!("HASH: {:?}", hash);
        tx.sign(&[&signer], hash);
        log::info!("Signed...");
        log::info!("TX: {:?}", tx);
        let x = self.rpc.send_and_confirm_transaction(&tx).await;
        log::info!("X: {:?}", x);
        x.or(Err(GatewayError::FailedTransaction))
    }

    // Ore
    pub async fn register_ore(&self) -> GatewayResult<()> {
        // Return early, if account is already initialized
        let signer = signer();
        let proof_address = proof_pubkey(signer.pubkey());
        if self.rpc.get_account(&proof_address).await.is_ok() {
            return Ok(());
        }

        // Sign and send transaction.
        let ix = ore::instruction::register(signer.pubkey());
        match self.send_and_confirm(&[ix]).await {
            Ok(_) => {
                track(AppEvent::Register, None);
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    pub async fn claim_ore(&self, amount: u64) -> GatewayResult<Signature> {
        let signer = signer();
        let beneficiary = ore_token_account_address(signer.pubkey());
        let ix = ore::instruction::claim(signer.pubkey(), beneficiary, amount);
        self.send_and_confirm(&[ix]).await
    }

    pub async fn transfer_ore(
        &self,
        amount: u64,
        to: Pubkey,
        memo: String,
    ) -> GatewayResult<Signature> {
        let signer = signer();
        let from_token_account = ore_token_account_address(signer.pubkey());
        let to_token_account = ore_token_account_address(to);
        let memo_ix = spl_memo::build_memo(&memo.into_bytes(), &[&signer.pubkey()]);
        let transfer_ix = spl_token::instruction::transfer(
            &spl_token::ID,
            &from_token_account,
            &to_token_account,
            &signer.pubkey(),
            &[&signer.pubkey()],
            amount,
        )
        .unwrap();
        // TODO Create recipient token account if necessary
        self.send_and_confirm(&[memo_ix, transfer_ix]).await
    }

    pub async fn create_token_account_ore(&self) -> GatewayResult<Pubkey> {
        // Build instructions.
        let signer = signer();

        // Check if account already exists.
        let token_account_address = ore_token_account_address(signer.pubkey());
        match self.rpc.get_token_account(&token_account_address).await {
            Ok(token_account) => {
                if token_account.is_some() {
                    return Ok(token_account_address);
                }
            }
            Err(err) => {
                if let GatewayError::AccountNotFound = GatewayError::from(err) {
                    // Noop
                }
            }
        }

        // Sign and send transaction.
        let ix = create_associated_token_account(
            &signer.pubkey(),
            &signer.pubkey(),
            &ore::MINT_ADDRESS,
            &spl_token::id(),
        );
        match self.send_and_confirm(&[ix]).await {
            Ok(_) => track(AppEvent::CreateTokenAccount, None),
            Err(err) => return Err(err),
        }

        // Return token account address
        Ok(token_account_address)
    }

    // API
    pub async fn get_transfer(&self, sig: String) -> Option<Transfer> {
        let client = reqwest::Client::new();
        match client
            .get(format!("{}/transfers/{}", self.api_url, sig))
            .send()
            .await
        {
            Ok(res) => res.json::<Transfer>().await.ok(),
            Err(e) => {
                log::error!("{:?}", e);
                None
            }
        }
    }

    pub async fn list_transfers(
        &self,
        user: Option<Pubkey>,
        offset: u64,
        limit: usize,
    ) -> Option<GetTransfersResponse> {
        let offset = offset.to_string();
        let limit = limit.to_string();
        let mut query = vec![("offset", offset.as_str()), ("limit", limit.as_str())];
        let user_str = user.map(|u| u.to_string());
        let user_ref = user_str.as_deref();
        if let Some(user_str) = user_ref {
            query.push(("user", user_str));
        };
        let client = reqwest::Client::new();
        match client
            .get(format!("{}/transfers", &self.api_url))
            .query(&query)
            .send()
            .await
        {
            Ok(res) => res.json::<GetTransfersResponse>().await.ok(),
            Err(e) => {
                log::error!("{:?}", e);
                None
            }
        }
    }
}

#[cfg(feature = "web")]
pub fn signer() -> Keypair {
    let key = "keypair";
    let value = LocalStorage::get(key).ok().unwrap_or_else(|| {
        let x = Keypair::new().to_base58_string();
        LocalStorage::set(key, &x).ok();
        x
    });
    Keypair::from_base58_string(&value)
}

#[cfg(feature = "desktop")]
pub fn signer() -> Keypair {
    use crate::file::{get_value, set_key_value};

    let key = "keypair";
    let value = get_value(key).ok().unwrap_or_else(|| {
        let value = Keypair::new().to_base58_string();
        if let Ok(v) = serde_json::to_value(&value) {
            set_key_value(key, &v).ok();
        }
        value
    });
    Keypair::from_base58_string(&value)
}

#[cached]
pub fn ore_token_account_address(pubkey: Pubkey) -> Pubkey {
    get_associated_token_address(&pubkey, &ore::MINT_ADDRESS)
}
