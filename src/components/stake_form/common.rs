use dioxus::prelude::*;

use crate::components::{Col, Row};

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum StakeTab {
    Deposit,
    Withdraw,
}

#[component]
pub(crate) fn StakeTabs(tab: Signal<StakeTab>) -> Element {
    let common_class = "flex-1 h-12 transition-colors rounded-full font-semibold hover:cursor-pointer";

    let deposit_class = if *tab.read() == StakeTab::Deposit {
        "text-elements-highEmphasis bg-controls-secondary"
    } else {
        "text-elements-lowEmphasis"
    };

    let withdraw_class = if *tab.read() == StakeTab::Withdraw {
        "text-elements-highEmphasis bg-controls-secondary"
    } else {
        "text-elements-lowEmphasis"
    };

    rsx! {
        Row {
            class: "w-full mb-4 rounded-full border-elevated bg-surface-elevated border border-gray-800 p-1",
            gap: 1,
            button {
                class: "{common_class} {deposit_class}",
                onclick: move |_| tab.set(StakeTab::Deposit),
                "Deposit"
            }
            button {
                class: "{common_class} {withdraw_class}",
                onclick: move |_| tab.set(StakeTab::Withdraw),
                "Withdraw"
            }
        }
    }
}

pub(crate) fn StakeDetails() -> Element {
    rsx! {
        Col {
            class: "px-1",
            gap: 3,
            DetailLabel {
                title: "Estimated yield",
                value: "1 ORE / day"
            }
            DetailLabel {
                title: "Transaction fee",
                value: "0.00005 SOL"
            }
        }
    }
}

pub(crate) fn WithdrawDetails() -> Element {
    rsx! {
        Col {
            class: "px-1",
            gap: 3,
            DetailLabel {
                title: "Transaction fee",
                value: "0.00005 SOL"
            }
        }
    }
}

#[component]
pub(crate) fn DetailLabel(title: String, value: String) -> Element {
    rsx! {
        Row {
            class: "w-full justify-between text-sm",
            span {
                class: "text-elements-lowEmphasis",
                "{title}"
            }
            span {
                class: "text-elements-midEmphasis",
                "{value}"
            }
        }
    }
}

#[component]
pub(crate) fn SubmitButton(enabled: bool, onclick: EventHandler<MouseEvent>) -> Element {
    rsx! {
        button {
            class: "h-12 w-full rounded-full controls-primary transition-transform hover:not-disabled:scale-105",
            disabled: !enabled,
            onclick: onclick,
            span {
                class: "mx-auto my-auto font-semibold",
                "Submit"
            }
        }
    }
}

#[component]
pub(crate) fn WithdrawButton(enabled: Signal<bool>) -> Element {
    rsx! {
        button {
            class: "h-12 w-full rounded-full controls-primary",
            disabled: !*enabled.read(),
            onclick: move |_| {
                // TODO: Implement withdraw logic
            },
            span {
                class: "mx-auto my-auto font-semibold",
                "Submit"
            }
        }
    }
}

