use ore_api::consts::MINT_ADDRESS;
use solana_extra_wasm::program::{spl_associated_token_account::get_associated_token_address, spl_token::state::Mint};
use solana_sdk::{program_pack::Pack, pubkey::Pubkey};

use super::{UiTokenAmount, GatewayError, GatewayResult, Rpc};

pub trait SplGateway {
    async fn get_token_balance(&self, owner: &Pubkey, mint: &Pubkey) -> GatewayResult<UiTokenAmount>;
    async fn get_ore_balance(&self, owner: &Pubkey) -> GatewayResult<UiTokenAmount>;
    async fn get_mint(&self, mint: &Pubkey) -> GatewayResult<Mint>;
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

    async fn get_ore_balance(&self, owner: &Pubkey) -> GatewayResult<UiTokenAmount> {
        self.get_token_balance(owner, &MINT_ADDRESS).await
    }

    async fn get_mint(&self, mint: &Pubkey) -> GatewayResult<Mint> {
        let mint = self.get_account_data(mint).await?;
        let mint = Mint::unpack(&mint.as_slice())?;
        Ok(mint)
    }
}
