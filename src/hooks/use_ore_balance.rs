use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;
use solana_extra_wasm::account_decoder::parse_token::UiTokenAmount;

use crate::gateway::{ore_token_account_address, wasm_client, AsyncResult, GatewayError};

pub fn use_ore_balance(
    cx: &ScopeState,
    pubkey: Pubkey,
) -> (AsyncResult<UiTokenAmount>, &UseFuture<()>) {
    // Balance state.
    let client = wasm_client();
    let token_account_address = ore_token_account_address(pubkey);
    let balance = use_state::<AsyncResult<UiTokenAmount>>(cx, || AsyncResult::Loading);

    // Future to fetch balance.
    let f = use_future(cx, (), |_| {
        let balance = balance.clone();
        async move {
            match client
                .get_token_account_balance(&token_account_address)
                .await
            {
                Ok(token_account_balance) => {
                    balance.set(AsyncResult::Ok(token_account_balance));
                }
                Err(err) => {
                    let err = GatewayError::from(err);
                    match err {
                        GatewayError::NotFound => balance.set(AsyncResult::Ok(UiTokenAmount {
                            ui_amount: Some(0f64),
                            decimals: ore::TOKEN_DECIMALS,
                            amount: "0.00".to_string(),
                            ui_amount_string: "0.00".to_string(),
                        })),
                        _ => {
                            balance.set(AsyncResult::Error(err));
                        }
                    }
                }
            }
        }
    });

    (balance.get().clone(), f)
}

pub trait UiTokenAmountBalance {
    fn balance(&self) -> u64;
}

impl UiTokenAmountBalance for UiTokenAmount {
    fn balance(&self) -> u64 {
        self.amount.parse().unwrap_or(0)
    }
}
