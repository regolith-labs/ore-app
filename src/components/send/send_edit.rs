use std::str::FromStr;

use dioxus::prelude::*;
use dioxus_router::hooks::use_navigator;
#[cfg(feature = "web")]
use solana_client_wasm::solana_sdk::pubkey::Pubkey;
#[cfg(feature = "desktop")]
use solana_sdk::pubkey::Pubkey;

use crate::{
    components::WarningIcon,
    gateway::AsyncResult,
    hooks::{use_ore_balance, UiTokenAmountBalance},
};

use super::SendStep;

// TODO Break recipient, amount, and memo into sequential fullscreen steps

#[derive(Props)]
pub struct SendEditProps<'a> {
    pub send_step: &'a UseState<SendStep>,
    pub amount_input: &'a UseState<String>,
    pub recipient_input: &'a UseState<String>,
    pub memo_input: &'a UseState<String>,
    pub parsed_amount: u64,
}

// TODO Max size on memo

#[component]
pub fn SendEdit<'a>(cx: Scope<'a, SendEditProps<'a>>) -> Element {
    let nav = use_navigator(cx);
    let ore_balance = use_ore_balance(cx);
    let amount_input = cx.props.amount_input;
    let recipient_input = cx.props.recipient_input;
    let memo_input = cx.props.memo_input;
    let send_step = cx.props.send_step;
    let recipient = Pubkey::from_str(recipient_input.get());
    let (max_amount, max_amount_str) = match ore_balance {
        AsyncResult::Ok(balance) => (balance.balance(), balance.ui_amount_string),
        _ => (0, "0".to_owned()),
    };

    let amount_error_text = if cx.props.parsed_amount.gt(&max_amount) {
        Some("Amount too large".to_string())
    } else {
        None
    };

    let recipient_error_text = match recipient {
        Ok(_) => None,
        Err(_) => {
            if recipient_input.get().len().gt(&0) {
                Some("Invalid address".to_string())
            } else {
                None
            }
        }
    };

    let memo = memo_input.get().trim();

    let is_disabled = amount_input.get().len().eq(&0)
        || amount_input.get().parse::<f64>().is_err()
        || amount_error_text.is_some()
        || recipient.is_err()
        || memo.len().eq(&0);

    render! {
        div {
            class: "flex flex-col h-full grow gap-12",
            div {
                class: "flex flex-col gap-3",
                h2 {
                    "Transfer"
                }
                p {
                    class: "text-lg",
                    "Send Ore to another user."
                }
                // p {
                //     class: "text-gray-300 text-sm",
                //     "This will transfer Ore from your account to another."
                // }
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
                        autofocus: recipient_input.get().eq(&""),
                        placeholder: "Address",
                        value: "{recipient_input.get()}",
                        oninput: move |evt| {
                            let s = evt.value.clone();
                            recipient_input.set(s);
                        },
                    }
                    if let Some(err) = recipient_error_text {
                        render! {
                            p {
                                class: "flex flex-row flex-nowrap gap-1.5 w-min text-nowrap text-red-500 font-semibold text-sm",
                                WarningIcon {
                                    class: "w-4 h-4 my-auto"
                                }
                                "{err}"
                            }
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
                            autofocus: !recipient_input.get().eq(&""),
                            value: "{amount_input}",
                            placeholder: "0",
                            oninput: move |evt| {
                                let s = evt.value.clone();
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
                        render! {
                            p {
                                class: "flex flex-row flex-nowrap gap-1.5 w-min text-nowrap text-red-500 font-semibold text-sm",
                                WarningIcon {
                                    class: "w-4 h-4 my-auto"
                                }
                                "{err}"
                            }
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
                        oninput: move |evt| {
                            let s = evt.value.clone();
                            memo_input.set(s);
                        },
                    }
                }
            }
            div {
                class: "flex flex-col sm:flex-row gap-2 mt-auto",
                button {
                    class: "w-full py-3 rounded font-semibold transition-colors hover-100 active-200",
                    onclick: move |_| {
                        nav.go_back();
                    },
                    "Cancel"
                }
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
