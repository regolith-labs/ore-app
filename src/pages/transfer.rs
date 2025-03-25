use crate::route::Route;
use crate::{
    components::submit_transaction,
    components::CheckCircleIcon,
    components::*,
    config::Token,
    gateway::GatewayResult,
    hooks::{use_token_balance_wss, use_transfer_transaction},
};
use dioxus::prelude::*;
use solana_sdk::transaction::VersionedTransaction;

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
    // Selected token
    let mut selected_token = use_signal(|| Some(Token::ore()));

    // Clone token_ticker for different uses
    let token_ticker_for_init = token_ticker.clone();
    let token_ticker_for_sync = token_ticker.clone();

    let navigator = use_navigator();

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

    // Intermediate signal to track URL changes
    let current_url_ticker = use_memo(move || token_ticker.clone());

    // Update the selected token when the URL changes
    use_effect(move || {
        if let Some(ticker) = current_url_ticker() {
            if let Some(token) = crate::config::LISTED_TOKENS_BY_TICKER.get(&ticker) {
                selected_token.set(Some(token.clone()));
            }
        }
    });

    // Transfer amount
    let mut amount = use_signal::<String>(|| "".to_string());

    // We'll use a memo to track token changes instead of a signal we read/write in the same effect
    let _token_mint = use_memo(move || selected_token.read().as_ref().map(|token| token.mint));

    // Effect that resets amount when token changes
    use_effect(move || {
        amount.set("".to_string());
    });

    // Use WebSocket version for UI display
    let mut token_balance = use_signal(|| Err(crate::gateway::GatewayError::Unknown));

    // Update token_balance when selected_token changes
    use_effect(move || {
        if let Some(token) = selected_token.read().as_ref() {
            // Get real-time balance updates via WebSocket
            let balance_wss = use_token_balance_wss(&token.mint);
            token_balance.set(balance_wss.cloned());
        }
    });

    // Error handling
    let err = use_signal::<Option<TokenInputError>>(|| None);

    // Priority fee
    let _priority_fee = use_signal(|| 0);

    // Status
    // let mut status = use_signal(|| TransferStatus::Editing);

    let mut destination_pubkey: Signal<String> = use_signal::<String>(|| "".to_string());

    let mut address_err = use_signal::<Option<TransferError>>(|| None);

    // Use the transfer transaction hook with WebSocket token balance
    let tx = use_transfer_transaction(
        destination_pubkey,
        selected_token,
        amount,
        token_balance,
        err,
        // priority_fee,
        address_err,
    );

    on_transaction_done(move |_| {
        // status.set(TransferStatus::Success);
        destination_pubkey.set("".to_string());
        amount.set("".to_string());
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
                        toolbar_shortcuts: true,
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

            // Transfer Button
            TransferButton {
                transaction: tx,
                err,
                amount,
                selected_token,
                destination: destination_pubkey,
                address_err,
                // status
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
) -> Element {
    let class = class.unwrap_or("controls-primary".to_string());

    // let is_tx_ready = use_memo(move || {
    //     if let Some(Ok(_tx)) = transaction.cloned() {
    //         true
    //     } else {
    //         false
    //     }
    // });

    // let enabled = if let Some(Ok(_)) = transaction.read().as_ref() {
    //     if let Some(_) = err.cloned() {
    //         false
    //     } else {
    //         true
    //     }
    // } else {
    //     false
    // };

    let is_tx_ready = use_memo(move || {
        if let Some(Ok(_tx)) = transaction.cloned() {
            true
        } else {
            false
        }
    });
    let amount_str = amount.read().clone();
    let amount_f64 = amount_str.parse::<f64>().unwrap_or(0.0);
    let token_ticker = selected_token
        .read()
        .as_ref()
        .map(|t| t.ticker.clone())
        .unwrap_or_default();

    let is_disabled = !is_tx_ready.cloned() || err.read().is_some();

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

// Component for the /transfer/:token_ticker route
#[component]
pub fn TransferWithToken(token_ticker: String) -> Element {
    rsx! {
        Transfer {
            token_ticker: Some(token_ticker)
        }
    }
}
