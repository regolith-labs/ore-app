use solana_client_wasm::WasmClient;
use solana_sdk::{
    hash::Hash, pubkey::Pubkey, signature::Signature, transaction::VersionedTransaction,
};

use super::{GatewayResult, Rpc, TransactionConfirmationStatus, UiTokenAmount};

pub struct WebRpc(WasmClient);

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
