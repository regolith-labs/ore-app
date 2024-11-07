mod confirm_signature;
mod error;
mod ore_program;
mod priority_fee;
mod retry;

use crate::steel_app::solana::{
    account_decoder::parse_token::UiTokenAccount,
    sdk::{clock::Clock, hash::Hash, pubkey::Pubkey, sysvar},
};
pub use error::*;

use retry::retry;
use solana_client_wasm::WasmClient;

pub const RPC_URL: &str = "https://rpc.ironforge.network/mainnet?apiKey=01J4NJDYJXSGJYE3AN6VXEB5VR";

pub struct Gateway {
    pub rpc: WasmClient,
}

impl Gateway {
    pub fn new(rpc_url: String) -> Self {
        Gateway {
            rpc: WasmClient::new(&rpc_url),
        }
    }

    pub async fn get_clock(&self) -> GatewayResult<Clock> {
        retry(|| async {
            let data = self
                .rpc
                .get_account_data(&sysvar::clock::ID)
                .await
                .map_err(GatewayError::from)?;
            bincode::deserialize::<Clock>(&data).or(Err(GatewayError::FailedDeserialization))
        })
        .await
    }

    pub async fn get_latest_blockhash(&self) -> GatewayResult<Hash> {
        retry(|| async {
            self.rpc
                .get_latest_blockhash()
                .await
                .map_err(GatewayError::from)
        })
        .await
    }

    pub async fn get_token_account(
        &self,
        pubkey: &Pubkey,
    ) -> GatewayResult<Option<UiTokenAccount>> {
        retry(|| async {
            self.rpc
                .get_token_account(pubkey)
                .await
                .map_err(GatewayError::from)
        })
        .await
    }
}
