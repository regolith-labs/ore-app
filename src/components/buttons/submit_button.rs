use dioxus::prelude::*;
use solana_sdk::transaction::VersionedTransaction;
use ore_types::request::TransactionType;

use crate::{components::submit_transaction, gateway::GatewayResult};

use crate::components::{Alert, Col, Spinner, TokenInputError};

#[component]
pub fn SubmitButton(
    class: Option<String>,
    title: String,
    transaction: Resource<GatewayResult<VersionedTransaction>>,
    err: Signal<Option<TokenInputError>>,
    tx_type: TransactionType
) -> Element {
    let class = class.unwrap_or("controls-primary".to_string());

    let enabled = if let Some(Ok(_)) = transaction.read().as_ref() {
        if let Some(_) = err.cloned() {
            false
        } else {
            true
        }
    } else {
        false
    };

    rsx! {
        Col {
            class: "w-full",
            gap: 4,
            button {
                class: "flex h-12 w-full rounded-full {class} transition-transform hover:not-disabled:scale-105",
                disabled: !enabled,
                onclick: move |_| {
                    if let Some(Ok(transaction)) = transaction.cloned() {
                        submit_transaction(transaction, tx_type.clone());
                    }
                },
                if let UseResourceState::Pending = *transaction.state().read() {
                    Spinner {
                        class: "mx-auto my-auto",
                    }
                } else if let Some(err) = err.cloned() {
                    span {
                        class: "mx-auto my-auto font-semibold",
                        "{err.to_string()}"
                    }
                } else {
                    span {
                        class: "mx-auto my-auto font-semibold",
                        "{title}"
                    }
                }
            }
            Alert {}
        }
    }
}
