use dioxus::prelude::*;
use ore_types::request::TransactionType;
use solana_sdk::transaction::VersionedTransaction;

use crate::{components::submit_transaction, gateway::GatewayResult};

#[component]
pub fn ClaimButton(transaction: Resource<GatewayResult<VersionedTransaction>>) -> Element {
    let enabled = if let Some(Ok(_)) = transaction.read().as_ref() {
        true
    } else {
        false
    };

    rsx! {
        button {
            class: "h-12 w-full rounded-full controls-gold",
            disabled: !enabled,
            onclick: move |_| {
                if let Some(Ok(transaction)) = transaction.cloned() {
                    submit_transaction(transaction, TransactionType::PoolClaim);
                }
            },
            span {
                class: "mx-auto my-auto font-semibold",
                "Claim"
            }
        }
    }
}
