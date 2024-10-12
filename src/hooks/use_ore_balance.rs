use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;
use solana_extra_wasm::{
    account_decoder::parse_token::UiTokenAmount,
    program::spl_associated_token_account::get_associated_token_address,
};

use crate::gateway::{ore_token_account_address, GatewayError, GatewayResult};

use super::{
    use_gateway,
    use_wallet_adapter::{use_wallet_adapter, WalletAdapter},
};

pub fn use_balance(mint: Pubkey, decimals: u8) -> Resource<GatewayResult<UiTokenAmount>> {
    let gateway = use_gateway();
    let wallet_adapter = use_wallet_adapter();
    use_resource(move || {
        let gateway = gateway.clone();
        async move {
            match *wallet_adapter.read() {
                WalletAdapter::Connected(pubkey) => {
                    let token_account_address = get_associated_token_address(&pubkey, &mint);
                    gateway
                        .rpc
                        .get_token_account_balance(&token_account_address)
                        .await
                        .map_err(GatewayError::from)
                }
                WalletAdapter::Disconnected => Ok(UiTokenAmount::default(decimals)),
            }
        }
    })
}

pub fn use_ore_balance() -> Resource<GatewayResult<UiTokenAmount>> {
    use_balance(
        ore_api::consts::MINT_ADDRESS,
        ore_api::consts::TOKEN_DECIMALS,
    )
}

pub fn use_ore_v1_balance() -> Resource<GatewayResult<UiTokenAmount>> {
    use_balance(
        ore_api::consts::MINT_V1_ADDRESS,
        ore_api::consts::TOKEN_DECIMALS_V1,
    )
}

pub fn use_ore_balance_user(pubkey: Pubkey) -> Resource<GatewayResult<UiTokenAmount>> {
    let gateway = use_gateway();
    let token_account_address = ore_token_account_address(pubkey);
    use_resource(move || {
        let gateway = gateway.clone();
        async move {
            gateway
                .rpc
                .get_token_account_balance(&token_account_address)
                .await
                .map_err(GatewayError::from)
        }
    })
}

pub trait UiTokenAmountBalance {
    fn balance(&self) -> u64;
}

impl UiTokenAmountBalance for UiTokenAmount {
    fn balance(&self) -> u64 {
        self.amount.parse().unwrap_or(0)
    }
}

pub trait UiTokenAmountDefault {
    fn default(decimals: u8) -> Self;
}

impl UiTokenAmountDefault for UiTokenAmount {
    fn default(decimals: u8) -> Self {
        UiTokenAmount {
            ui_amount: None,
            decimals,
            amount: "0".to_string(),
            ui_amount_string: "0".to_string(),
        }
    }
}
