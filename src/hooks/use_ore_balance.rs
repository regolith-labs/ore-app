use std::str::FromStr;

use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::{native_token::lamports_to_sol, pubkey::Pubkey};
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

pub fn use_quote(mint: Pubkey) -> Resource<GatewayResult<u64>> {
    use_resource(move || async move {
        use_gateway()
            .get_quote(&mint)
            .await
            .map_err(GatewayError::from)
    })
}

pub fn use_token_balance(mint: Pubkey) -> Resource<GatewayResult<UiTokenAmount>> {
    let wallet_status = use_wallet_status();
    use_resource(move || async move {
        match *wallet_status.read() {
            WalletStatus::Disconnected => Err(GatewayError::AccountNotFound.into()),
            WalletStatus::Connected(pubkey) => {
                if mint == Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap()
                {
                    use_gateway()
                        .rpc
                        .get_balance(&pubkey)
                        .await
                        .map(|lamports| {
                            let sol = lamports_to_sol(lamports);
                            UiTokenAmount {
                                ui_amount: Some(sol),
                                decimals: 8,
                                amount: format!("{}", lamports).to_owned(),
                                ui_amount_string: format!("{}", sol).to_owned(),
                            }
                        })
                        .map_err(GatewayError::from)
                } else {
                    use_gateway()
                        .get_token_balance(&pubkey, &mint)
                        .await
                        .map_err(GatewayError::from)
                }
            }
        }
    })
}
