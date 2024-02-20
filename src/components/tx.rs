use dioxus::prelude::*;
use dioxus_router::components::Link;

use crate::{gateway::AsyncResult, hooks::use_transfer, route::Route};

// Header
// Sig (Link to solana explorer)
// Type
// Amount
// From
// To
// Memo

#[component]
pub fn Tx(cx: Scope, sig: String) -> Element {
    let transfer = use_transfer(cx, sig.clone());

    match transfer {
        AsyncResult::Ok(transfer) => {
            render! {
                p {
                    "Transfer"
                }
                p {
                    "{transfer.sig}"
                }
                Link {
                    to: Route::User { id: transfer.from_address.clone() },
                    "{transfer.from_address}"
                }
                Link {
                    to: Route::User { id: transfer.to_address.clone() },
                    "{transfer.to_address}"
                }
                p {
                    "{transfer.amount}"
                }
                // p {
                //     "{transfer.memo}"
                // }
                // p {
                //     "{transfer.transfer_type}"
                // }
                p {
                    "{transfer.ts}"
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
