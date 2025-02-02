use dioxus::prelude::*;
use solana_sdk::transaction::VersionedTransaction;

use crate::{components::submit_transaction, gateway::GatewayResult};

use super::TokenInputError;

#[component]
pub fn SubmitButton(
    title: String,
    transaction: Resource<GatewayResult<VersionedTransaction>>,
    error_msg: Signal<Option<TokenInputError>>
) -> Element {
    let enabled = if let Some(Ok(_)) = transaction.read().as_ref() {
        if let Some(_) = error_msg.cloned() {
            false
        } else {
            true
        }
    } else {
        false
    };

    rsx! {
        button {
            class: "h-12 w-full rounded-full controls-primary transition-transform hover:not-disabled:scale-105",
            disabled: !enabled,
            onclick: move |_| {
                if let Some(Ok(transaction)) = transaction.cloned() {
                    submit_transaction(transaction);
                }
            },
            if let Some(error) = error_msg.cloned() {
                span {
                    class: "mx-auto my-auto font-semibold",
                    "{error.to_string()}"
                }
            } else {
                span {
                    class: "mx-auto my-auto font-semibold",
                    "{title}"
                }
            }
        }
    }
}