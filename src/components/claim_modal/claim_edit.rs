use dioxus::prelude::*;
use dioxus_router::hooks::use_navigator;

use crate::components::WarningIcon;

use super::ClaimStep;

#[derive(Props)]
pub struct ClaimEditProps<'a> {
    pub claim_step: &'a UseState<ClaimStep>,
    pub amount_input: &'a UseState<String>,
    pub parsed_amount: u64,
    pub max_rewards: u64,
}

#[component]
pub fn ClaimEdit<'a>(cx: Scope<'a, ClaimEditProps<'a>>) -> Element {
    let amount_input = cx.props.amount_input;
    let claim_step = cx.props.claim_step;
    let max_rewards = (cx.props.max_rewards as f64) / 10f64.powf(ore::TOKEN_DECIMALS.into());
    let nav = use_navigator(cx);

    let error_text: Option<String> = if cx.props.parsed_amount.gt(&cx.props.max_rewards) {
        Some("Amount too large".to_string())
    } else {
        None
    };

    let is_disabled = amount_input.get().len().eq(&0)
        || amount_input.get().parse::<f64>().is_err()
        || error_text.is_some();

    render! {
        div {
            class: "flex flex-col h-full grow justify-between",
            div {
                class: "flex flex-col gap-3",
                h1 {
                    "Claim"
                }
                p {
                    class: "text-lg",
                    "Select an amount of rewards to claim."
                }
                p {
                    class: "text-sm text-gray-300 dark:text-gray-700",
                    "Upon claiming, this amount will be immediately added to your balance in the dashboard."
                }
            }
            div {
                class: "flex flex-col gap-8",
                if let Some(error_text) = error_text {
                    render! {
                        p {
                            class: "flex flex-row flex-nowrap gap-2 text-white w-min mx-auto text-nowrap bg-red-500 text-center font-semibold text-sm rounded py-1 px-2",
                            WarningIcon {
                                class: "w-3.5 h-3.5 my-auto"
                            }
                            "{error_text}"
                        }
                    }
                }
                input {
                    autofocus: true,
                    class: "mx-auto w-full text-center focus:ring-0 outline-none placeholder-gray-200 dark:placeholder-gray-800 bg-transparent text-3xl sm:text-4xl md:text-5xl font-medium",
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
                    class: "flex transition-colors w-min text-nowrap py-2 px-4 mx-auto text-center text-nowrap rounded-full font-medium hover-100 active-200",
                    onclick: move |_| {
                        amount_input.set(max_rewards.to_string())
                    },
                    "Max: {max_rewards}"
                }
            }
            div {
                class: "flex flex-col sm:flex-row gap-2",
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
                        claim_step.set(ClaimStep::Confirm);
                    },
                    "Review"
                }
            }
        }
    }
}
