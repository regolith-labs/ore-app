use crate::{components::submit_transaction, components::*, gateway::GatewayResult};
use dioxus::prelude::*;
use ore_types::request::TransactionType;
use solana_sdk::transaction::VersionedTransaction;

#[component]
pub fn Confirmation(
    show: Signal<bool>,
    transaction: Resource<GatewayResult<VersionedTransaction>>,
    transaction_type: TransactionType,
) -> Element {
    rsx! {
        {
            show.read().then(|| rsx! {
                div {
                    class: "p-4 fixed inset-0 bg-black bg-opacity-50 z-50 flex items-center justify-center",
                    onclick: move |_| show.set(false),
                    div {
                        class: "bg-surface-floating rounded-lg p-6 w-96 border border-gray-800 max-w-md",
                        onclick: move |e| e.stop_propagation(),
                        Col {
                            class: "p-4",
                            gap: 4,
                            span {
                                class: "text-xl font-semibold text-elements-highEmphasis text-center",
                                "Confirm Pair Deposit"
                            }
                            span {
                                class: "text-elements-midEmphasis text-center",
                                "Are you sure you want to deposit?"
                            }
                            Row {
                                class: "mt-4",
                                gap: 3,
                                button {
                                    class: "flex-1 h-12 rounded-full controls-secondary",
                                    onclick: move |_| show.set(false),
                                    span {
                                        class: "mx-auto my-auto",
                                        "Cancel"
                                    }
                                }
                                button {
                                    class: "flex-1 h-12 rounded-full controls-primary",
                                    onclick: move |_| {
                                        if let Some(Ok(tx)) = transaction.cloned() {
                                            submit_transaction(tx, transaction_type.clone());
                                            show.set(false);
                                        }
                                    },
                                    span {
                                        class: "mx-auto my-auto",
                                        "Yes, I'm sure"
                                    }
                                }
                            }
                        }
                    }
                }
            })
        }
    }
}
