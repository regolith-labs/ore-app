use std::str::FromStr;

use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;

use crate::{
    components::{BackButton, WarningIcon},
    hooks::{use_ore_balance, UiTokenAmountBalance},
};

use super::SendStep;

// TODO Break recipient, amount, and memo into sequential fullscreen steps
// TODO Max size on memo

#[component]
pub fn SendEdit(
    send_step: Signal<SendStep>,
    amount_input: Signal<String>,
    recipient_input: Signal<String>,
    memo_input: Signal<String>,
    parsed_amount: u64,
) -> Element {
    let nav = navigator();
    let balance = use_ore_balance();
    let recipient = Pubkey::from_str(recipient_input.read().as_str());
    let (max_amount, max_amount_str) = balance
        .cloned()
        .and_then(|b| b.ok())
        .map(|b| (b.balance(), b.ui_amount_string))
        .unwrap_or_else(|| (0, "0".to_owned()));
    let amount_error_text = if parsed_amount.gt(&max_amount) {
        Some("Amount too large".to_string())
    } else {
        None
    };
    let recipient_error_text = match recipient {
        Ok(_) => None,
        Err(_) => {
            if recipient_input.read().len().gt(&0) {
                Some("Invalid address".to_string())
            } else {
                None
            }
        }
    };

    let is_disabled = amount_input.read().len().eq(&0)
        || amount_input.read().parse::<f64>().is_err()
        || amount_error_text.is_some()
        || recipient.is_err()
        || memo_input.read().trim().len().eq(&0);

    rsx! {
        div {
            class: "flex flex-col h-full grow gap-12",
            div {
                class: "flex flex-col gap-4 -mt-3.5 mb-4",
                BackButton {
                    onclick: move |_| {
                        nav.go_back()
                    }
                }
                div {
                    class: "flex flex-col gap-2",
                    h2 {
                        "Transfer"
                    }
                    p {
                        class: "text-lg",
                        "Send ORE to anyone, anywhere in the world."
                    }
                    p {
                        class: "text-sm text-gray-300",
                        "This transaction will be executed and settled on the Solana blockchain."
                    }
                }
            }
            div {
                class: "flex flex-col gap-12",
                div {
                    class: "flex flex-col gap-3",
                    p {
                        class: "font-semibold text-sm",
                        "To"
                    }
                    input {
                        class: "mx-auto w-full focus:ring-0 outline-none placeholder-gray-200 dark:placeholder-gray-700 bg-transparent text-xl",
                        autofocus: recipient_input.read().eq(&""),
                        placeholder: "Address",
                        value: "{*recipient_input.read()}",
                        oninput: move |e| {
                            recipient_input.set(e.value());
                        },
                    }
                    if let Some(err) = recipient_error_text {
                        p {
                            class: "flex flex-row flex-nowrap gap-1.5 w-min text-nowrap text-red-500 font-semibold text-sm",
                            WarningIcon {
                                class: "w-4 h-4 my-auto"
                            }
                            "{err}"
                        }
                    }
                }
                div {
                    class: "flex flex-col gap-2",
                    p {
                        class: "font-semibold text-sm",
                        "Amount"
                    }
                    div {
                        class: "flex flex-row gap-3",
                        input {
                            class: "mx-auto w-full focus:ring-0 outline-none placeholder-gray-200 dark:placeholder-gray-700 bg-transparent text-xl font-medium",
                            autofocus: !recipient_input.read().eq(&""),
                            value: "{amount_input}",
                            placeholder: "0",
                            oninput: move |e| {
                                let s = e.value();
                                if s.len().eq(&0) || s.parse::<f64>().is_ok() {
                                    amount_input.set(s);
                                } else {
                                    amount_input.set(s[..s.len()-1].to_string());
                                }
                            },
                        }
                        button {
                            class: "flex transition-colors w-min text-nowrap py-2 px-4 mx-auto text-center rounded-full text-sm font-medium hover-100 active-200",
                            onclick: move |_| {
                                amount_input.set(max_amount_str.clone());
                            },
                            "Max"
                        }
                    }
                    if let Some(err) = amount_error_text {
                        p {
                            class: "flex flex-row flex-nowrap gap-1.5 w-min text-nowrap text-red-500 font-semibold text-sm",
                            WarningIcon {
                                class: "w-4 h-4 my-auto"
                            }
                            "{err}"
                        }
                    }
                }
                div {
                    class: "flex flex-col gap-3",
                    p {
                        class: "font-semibold text-sm",
                        "Memo"
                    }
                    input {
                        class: "mx-auto w-full focus:ring-0 outline-none placeholder-gray-200 dark:placeholder-gray-700 bg-transparent text-xl",
                        placeholder: "What's this for?",
                        oninput: move |e| {
                            memo_input.set(e.value());
                        },
                    }
                }
            }
            div {
                class: "flex flex-col sm:flex-row gap-2 mt-auto",
                button {
                    class: "w-full py-3 rounded font-semibold transition-colors transition-opacity text-white bg-green-500 hover:bg-green-600 active:bg-green-700 disabled:opacity-20",
                    disabled: is_disabled,
                    onclick: move |_| {
                        send_step.set(SendStep::Confirm);
                    },
                    "Review"
                }
            }
        }
    }
}
