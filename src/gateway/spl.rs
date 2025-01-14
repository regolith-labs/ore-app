use ore_api::consts::MINT_ADDRESS;
use solana_sdk::pubkey::Pubkey;

use crate::steel_app::solana::program::get_associated_token_address;

use super::{retry, ui_token_amount::UiTokenAmount, GatewayError, GatewayResult, Rpc};

pub trait SplGateway {
    async fn get_token_balance(
        &self,
        owner: &Pubkey,
        mint: &Pubkey,
    ) -> GatewayResult<UiTokenAmount>;
    async fn get_ore_balance(&self, owner: &Pubkey) -> GatewayResult<UiTokenAmount> {
        retry(|| async { self.get_token_balance(owner, &MINT_ADDRESS).await }).await
    }
}

impl<R: Rpc> SplGateway for R {
    async fn get_token_balance(
        &self,
        owner: &Pubkey,
        mint: &Pubkey,
    ) -> GatewayResult<UiTokenAmount> {
        let ata_address = get_associated_token_address(owner, &mint);
        let Some(token_account) = self.get_token_account(&ata_address).await? else {
            return Err(GatewayError::AccountNotFound.into());
        };
        Ok(token_account)
    }
}
