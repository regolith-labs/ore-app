use dioxus::prelude::*;
use solana_sdk::transaction::VersionedTransaction;

use crate::{
    components::submit_transaction,
    components::CheckCircleIcon,
    components::*,
    config::Token,
    gateway::GatewayResult,
    hooks::{use_token_balance, use_transfer_transaction},
};

use ore_types::request::TransactionType;

use crate::hooks::on_transaction_done;

use solana_sdk::pubkey::Pubkey;

use dioxus::router::prelude::use_navigator;

#[derive(Clone, PartialEq, Eq)]
pub enum TransferError {
    InvalidAddress,
}

impl ToString for TransferError {
    fn to_string(&self) -> String {
        match self {
            TransferError::InvalidAddress => "Invalid Solana address".to_string(),
        }
    }
}

enum TransferStatus {
    Editing,
    Success,
}

#[component]
pub fn Transfer(token_ticker: Option<String>) -> Element {
    let navigator = use_navigator();

    // Selected token - now initialized based on the token_ticker parameter
    let mut selected_token = use_signal(|| Some(Token::ore()));

    // Clone token_ticker for different uses
    let token_ticker_for_init = token_ticker.clone();
    let token_ticker_for_sync = token_ticker;

    // Initialize with the provided token ticker if available
    use_effect(move || {
        if let Some(ticker) = &token_ticker_for_init {
            if let Some(token) = crate::config::LISTED_TOKENS_BY_TICKER.get(ticker) {
                selected_token.set(Some(token.clone()));
            }
        }
    });

    // Add this effect to sync URL when token changes manually
    use_effect(move || {
        let current_token = selected_token.read().clone();
        if let Some(token) = current_token {
            // Only update URL if the token ticker doesn't match the URL parameter
            if token_ticker_for_sync.as_ref() != Some(&token.ticker) {
                // Update the URL without a full page reload
                navigator.replace(crate::route::Route::TransferWithToken {
                    token_ticker: token.ticker.clone(),
                });
            }
        }
    });

    // Transfer amount
    let mut amount = use_signal::<String>(|| "".to_string());

    // Token balance - use a memo to get the current token mint for use_token_balance
    let token_mint = use_memo(move || selected_token.read().as_ref().map(|token| token.mint));

    // Get token balance using use_token_balance - store in a local variable first
    let token_balance_resource = {
        if let Some(mint) = token_mint() {
            use_token_balance(mint)
        } else {
            use_resource(move || async move { Err(crate::gateway::GatewayError::Unknown.into()) })
        }
    };

    // Convert Resource to Signal
    let mut token_balance = use_signal(|| {
        token_balance_resource
            .cloned()
            .unwrap_or(Err(crate::gateway::GatewayError::Unknown))
    });

    // Update the signal when the resource changes
    use_effect(move || {
        if let Some(balance) = token_balance_resource.cloned() {
            token_balance.set(balance);
        }
    });

    // Error handling
    let err = use_signal::<Option<TokenInputError>>(|| None);

    // Priority fee
    let _priority_fee = use_signal(|| 0);

    // Status
    let mut status = use_signal(|| TransferStatus::Editing);

    // Confirmation dialog
    let show_confirmation = use_signal(|| true);

    let mut destination_pubkey: Signal<String> = use_signal::<String>(|| "".to_string());

    let mut address_err = use_signal::<Option<TransferError>>(|| None);

    // Use the transfer transaction hook
    let tx = use_transfer_transaction(
        destination_pubkey,
        selected_token,
        amount,
        token_balance_resource,
        err,
        // priority_fee,
        address_err,
    );

    on_transaction_done(move |_| {
        status.set(TransferStatus::Success);
        destination_pubkey.set("".to_string());
    });

    // Add validation function
    let validate_destination = move |addr: &str| -> Option<TransferError> {
        if addr.is_empty() {
            None
        } else if Pubkey::try_from(addr).is_err() {
            Some(TransferError::InvalidAddress)
        } else {
            None
        }
    };

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
                            gap: 4,
                            Col {
                                class: "w-full lg:flex elevated elevated-border shrink-0 h-min rounded-xl z-0",
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
                                Col {
                                    class: "w-full p-4",
                                    gap: 2,
                                    span {
                                        class: "text-elements-lowEmphasis ",
                                        "Enter address"
                                    }
                                    Row {
                                        class: "w-full items-center",
                                        div {
                                            class: "flex-grow w-full overflow-hidden mr-2",
                                            input {
                                                class: format!("h-12 outline-none w-full overflow-x-auto {}",
                                                    if matches!(*address_err.read(), Some(TransferError::InvalidAddress)) {
                                                        "text-red-500"
                                                    } else if destination_pubkey.read().is_empty() {
                                                        "text-elements-lowEmphasis"
                                                    } else {
                                                        "text-elements-highEmphasis"
                                                    }),
                                                placeholder: "Enter wallet address",
                                                value: destination_pubkey.clone(),
                                                oninput: move |e: FormEvent| {
                                                    let new_value = e.value();
                                                    address_err.set(validate_destination(&new_value));
                                                    destination_pubkey.set(new_value);
                                                },
                                            }
                                        }
                                        div {
                                            class: "flex-shrink-0 flex-grow-0 w-6",
                                            if !destination_pubkey.read().is_empty() {
                                                match address_err.cloned() {
                                                    Some(TransferError::InvalidAddress) => {
                                                        rsx! {}
                                                    }
                                                    None => {
                                                        rsx! {
                                                            CheckCircleIcon {
                                                                class: "w-5 h-5 text-elements-green"
                                                            }
                                                        }
                                                    }
                                                }

                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // // Fee display
                        // Col {
                        //     class: "px-4",
                        //     gap: 2,
                        //     Fee { priority_fee: priority_fee.clone() }
                        // }

                        // Transfer Button
                        TransferButton {
                            transaction: tx,
                            err,
                            amount,
                            selected_token,
                            destination: destination_pubkey,
                            show_confirmation,
                            address_err
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
    address_err: Signal<Option<TransferError>>,
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
                    let transfer_tx = &*transaction.read();
                    if let Some(Ok(tx)) = transfer_tx {
                        submit_transaction(tx.clone(), TransactionType::Swap);
                    }
                },
                if let Some(err) = err.cloned() {
                    span {
                        class: "mx-auto my-auto font-semibold",
                        "{err.to_string()}"
                    }

                } else if let Some(address_err) = address_err.cloned() {
                    span {
                        class: "mx-auto my-auto font-semibold",
                        "{address_err.to_string()}"
                    }
                } else if amount_f64 > 0.0 {
                    span {
                        class: "mx-auto my-auto font-semibold",
                        "Send {amount_f64} {token_ticker}"
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
                                "Confirm Transfer"
                            }
                            span {
                                class: "text-elements-midEmphasis text-center",
                                "Are you sure you want to transfer {amount_f64} {token_ticker} to this address?"
                            }
                            span {
                                class: "text-elements-highEmphasis text-center bg-surface-elevated p-2 rounded",
                                "{abbreviated_address}"
                            }
                            Row {
                                class: "mt-4",
                                gap: 3,
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

// Component for the /transfer/:token_ticker route
#[component]
pub fn TransferWithToken(token_ticker: String) -> Element {
    rsx! {
        Transfer {
            token_ticker: Some(token_ticker)
        }
    }
}
