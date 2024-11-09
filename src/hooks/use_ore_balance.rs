use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;
use solana_extra_wasm::account_decoder::parse_token::UiTokenAmount;

use crate::gateway::{GatewayError, GatewayResult};

use super::{use_gateway, use_wallet_status, WalletStatus};

pub fn use_ore_balance() -> Resource<GatewayResult<UiTokenAmount>> {
    let wallet_status = use_wallet_status();
    use_resource(move || async move {
        match *wallet_status.read() {
            WalletStatus::Disconnected => Err(GatewayError::AccountNotFound.into()),
            WalletStatus::Connected(pubkey) => use_gateway()
                .get_ore_balance(&pubkey)
                .await
                .map_err(GatewayError::from),
        }
    })
}

pub fn use_token_balance(mint: Pubkey) -> Resource<GatewayResult<UiTokenAmount>> {
    let wallet_status = use_wallet_status();
    use_resource(move || async move {
        match *wallet_status.read() {
            WalletStatus::Disconnected => Err(GatewayError::AccountNotFound.into()),
            WalletStatus::Connected(pubkey) => use_gateway()
                .get_token_balance(&pubkey, &mint)
                .await
                .map_err(GatewayError::from),
        }
    })
}
