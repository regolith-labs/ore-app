use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;
use solana_extra_wasm::account_decoder::parse_token::UiTokenAmount;

use crate::gateway::{ore_token_account_address, GatewayError, GatewayResult};

use super::{use_gateway, use_pubkey};

pub fn use_ore_balance() -> Resource<GatewayResult<UiTokenAmount>> {
    let gateway = use_gateway();
    let pubkey = use_pubkey();
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
