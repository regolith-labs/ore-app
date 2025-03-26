use crate::{components::submit_transaction, components::*, gateway::GatewayResult};
use dioxus::prelude::*;
use ore_types::request::TransactionType;
use solana_sdk::transaction::VersionedTransaction;

#[derive(Clone, PartialEq, Eq)]
pub struct ConfirmationDialog {
    pub title: String,
    pub detail: String,
    pub ack: String,
}

#[component]
pub fn Confirmation(
    err: Signal<Option<TokenInputError>>,
    show_signal: Signal<bool>,
    transaction: Resource<GatewayResult<VersionedTransaction>>,
    dialog: ConfirmationDialog,
    transaction_type: TransactionType,
) -> Element {
    let mut is_confirmed = use_signal(|| false);

    let show = use_memo(move || *show_signal.read() && err.read().is_none());

    let lines = dialog.detail.split('\n').collect::<Vec<&str>>();

    rsx! {
        {
            show.read().then(|| rsx! {
                div {
                    class: "p-4 fixed inset-0 bg-black bg-opacity-50 z-50 flex items-center justify-center",
                    onclick: move |_| show_signal.set(false),
                    div {
                        class: "bg-surface-floating rounded-lg p-4 w-96 border border-gray-800 max-w-md",
                        onclick: move |e| e.stop_propagation(),
                        Col {
                            class: "p-4 justify-start",
                            gap: 4,
                            span {
                                class: "text-xl font-semibold text-elements-highEmphasis text-left",
                                "{dialog.title}"
                            }
                            for line in lines {
                                span {
                                    class: "text-elements-midEmphasis text-left",
                                    "{line}"
                                }
                            }
                            Row {
                                class: "justify-between w-full my-8",
                                gap: 8,
                                input {
                                    class: "checkbox mb-auto",
                                    r#type: "checkbox",
                                    checked: is_confirmed,
                                    onchange: move |e| is_confirmed.set(e.checked()),
                                }
                                span {
                                    class: "text-sm text-elements-midEmphasis text-right",
                                    "{dialog.ack}"
                                }
                            }
                            Col {
                                gap: 2,
                                button {
                                    class: "h-12 w-full rounded-full controls-secondary",
                                    onclick: move |_| show_signal.set(false),
                                    span {
                                        class: "mx-auto my-auto",
                                        "Cancel"
                                    }
                                }
                                button {
                                    class: "h-12 w-full rounded-full controls-primary",
                                    disabled: !*is_confirmed.read(),
                                    onclick: move |_| {
                                        if let Some(Ok(tx)) = transaction.cloned() {
                                            submit_transaction(tx, transaction_type.clone());
                                            show_signal.set(false);
                                        }
                                    },
                                    span {
                                        class: "mx-auto my-auto",
                                        "Continue"
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
