use dioxus::prelude::*;

use crate::{
    components::WarningIcon,
    hooks::{use_balance, UiTokenAmountBalance},
};

use super::{Stake, StakeStep};

#[component]
pub fn StakeEdit(
    step: Signal<StakeStep>,
    amount_input: Signal<String>,
    parsed_amount: u64,
    stake: Stake,
) -> Element {
    let balance = use_balance(stake.mint, stake.decimals);
    let (max_amount, max_amount_str) = balance
        .cloned()
        .and_then(|b| b.ok())
        .map(|b| (b.balance(), b.ui_amount_string))
        .unwrap_or_else(|| (0, "0".to_owned()));

    let error_text = if parsed_amount.gt(&max_amount) {
        Some("Amount too large".to_string())
    } else {
        None
    };

    let is_disabled = amount_input.read().len().eq(&0)
        || amount_input.read().parse::<f64>().is_err()
        || error_text.is_some();

    let title = format!("Stake {} with your miner.", stake.name);
    let action = format!(
        "This will transfer {} to your miner and increase your mining multiplier.",
        stake.name
    );

    rsx! {
        div {
            class: "flex flex-col h-full grow justify-between",
            style: "margin-bottom: 50px;",
            div {
                class: "flex flex-col gap-4 -mt-3.5 mb-4",
                div {
                    class: "flex flex-col gap-3",
                    p {
                        class: "text-lg",
                        "{title}"
                    }
                    p {
                        class: "text-sm text-gray-300",
                        "{action}"
                    }
                }
            }
            div {
                class: "flex flex-col gap-8",
                if let Some(error_text) = error_text {
                    p {
                        class: "flex flex-row flex-nowrap gap-2 text-white w-min mx-auto text-nowrap bg-red-500 text-center font-semibold text-sm rounded py-1 px-2",
                        WarningIcon {
                            class: "w-3.5 h-3.5 my-auto"
                        }
                        "{error_text}"
                    }
                }
                input {
                    autofocus: true,
                    class: "mx-auto w-full text-center focus:ring-0 outline-none placeholder-gray-200 dark:placeholder-gray-800 bg-transparent text-3xl sm:text-4xl md:text-5xl font-medium",
                    value: "{amount_input}",
                    placeholder: "0",
                    oninput: move |evt| {
                        let s = evt.value();
                        if s.len().eq(&0) || s.parse::<f64>().is_ok() {
                            amount_input.set(s);
                        } else {
                            amount_input.set(s[..s.len()-1].to_string());
                        }
                    },
                }
                button {
                    class: "flex transition-colors shrink text-nowrap py-2 px-4 mx-auto text-center text-nowrap rounded-full font-medium hover-100 active-200",
                    onclick: move |_| { amount_input.set(max_amount_str.clone()) },
                    "Max"
                }
            }
            div {
                class: "flex flex-col sm:flex-row gap-2",
                button {
                    class: "w-full py-3 rounded font-semibold transition-colors transition-opacity text-white bg-green-500 hover:bg-green-600 active:bg-green-700 disabled:opacity-20",
                    disabled: is_disabled,
                    onclick: move |_| {
                        step.set(StakeStep::Confirm);
                    },
                    "Review"
                }
            }
        }
    }
}
