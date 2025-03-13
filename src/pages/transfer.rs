use dioxus::prelude::*;
use solana_sdk::transaction::VersionedTransaction;

use crate::{
    components::submit_transaction,
    // components::token_icon::TokenIcon,
    components::*,
    config::{Token, LISTED_TOKENS},
    gateway::GatewayResult,
    hooks::{use_token_balance, use_token_balance_for_token, use_transfer_transaction},
};

use std::str::FromStr;

use ore_types::request::TransactionType;
use steel::Pubkey;

use crate::hooks::on_transaction_done;

enum TransferStatus {
    Editing,
    Success,
}

#[component]
pub fn Transfer() -> Element {
    // Selected token
    let selected_token = use_signal(|| Some(Token::ore()));

    // Transfer amount
    let mut amount = use_signal::<String>(|| "".to_string());

    // Token balance
    let token_balance = use_token_balance_for_token(selected_token);

    // Error handling
    let mut err = use_signal::<Option<TokenInputError>>(|| None);

    // Priority fee
    let priority_fee = use_signal(|| 0);

    // Status
    let mut status = use_signal(|| TransferStatus::Editing);

    let display_picker = use_signal(|| false);

    // Confirmation dialog
    let show_confirmation = use_signal(|| false);

    let mut destination_pubkey: Signal<String> = use_signal::<String>(|| "".to_string());

    use_effect(move || {
        err.set(None);
    });

    // Use the transfer transaction hook
    let tx = use_transfer_transaction(
        destination_pubkey,
        selected_token,
        amount,
        token_balance,
        err,
        priority_fee,
    );

    on_transaction_done(move |_| {
        status.set(TransferStatus::Success);
        destination_pubkey.set("".to_string());
    });

    rsx! {
        Col {
            class: "w-full h-full max-w-2xl mx-auto px-5 sm:px-8 pb-20 sm:pb-16",
            gap: 8,
            Heading {
                class: "mx-auto w-full",
                title: "Transfer",
                subtitle: "Send tokens to another wallet."
            }

            match *status.read() {
                TransferStatus::Editing => {
                    rsx! {
                        Col {
                            class: "mx-auto w-full",
                            gap: 2,
                            span {
                                class: "text-elements-lowEmphasis font-medium pl-1",
                                "Token to transfer"
                            }
                            Row {
                                class: "lg:flex elevated elevated-border shrink-0 h-min rounded-lg z-0",
                                TokenInputForm {
                                    class: "p-4 border-b border-gray-800 w-full",
                                    title: "Select token".to_string(),
                                    token: selected_token,
                                    balance: token_balance,
                                    value: amount,
                                    update: amount,
                                    with_picker: true,
                                    err,
                                }
                            }
                        }
                        // Destination
                        if !display_picker.cloned() {
                            Col {
                                class: "mx-auto w-full",
                                gap: 2,
                                span {
                                    class: "text-elements-lowEmphasis font-medium pl-1",
                                    "Destination"
                                }
                                Row {
                                    class: "lg:flex elevated elevated-border shrink-0 h-min rounded-lg",
                                    div {
                                        class: "w-full",
                                        input {
                                            class: "p-4 border-b border-gray-800 h-12 my-auto w-full text-left outline-none text-elements-highEmphasis",
                                            placeholder: "Enter wallet address",
                                            value: destination_pubkey.clone(),
                                            oninput: move |e: FormEvent| destination_pubkey.set(e.value()),
                                        }
                                    }
                                }
                            }
                        }

                        // Fee display
                        Col {
                            class: "px-4",
                            gap: 2,
                            Fee { priority_fee: priority_fee.clone() }
                        }

                        // Transfer Button
                        TransferButton {
                            transaction: tx,
                            err,
                            amount,
                            selected_token,
                            destination: destination_pubkey,
                            show_confirmation,
                        }

                        // Confirmation Dialog
                        TransferConfirmation {
                            show: show_confirmation,
                            destination: destination_pubkey,
                            amount,
                            selected_token,
                            transaction: tx,
                        }
                    }
                }
                TransferStatus::Success => {
                    rsx! {
                        Col {
                            class: "mx-auto w-full",
                            gap: 8,
                            CheckCircleIcon {
                                class: "mx-auto w-24 h-24 text-elements-green mt-8"
                            }
                            Col {
                                gap: 2,
                                span {
                                    class: "text-elements-highEmphasis font-semibold text-2xl mx-auto",
                                    "Success!"
                                }
                                span {
                                    class: "text-elements-lowEmphasis font-medium mx-auto",
                                    "Your tokens have been transferred."
                                }
                            }
                            button {
                                class: "flex controls-primary w-full h-12 rounded-full hover:cursor-pointer mt-8",
                                onclick: move |_| {
                                    // Reset form
                                    status.set(TransferStatus::Editing);
                                    amount.set("".to_string());
                                    destination_pubkey.set("".to_string());
                                },
                                span {
                                    class: "mx-auto my-auto",
                                    "Make another transfer"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn TransferButton(
    class: Option<String>,
    transaction: Resource<GatewayResult<VersionedTransaction>>,
    err: Signal<Option<TokenInputError>>,
    amount: Signal<String>,
    selected_token: Signal<Option<Token>>,
    destination: Signal<String>,
    show_confirmation: Signal<bool>,
) -> Element {
    let class = class.unwrap_or("controls-primary".to_string());

    let is_tx_ready = use_memo(move || {
        if let Some(Ok(_tx)) = transaction.cloned() {
            true
        } else {
            false
        }
    });

    let is_disabled = !is_tx_ready.cloned() || err.read().is_some();
    let amount_str = amount.read().clone();
    let amount_f64 = amount_str.parse::<f64>().unwrap_or(0.0);
    let token_ticker = selected_token
        .read()
        .as_ref()
        .map(|t| t.ticker.clone())
        .unwrap_or_default();

    rsx! {
        Col {
            class: "w-full",
            gap: 4,
            button {
                class: "h-12 w-full rounded-full {class} transition-all duration-300 ease-in-out hover:not-disabled:scale-105",
                disabled: is_disabled,
                onclick: move |_| {
                    if is_tx_ready.cloned() {
                        show_confirmation.set(true);
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
                } else if amount_f64 > 0.0 {
                    span {
                        class: "mx-auto my-auto font-semibold",
                        "Transfer {amount_f64} {token_ticker}"
                    }
                } else {
                    span {
                        class: "mx-auto my-auto font-semibold",
                        "Transfer"
                    }
                }
            }
            Alert {}
        }
    }
}

#[component]
fn TransferConfirmation(
    show: Signal<bool>,
    destination: Signal<String>,
    amount: Signal<String>,
    selected_token: Signal<Option<Token>>,
    transaction: Resource<GatewayResult<VersionedTransaction>>,
) -> Element {
    let destination_str = destination.read().clone();
    let abbreviated_address = if destination_str.len() > 8 {
        format!(
            "{}...{}",
            &destination_str[0..4],
            &destination_str[destination_str.len() - 4..]
        )
    } else {
        destination_str.clone()
    };

    let amount_str = amount.read().clone();
    let amount_f64 = amount_str.parse::<f64>().unwrap_or(0.0);
    let token_ticker = selected_token
        .read()
        .as_ref()
        .map(|t| t.ticker.clone())
        .unwrap_or_default();

    rsx! {
        {
            show.read().then(|| rsx! {
                div {
                    class: "fixed inset-0 bg-black bg-opacity-50 z-50 flex items-center justify-center",
                    onclick: move |_| show.set(false),
                    div {
                        class: "bg-surface-floating rounded-lg p-6 w-96 border border-gray-800 max-w-md",
                        onclick: move |e| e.stop_propagation(),
                        div {
                            class: "flex flex-col gap-4",
                            div {
                                class: "text-xl font-semibold text-elements-highEmphasis text-center",
                                "Confirm Transfer"
                            }
                            div {
                                class: "text-elements-midEmphasis text-center",
                                "Are you sure you want to transfer {amount_f64} {token_ticker} to this address?"
                            }
                            div {
                                class: "text-elements-highEmphasis font-mono text-center bg-surface-elevated p-2 rounded",
                                "{abbreviated_address}"
                            }
                            div {
                                class: "flex gap-3 mt-4",
                                button {
                                    class: "flex-1 h-12 rounded-full controls-secondary",
                                    onclick: move |_| show.set(false),
                                    "Cancel"
                                }
                                button {
                                    class: "flex-1 h-12 rounded-full controls-primary",
                                    onclick: move |_| {
                                        if let Some(Ok(tx)) = transaction.cloned() {
                                            submit_transaction(tx, TransactionType::Swap);
                                            show.set(false);
                                        }
                                    },
                                    "Yes, I'm sure"
                                }
                            }
                        }
                    }
                }
            })
        }
    }
}
