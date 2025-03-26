use dioxus::prelude::*;
use ore_types::request::TransactionType;
use solana_sdk::transaction::VersionedTransaction;

use crate::{components::submit_transaction, gateway::GatewayResult};

use crate::components::{Alert, Col, Confirmation, ConfirmationDialog, Spinner, TokenInputError};

#[component]
pub fn SubmitButton(
    class: Option<String>,
    title: String,
    transaction: Resource<GatewayResult<VersionedTransaction>>,
    err: Signal<Option<TokenInputError>>,
    tx_type: TransactionType,
    confirmation: Option<ConfirmationDialog>,
) -> Element {
    let class = class.unwrap_or("controls-primary".to_string());

    let mut show_confirmation = use_signal(|| false);

    let enabled = if let Some(Ok(_)) = transaction.read().as_ref() {
        if let Some(_) = err.cloned() {
            false
        } else {
            true
        }
    } else {
        false
    };

    let confirmation_is_some = confirmation.is_some();

    rsx! {
        Col {
            class: "w-full",
            gap: 4,
            button {
                class: "flex h-12 w-full rounded-full {class} transition-transform hover:not-disabled:scale-105",
                disabled: !enabled,
                onclick: move |_| {
                    if confirmation_is_some {
                        show_confirmation.set(true);
                    } else if let Some(Ok(transaction)) = transaction.cloned() {
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
            if let Some(confirmation) = confirmation {
                Confirmation {
                    show_signal: show_confirmation,
                    err: err,
                    transaction: transaction,
                    transaction_type: TransactionType::BoostDeposit,
                    dialog: confirmation,
                }
            }
        }
    }
}
