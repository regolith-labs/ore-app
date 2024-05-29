use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;
use solana_extra_wasm::account_decoder::parse_token::UiTokenAmount;

use crate::gateway::{ore_token_account_address, AsyncResult, GatewayError};

use super::{use_gateway, use_pubkey};

#[derive(Clone)]
pub struct BalanceHandle(UseFuture<()>);

impl BalanceHandle {
    pub fn restart(&self) {
        self.0.restart();
    }
}

pub fn use_ore_balance_provider(cx: &ScopeState) {
    // Balance state.
    let gateway = use_gateway(cx);
    let pubkey = use_pubkey(cx);
    let token_account_address = ore_token_account_address(pubkey);
    use_shared_state_provider::<AsyncResult<UiTokenAmount>>(cx, || AsyncResult::Loading);
    let balance = use_shared_state::<AsyncResult<UiTokenAmount>>(cx).unwrap();

    // Future to fetch balance.
    let f = use_future(cx, (), |_| {
        let balance = balance.clone();
        let gateway = gateway.clone();
        async move {
            match gateway
                .rpc
                .get_token_account_balance(&token_account_address)
                .await
            {
                Ok(token_account_balance) => {
                    *balance.write() = AsyncResult::Ok(token_account_balance);
                }
                Err(err) => {
                    let err = GatewayError::from(err);
                    match err {
                        GatewayError::AccountNotFound => {
                            *balance.write() = AsyncResult::Ok(UiTokenAmount {
                                ui_amount: Some(0f64),
                                decimals: ore::TOKEN_DECIMALS,
                                amount: "0.00".to_string(),
                                ui_amount_string: "0.00".to_string(),
                            });
                        }
                        _ => {
                            *balance.write() = AsyncResult::Error(err);
                        }
                    }
                }
            }
        }
    });

    cx.provide_context(BalanceHandle(f.clone()));
}

pub fn use_ore_balance_handle(cx: &ScopeState) -> BalanceHandle {
    cx.consume_context::<BalanceHandle>().unwrap()
}

pub fn use_ore_balance(cx: &ScopeState) -> AsyncResult<UiTokenAmount> {
    use_shared_state::<AsyncResult<UiTokenAmount>>(cx)
        .unwrap()
        .read()
        .clone()
}

pub fn use_ore_balance_user(cx: &ScopeState, pubkey: Pubkey) -> AsyncResult<UiTokenAmount> {
    // Balance state.
    let gateway = use_gateway(cx);
    let token_account_address = ore_token_account_address(pubkey);
    let balance = use_state::<AsyncResult<UiTokenAmount>>(cx, || AsyncResult::Loading);

    // Future to fetch balance.
    let _ = use_future(cx, (), |_| {
        let balance = balance.clone();
        let gateway = gateway.clone();
        async move {
            match gateway
                .rpc
                .get_token_account_balance(&token_account_address)
                .await
            {
                Ok(token_account_balance) => {
                    balance.set(AsyncResult::Ok(token_account_balance));
                }
                Err(err) => {
                    let err = GatewayError::from(err);
                    match err {
                        GatewayError::AccountNotFound => {
                            balance.set(AsyncResult::Ok(UiTokenAmount {
                                ui_amount: Some(0f64),
                                decimals: ore::TOKEN_DECIMALS,
                                amount: "0.00".to_string(),
                                ui_amount_string: "0.00".to_string(),
                            }))
                        }
                        _ => {
                            balance.set(AsyncResult::Error(err));
                        }
                    }
                }
            }
        }
    });

    balance.get().clone()
}

pub trait UiTokenAmountBalance {
    fn balance(&self) -> u64;
}

impl UiTokenAmountBalance for UiTokenAmount {
    fn balance(&self) -> u64 {
        self.amount.parse().unwrap_or(0)
    }
}
