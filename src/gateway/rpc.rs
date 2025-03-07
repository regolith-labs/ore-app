use solana_sdk::{
    hash::Hash, pubkey::Pubkey, signature::Signature, transaction::VersionedTransaction,
};

use super::{GatewayResult, TransactionConfirmationStatus, UiTokenAmount};

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
