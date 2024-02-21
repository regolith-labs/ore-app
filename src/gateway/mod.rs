mod async_result;
mod error;
mod pubkey;
mod webworker;

pub use async_result::*;
pub use error::*;
pub use pubkey::*;
pub use webworker::*;

use cached::proc_macro::cached;
use gloo::net::websocket::futures::WebSocket;
use gloo_storage::{LocalStorage, Storage};
use ore::{
    state::{Proof, Treasury},
    utils::AccountDeserialize,
    TREASURY_ADDRESS,
};
use ore_types::{response::GetTransfersResponse, Transfer};
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
    WasmClient,
};

// TODO GatewayResult type

const API_URL: &str = "https://ore-api-lthm.onrender.com";
const RPC_URL: &str = "https://devnet.helius-rpc.com/?api-key=bb9df66a-8cba-404d-b17a-e739fe6a480c";
const WSS_URL: &str = "wss://ore-websockets.onrender.com/ws";

pub struct Gateway {
    // api: Client,
    pub rpc: WasmClient,
    api_url: String,
    wss: WebSocket,
}

impl Gateway {
    pub fn new() -> Self {
        Gateway {
            api_url: API_URL.to_string(),
            rpc: WasmClient::new(RPC_URL),
            wss: WebSocket::open(WSS_URL).unwrap(),
        }
    }

    pub async fn get_clock(&self) -> GatewayResult<Clock> {
        let data = self
            .rpc
            .get_account_data(&sysvar::clock::ID)
            .await
            .or(Err(GatewayError::NetworkUnavailable))?;
        bincode::deserialize::<Clock>(&data).or(Err(GatewayError::DeserializationFailure))
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

    pub async fn send_and_confirm(&self, ixs: &[Instruction]) -> Option<Signature> {
        let signer = signer();
        let mut transaction = Transaction::new_with_payer(ixs, Some(&signer.pubkey()));
        let recent_blockhash = self.rpc.get_latest_blockhash().await.unwrap();
        transaction.sign(&[&signer], recent_blockhash);
        let result = self.rpc.send_and_confirm_transaction(&transaction).await;
        match result {
            Ok(sig) => {
                log::info!("Transaction success: {:?}", sig);
                Some(sig)
            }
            Err(err) => {
                log::error!("Transaction failed: {:?}", err);
                None
            }
        }
    }

    // Ore
    pub async fn register_ore(&self) {
        // Return early, if account is already initialized
        let signer = signer();
        let proof_address = proof_pubkey(signer.pubkey());
        if self.rpc.get_account(&proof_address).await.is_ok() {
            return;
        }

        // Sign and send transaction.
        let ix = ore::instruction::register(signer.pubkey());
        self.send_and_confirm(&[ix]).await;
    }

    pub async fn claim_ore(&self, amount: u64) -> Option<Signature> {
        let signer = signer();
        let beneficiary = ore_token_account_address(signer.pubkey());
        let ix = ore::instruction::claim(signer.pubkey(), beneficiary, amount);
        self.send_and_confirm(&[ix]).await
    }

    pub async fn transfer_ore(&self, amount: u64, to: Pubkey, memo: String) -> Option<Signature> {
        let signer = signer();
        let from_token_account = ore_token_account_address(signer.pubkey());
        let to_token_account = ore_token_account_address(to);
        let memo_ix = solana_extra_wasm::program::spl_memo::build_memo(
            &memo.into_bytes(),
            &[&signer.pubkey()],
        );
        let transfer_ix = solana_extra_wasm::program::spl_token::instruction::transfer(
            &solana_extra_wasm::program::spl_token::ID,
            &from_token_account,
            &to_token_account,
            &signer.pubkey(),
            &[&signer.pubkey()],
            amount,
        )
        .unwrap();
        self.send_and_confirm(&[memo_ix, transfer_ix]).await
    }

    // TODO Result type
    pub async fn create_token_account_ore(&self) -> Pubkey {
        // Build instructions.
        let signer = signer();

        // Check if account already exists.
        let token_account_address = ore_token_account_address(signer.pubkey());
        match self.rpc.get_token_account(&token_account_address).await {
            Ok(token_account) => {
                if token_account.is_some() {
                    return token_account_address;
                }
            }
            Err(err) => {
                if let GatewayError::NotFound = GatewayError::from(err) {
                    // Do nothing
                }
            }
        }

        // Sign and send transaction.
        let ix =
        solana_extra_wasm::program::spl_associated_token_account::instruction::create_associated_token_account(
            &signer.pubkey(),
            &signer.pubkey(),
            &ore::MINT_ADDRESS,
            &solana_extra_wasm::program::spl_token::id(),
        );
        self.send_and_confirm(&[ix]).await;

        // Return token account address
        token_account_address
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

pub fn signer() -> Keypair {
    let key = "keypair";
    let value = LocalStorage::get(key).ok().unwrap_or_else(|| {
        let x = Keypair::new().to_base58_string();
        LocalStorage::set(key, &x).ok();
        x
    });
    Keypair::from_base58_string(&value)
}

#[cached]
pub fn ore_token_account_address(pubkey: Pubkey) -> Pubkey {
    solana_extra_wasm::program::spl_associated_token_account::get_associated_token_address(
        &pubkey,
        &ore::MINT_ADDRESS,
    )
}
