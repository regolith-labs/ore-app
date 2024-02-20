use std::str::FromStr;

use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;

use crate::{
    gateway::AsyncResult,
    hooks::{use_ore_balance, use_user_transfers},
};

#[component]
pub fn User(cx: Scope, id: String) -> Element {
    let user_id = Pubkey::from_str(id);

    if user_id.is_err() {
        return render! {
            p {
                "Invalid user id"
            }
        };
    }

    let user_id = user_id.unwrap();
    let offset = use_state(cx, || 0);
    let (transfers, _has_more) = use_user_transfers(cx, user_id, offset);
    let (balance, _) = use_ore_balance(cx, user_id);

    render! {
        h2 {
            "User"
        }
        p {
            "{id}"
        }
        match balance {
            AsyncResult::Ok(balance) => {
                render! {
                    p {
                        "Balance {balance.real_number_string_trimmed()} ORE"
                    }
                }
            }
            _ => {
                render! {
                    p {
                        "Loading"
                    }
                }
            }
        }
        match transfers {
            AsyncResult::Ok(_transfers) => {
                render! {
                    p {
                        "Transfers [TODO]"
                    }
                }
            }
            AsyncResult::Loading => {
                render! {
                    p {
                        "Loading"
                    }
                }
            }
            _ => None,
        }
    }
}
