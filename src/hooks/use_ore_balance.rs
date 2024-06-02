use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;
use solana_extra_wasm::account_decoder::parse_token::UiTokenAmount;

use crate::gateway::{ore_token_account_address, AsyncResult, GatewayError};

use super::{use_gateway, use_pubkey};

#[derive(Clone)]
pub struct BalanceHandle(UseFuture);

impl BalanceHandle {
    pub fn restart(&mut self) {
        self.0.restart();
    }
}

pub fn use_ore_balance_provider() {
    // Balance state.
    let gateway = use_gateway();
    let pubkey = use_pubkey();
    let token_account_address = ore_token_account_address(pubkey);
    use_context_provider::<Signal<AsyncResult<UiTokenAmount>>>(|| {
        Signal::new(AsyncResult::Loading)
    });
    let mut balance = use_context::<Signal<AsyncResult<UiTokenAmount>>>();

    // Future to fetch balance.
    let f = use_future(move || {
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
                            *balance.write() = AsyncResult::Ok(UiTokenAmount {
                                ui_amount: Some(0f64),
                                decimals: ore::TOKEN_DECIMALS,
                                amount: "0.00".to_string(),
                                ui_amount_string: "0.00".to_string(),
                            });
                        }
                        _ => {
                            balance.set(AsyncResult::Error(err));
                        }
                    }
                }
            }
        }
    });

    use_context_provider(|| BalanceHandle(f.clone()));
}

pub fn use_ore_balance_handle() -> BalanceHandle {
    use_context::<BalanceHandle>()
}

pub fn use_ore_balance(cx: &ScopeState) -> Signal<AsyncResult<UiTokenAmount>> {
    use_context::<Signal<AsyncResult<UiTokenAmount>>>()
}

pub fn use_ore_balance_user(pubkey: Pubkey) -> Signal<AsyncResult<UiTokenAmount>> {
    // Balance state.
    let gateway = use_gateway();
    let token_account_address = ore_token_account_address(pubkey);
    let mut balance = use_signal::<AsyncResult<UiTokenAmount>>(|| AsyncResult::Loading);

    // Future to fetch balance.
    use_future(move || {
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

    balance
}

pub trait UiTokenAmountBalance {
    fn balance(&self) -> u64;
}

impl UiTokenAmountBalance for UiTokenAmount {
    fn balance(&self) -> u64 {
        self.amount.parse().unwrap_or(0)
    }
}
