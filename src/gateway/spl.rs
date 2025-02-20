use ore_api::consts::MINT_ADDRESS;
use solana_sdk::{program_pack::Pack, pubkey::Pubkey};

#[cfg(not(feature = "web"))]
use super::NativeRpc;
#[cfg(feature = "web")]
use super::WebRpc;
use super::{GatewayError, GatewayResult, Rpc, UiTokenAmount};
use crate::solana::{
    spl_associated_token_account::get_associated_token_address, spl_token::state::Mint,
};

pub trait SplGateway {
    async fn get_token_balance(
        &self,
        owner: &Pubkey,
        mint: &Pubkey,
    ) -> GatewayResult<UiTokenAmount>;
    async fn get_mint(&self, mint: &Pubkey) -> GatewayResult<Mint>;
    async fn get_ore_balance(&self, owner: &Pubkey) -> GatewayResult<UiTokenAmount> {
        self.get_token_balance(owner, &MINT_ADDRESS).await
    }
}

#[cfg(feature = "web")]
impl SplGateway for WebRpc {
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

    async fn get_mint(&self, mint: &Pubkey) -> GatewayResult<Mint> {
        let mint = self.get_account_data(mint).await?;
        let mint = Mint::unpack(&mint.as_slice())?;
        Ok(mint)
    }
}

#[cfg(not(feature = "web"))]
impl SplGateway for NativeRpc {
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

    async fn get_mint(&self, mint: &Pubkey) -> GatewayResult<Mint> {
        let mint = self.get_account_data(mint).await?;
        let mint = Mint::unpack(&mint.as_slice())?;
        Ok(mint)
    }
}
