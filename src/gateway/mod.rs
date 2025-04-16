mod error;
pub mod kamino;
pub mod meteora;
pub mod ore;
pub mod pool;
pub mod solana;
pub mod spl;
mod utils;
pub mod wss;

pub use error::*;
use serde_json::{json, Value};
#[cfg(not(feature = "web"))]
use solana_client::nonblocking::rpc_client::RpcClient;

#[cfg(feature = "web")]
use solana_client_wasm::WasmClient;

use solana_sdk::{
    hash::Hash,
    pubkey::Pubkey,
    signature::Signature,
    transaction::{Transaction, VersionedTransaction},
};

pub use utils::SimulateTransactionResponse;
pub use utils::*;
pub use wss::*;

#[cfg(feature = "web")]
pub const RPC_URL: &str = "https://rpc.ironforge.network/mainnet?apiKey=01J4NJDYJXSGJYE3AN6VXEB5VR";
#[cfg(not(feature = "web"))]
pub const RPC_URL: &str = "https://rpc.ironforge.network/mainnet?apiKey=01JR0QT6CKAF608VC1DKSE1KC3";

#[cfg(feature = "web")]
pub const WSS_URL: &str = "wss://rpc.ironforge.network/mainnet?apiKey=01J4NJDYJXSGJYE3AN6VXEB5VR";
#[cfg(not(feature = "web"))]
pub const WSS_URL: &str = "wss://rpc.ironforge.network/mainnet?apiKey=01JR0QT6CKAF608VC1DKSE1KC3";

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

    pub async fn _get_recent_priority_fee_estimate(
        &self,
        tx: &VersionedTransaction,
    ) -> GatewayResult<u64> {
        match bincode::serialize(&tx) {
            Ok(tx_bytes) => {
                let tx_bs58 = bs58::encode(tx_bytes).into_string();
                let req = json!({
                    "jsonrpc": "2.0",
                    "id": "priority-fee-estimate",
                    "method": "getPriorityFeeEstimate",
                    "params": [{
                        "transaction": tx_bs58,
                        "options": {
                            "recommended": true
                        }
                    }]
                });
                if let Ok(res) = self.http.post(RPC_URL.to_string()).json(&req).send().await {
                    if let Ok(res) = res.json::<Value>().await {
                        // Get dynamic fee estimate in microlamports
                        let dynamic_fee_estimate = res["result"]["priorityFeeEstimate"]
                            .as_f64()
                            .map(|fee| fee as u64)
                            .unwrap_or(0);
                        return Ok(dynamic_fee_estimate);
                    } else {
                        log::error!("Failed to parse priority fee json");
                        return Err(GatewayError::Unknown);
                    }
                } else {
                    log::error!("Failed to send priority fee estimate request");
                    return Err(GatewayError::Unknown);
                }
            }
            Err(e) => {
                log::error!("err serializing tx: {}", e);
                Err(GatewayError::Unknown)
            }
        }
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

    #[cfg(not(feature = "web"))]
    async fn simulate_transaction(
        &self,
        transaction: &VersionedTransaction,
    ) -> GatewayResult<SimulateTransactionResponse>;

    #[cfg(feature = "web")]
    async fn simulate_transaction(
        &self,
        transaction: &Transaction,
    ) -> GatewayResult<SimulateTransactionResponse>;
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

    async fn simulate_transaction(
        &self,
        transaction: &VersionedTransaction,
    ) -> GatewayResult<SimulateTransactionResponse> {
        match self.0.simulate_transaction(transaction).await {
            Ok(response) => Ok(SimulateTransactionResponse {
                err: response.value.err,
                logs: response.value.logs,
                units_consumed: response.value.units_consumed,
            }),
            Err(err) => {
                log::error!("Simulation error: {:?}", err);
                Err(From::from(err))
            }
        }
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
    async fn simulate_transaction(
        &self,
        transaction: &Transaction,
    ) -> GatewayResult<SimulateTransactionResponse> {
        match self.0.simulate_transaction(transaction).await {
            Ok(response) => Ok(SimulateTransactionResponse {
                err: response.err,
                logs: response.logs,
                units_consumed: response.units_consumed,
            }),
            Err(err) => {
                log::error!("Simulation error: {:?}", err);
                Err(From::from(err))
            }
        }
    }
}
