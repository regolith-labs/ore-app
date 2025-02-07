mod error;
pub mod kamino;
pub mod meteora;
pub mod ore;
pub mod pool;
pub mod solana;
pub mod spl;
mod utils;

pub use error::*;
use serde_json::{json, Value};
#[cfg(not(feature = "web"))]
use solana_client::nonblocking::rpc_client::RpcClient;
#[cfg(feature = "web")]
use solana_client_wasm::WasmClient;
use solana_sdk::{
    hash::Hash, pubkey::Pubkey, signature::Signature, transaction::VersionedTransaction,
};
pub use utils::*;

pub const RPC_URL: &str = "https://rpc.ironforge.network/mainnet?apiKey=01J4NJDYJXSGJYE3AN6VXEB5VR";
// pub const RPC_URL: &str = "https://rainy-alis-fast-mainnet.helius-rpc.com";

pub struct Gateway<R: Rpc> {
    pub rpc: R,
    pub http: reqwest::Client,
}

impl<R: Rpc> Gateway<R> {
    pub fn new(rpc_url: String) -> Gateway<R> {
        Gateway {
            rpc: R::new(rpc_url),
            http: reqwest::Client::new(),
        }
    }
    
    pub async fn get_recent_priority_fee_estimate(&self, treasury: bool) -> u64 {
        let mut ore_addresses: Vec<String> = vec![ore_api::id().to_string()];
        if treasury {
            ore_addresses.push(ore_api::consts::TREASURY_ADDRESS.to_string());
        }
        let req = json!({
            "jsonrpc": "2.0",
            "id": "priority-fee-estimate",
            "method": "getPriorityFeeEstimate",
            "params": [{
                "accountKeys": ore_addresses,
                "options": {
                    "recommended": true
                }
            }]
        });
        if let Ok(res) = self.http.post(RPC_URL.to_string()).json(&req).send().await {
            if let Ok(res) = res.json::<Value>().await {
                return res["result"]["priorityFeeEstimate"]
                    .as_f64()
                    .map(|fee| fee as u64)
                    .unwrap_or(0);
            }
        }
        0
    }
}

pub trait Rpc {
    fn new(rpc_url: String) -> Self;
    async fn get_account_data(&self, pubkey: &Pubkey) -> GatewayResult<Vec<u8>>;
    async fn get_balance(&self, pubkey: &Pubkey) -> GatewayResult<u64>;
    async fn get_latest_blockhash(&self) -> GatewayResult<Hash>;
    async fn get_signature_statuses(
        &self,
        signatures: &[Signature],
    ) -> GatewayResult<Vec<Option<TransactionConfirmationStatus>>>;
    async fn get_token_account(&self, pubkey: &Pubkey) -> GatewayResult<Option<UiTokenAmount>>;
    async fn get_token_supply(&self, mint: &Pubkey) -> GatewayResult<UiTokenAmount>;
    async fn send_transaction(
        &self,
        transaction: &VersionedTransaction,
    ) -> GatewayResult<Signature>;
}

#[cfg(not(feature = "web"))]
pub struct NativeRpc(RpcClient);

#[cfg(not(feature = "web"))]
impl Rpc for NativeRpc {
    fn new(rpc_url: String) -> Self {
        NativeRpc(RpcClient::new(rpc_url))
    }
    async fn get_account_data(&self, pubkey: &Pubkey) -> GatewayResult<Vec<u8>> {
        self.0.get_account_data(pubkey).await.map_err(From::from)
    }
    async fn get_balance(&self, pubkey: &Pubkey) -> GatewayResult<u64> {
        self.0.get_balance(pubkey).await.map_err(From::from)
    }
    async fn get_latest_blockhash(&self) -> GatewayResult<Hash> {
        self.0.get_latest_blockhash().await.map_err(From::from)
    }
    async fn get_signature_statuses(
        &self,
        signatures: &[Signature],
    ) -> GatewayResult<Vec<Option<TransactionConfirmationStatus>>> {
        let vec = self.0.get_signature_statuses(signatures).await?.value;
        let vec = vec
            .into_iter()
            .map(|opt| {
                if let Some(status) = opt {
                    if let Some(status) = status.confirmation_status {
                        match status {
                            solana_transaction_status::TransactionConfirmationStatus::Processed => {
                                Some(TransactionConfirmationStatus::Processed)
                            }
                            solana_transaction_status::TransactionConfirmationStatus::Confirmed => {
                                Some(TransactionConfirmationStatus::Confirmed)
                            }
                            solana_transaction_status::TransactionConfirmationStatus::Finalized => {
                                Some(TransactionConfirmationStatus::Finalized)
                            }
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();
        Ok(vec)
    }
    async fn get_token_account(&self, pubkey: &Pubkey) -> GatewayResult<Option<UiTokenAmount>> {
        let option = self.0.get_token_account(pubkey).await?;
        let option = option.map(|ta| UiTokenAmount {
            ui_amount: ta.token_amount.ui_amount,
            decimals: ta.token_amount.decimals,
            amount: ta.token_amount.amount,
            ui_amount_string: ta.token_amount.ui_amount_string,
        });
        Ok(option)
    }
    async fn get_token_supply(&self, mint: &Pubkey) -> GatewayResult<UiTokenAmount> {
        let ta = self.0.get_token_supply(mint).await?;
        let ta = UiTokenAmount {
            ui_amount: ta.ui_amount,
            decimals: ta.decimals,
            amount: ta.amount,
            ui_amount_string: ta.ui_amount_string,
        };
        Ok(ta)
    }
    async fn send_transaction(
        &self,
        transaction: &VersionedTransaction,
    ) -> GatewayResult<Signature> {
        self.0
            .send_transaction(transaction)
            .await
            .map_err(From::from)
    }
}

#[cfg(feature = "web")]
pub struct WebRpc(WasmClient);

#[cfg(feature = "web")]
impl Rpc for WebRpc {
    fn new(rpc_url: String) -> Self {
        WebRpc(WasmClient::new(rpc_url.as_str()))
    }
    async fn get_account_data(&self, pubkey: &Pubkey) -> GatewayResult<Vec<u8>> {
        self.0.get_account_data(pubkey).await.map_err(From::from)
    }
    async fn get_balance(&self, pubkey: &Pubkey) -> GatewayResult<u64> {
        self.0.get_balance(pubkey).await.map_err(From::from)
    }
    async fn get_latest_blockhash(&self) -> GatewayResult<Hash> {
        self.0.get_latest_blockhash().await.map_err(From::from)
    }
    async fn get_signature_statuses(
        &self,
        signatures: &[Signature],
    ) -> GatewayResult<Vec<Option<TransactionConfirmationStatus>>> {
        let vec = self.0.get_signature_statuses(signatures).await?;
        let vec = vec.into_iter().map(|opt| {
            if let Some(status) = opt {
                if let Some(status) = status.confirmation_status {
                    match status {
                        solana_extra_wasm::transaction_status::TransactionConfirmationStatus::Processed =>  Some(TransactionConfirmationStatus::Processed),
                        solana_extra_wasm::transaction_status::TransactionConfirmationStatus::Confirmed => Some(TransactionConfirmationStatus::Confirmed),
                        solana_extra_wasm::transaction_status::TransactionConfirmationStatus::Finalized => Some(TransactionConfirmationStatus::Finalized),
                    }
                } else {
                    None
                }
            } else {
                None
            }
        }).collect();
        Ok(vec)
    }
    async fn get_token_account(&self, pubkey: &Pubkey) -> GatewayResult<Option<UiTokenAmount>> {
        let option = self.0.get_token_account(pubkey).await?;
        let option = option.map(|ta| UiTokenAmount {
            ui_amount: ta.token_amount.ui_amount,
            decimals: ta.token_amount.decimals,
            amount: ta.token_amount.amount,
            ui_amount_string: ta.token_amount.ui_amount_string,
        });
        Ok(option)
    }
    async fn get_token_supply(&self, mint: &Pubkey) -> GatewayResult<UiTokenAmount> {
        let ta = self.0.get_token_supply(mint).await?;
        let ta = UiTokenAmount {
            ui_amount: ta.ui_amount,
            decimals: ta.decimals,
            amount: ta.amount,
            ui_amount_string: ta.ui_amount_string,
        };
        Ok(ta)
    }
    async fn send_transaction(
        &self,
        transaction: &VersionedTransaction,
    ) -> GatewayResult<Signature> {
        self.0
            .send_versioned_transaction(transaction)
            .await
            .map_err(From::from)
    }
}
