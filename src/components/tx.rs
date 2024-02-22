use dioxus::prelude::*;
use dioxus_router::components::Link;
use ore_types::TransferType;

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
            let transfer_memo = transfer.memo.unwrap_or("–".to_string());
            let transfer_type = match transfer.transfer_type {
                TransferType::Claim => "Claim",
                TransferType::Mine => "Mine",
                TransferType::Spl => "Spl",
            };
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
                p {
                    "{transfer_memo}"
                }
                p {
                    "{transfer_type}"
                }
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
