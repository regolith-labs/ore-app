use dioxus::prelude::*;

use crate::{
    // components::token_icon::TokenIcon,
    components::*,
    config::Token,
    hooks::{use_token_balance, use_token_balance_for_token},
};

#[component]
pub fn Transfer() -> Element {
    use std::str::FromStr;

    use ore_types::request::TransactionType;
    use steel::Pubkey;

    use crate::hooks::on_transaction_done;

    enum TransferStatus {
        Editing,
        Success,
    }

    // Fixed token (SOL)
    let sol_token = Token::sol();
    let selected_token = use_signal(|| Some(sol_token.clone()));

    // Input amount
    let mut amount = use_signal(|| "0.1".to_string());

    // Destination address
    let mut destination = use_signal(|| "".to_string());
    let destination_pubkey = use_memo(move || {
        if destination.read().is_empty() {
            None
        } else {
            Pubkey::from_str(&destination.read()).ok()
        }
    });

    // Token balance
    let token_balance = use_token_balance_for_token(selected_token);

    // Error handling
    let err = use_signal::<Option<TokenInputError>>(|| None);

    // Priority fee
    let priority_fee = use_signal(|| 0);

    // Status
    let mut status = use_signal(|| TransferStatus::Editing);

    // TODO: Implement transfer transaction hook
    // let tx = use_transfer_transaction(destination_pubkey, selected_token, amount, token_balance, err, priority_fee);

    on_transaction_done(move |_| {
        status.set(TransferStatus::Success);
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
                        // Token Display (Fixed to SOL)
                        Col {
                            class: "mx-auto w-full",
                            gap: 2,
                            span {
                                class: "text-elements-lowEmphasis font-medium",
                                "Token"
                            }
                            div {
                                class: "flex flex-row items-center gap-2 px-4 py-2 rounded-md controls-secondary w-fit",
                                // TokenIcon {
                                //     token: sol_token.clone(),
                                //     size: 24
                                // }
                                span {
                                    class: "font-medium",
                                    "{sol_token.ticker}"
                                }
                            }
                        }

                        // Amount
                        Col {
                            class: "mx-auto w-full",
                            gap: 2,
                            span {
                                class: "text-elements-lowEmphasis font-medium pl-1",
                                "Amount"
                            }
                            Row {
                                class: "w-full justify-between",
                                // Token display
                                Row {
                                    class: "my-auto h-12",
                                    gap: 2,
                                    img {
                                        class: "w-8 h-8 rounded-full my-auto",
                                        src: "{sol_token.image}",
                                    }
                                    span {
                                        class: "font-semibold my-auto",
                                        "{sol_token.ticker}"
                                    }
                                }
                                input {
                                    class: "text-3xl placeholder:text-gray-700 font-semibold bg-transparent h-12 pr-1 my-auto w-full outline-none text-right [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none",
                                    placeholder: "0.0",
                                    r#type: "number",
                                    step: "any",
                                    inputmode: "decimal",
                                    value: amount.clone(),
                                    oninput: move |e: FormEvent| amount.set(e.value()),
                                }
                            }
                        }

                        // Destination
                        Col {
                            class: "mx-auto w-full",
                            gap: 2,
                            span {
                                class: "text-elements-lowEmphasis font-medium pl-1",
                                "Destination"
                            }
                            Row {
                                class: "w-full justify-between items-center px-2 py-2 rounded-md controls-secondary",
                                input {
                                    class: "text-base font-mono bg-transparent h-12 my-auto w-full text-left outline-none text-elements-highEmphasis",
                                    placeholder: "Enter wallet address",
                                    value: destination.clone(),
                                    oninput: move |e: FormEvent| destination.set(e.value()),
                                }
                            }
                            if let Some(pubkey) = destination_pubkey.cloned() {
                                span {
                                    class: "text-elements-green text-sm pl-1",
                                    "Valid address"
                                }
                            } else if !destination.read().is_empty() {
                                span {
                                    class: "text-elements-red text-sm pl-1",
                                    "Invalid address"
                                }
                            }
                        }

                        // Submit Button
                        button {
                            class: "h-12 w-full rounded-full controls-primary transition-all duration-300 ease-in-out hover:not-disabled:scale-105 mt-4",
                            disabled: destination_pubkey.cloned().is_none() || amount.read().parse::<f64>().unwrap_or(0.0) <= 0.0,
                            onclick: move |_| {
                                // TODO: Implement transfer transaction
                                // if let Some(Ok(tx)) = tx.cloned() {
                                //     submit_transaction(tx, TransactionType::Transfer);
                                // }

                                // For now, just show success state
                                status.set(TransferStatus::Success);
                            },
                            span {
                                class: "mx-auto my-auto font-semibold",
                                "Transfer"
                            }
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
                                    amount.set("0.1".to_string());
                                    destination.set("".to_string());
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

// Placeholder for TokenInputError
enum TokenInputError {
    InsufficientBalance,
    InvalidAmount,
}

impl ToString for TokenInputError {
    fn to_string(&self) -> String {
        match self {
            Self::InsufficientBalance => "Insufficient balance".to_string(),
            Self::InvalidAmount => "Invalid amount".to_string(),
        }
    }
}
