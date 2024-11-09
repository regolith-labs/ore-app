use ore_api::consts::MINT_ADDRESS;

use crate::steel_app::solana::{
    account_decoder::parse_token::{UiTokenAccount, UiTokenAmount},
    program::spl_associated_token_account::get_associated_token_address,
    sdk::pubkey::Pubkey,
};

use super::{retry, Gateway, GatewayError, GatewayResult};

impl Gateway {
    pub async fn get_token_account(
        &self,
        token_account_pubkey: &Pubkey,
    ) -> GatewayResult<Option<UiTokenAccount>> {
        retry(|| async {
            self.rpc
                .get_token_account(token_account_pubkey)
                .await
                .map_err(GatewayError::from)
        })
        .await
    }

    pub async fn get_token_balance(
        &self,
        owner: &Pubkey,
        mint: &Pubkey,
    ) -> GatewayResult<UiTokenAmount> {
        let ata_address = get_associated_token_address(owner, &mint);
        let Some(token_account) = self.get_token_account(&ata_address).await? else {
            return Err(GatewayError::AccountNotFound.into());
        };
        Ok(token_account.token_amount)
    }

    pub async fn get_ore_balance(&self, owner: &Pubkey) -> GatewayResult<UiTokenAmount> {
        self.get_token_balance(owner, &MINT_ADDRESS).await
    }
}
