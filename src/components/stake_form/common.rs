use dioxus::prelude::*;
use steel::Pubkey;

use crate::components::{Col, Row};

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum StakeTab {
    Deposit,
    Withdraw,
}

#[component]
pub(crate) fn StakeTabs(tab: Signal<StakeTab>) -> Element {
    // let deposit_class = if *tab.read() == StakeTab::Deposit {
    //     "flex-1 h-12 transition-colors text-elements-highEmphasis border-b-2 border-white"
    // } else {
    //     "flex-1 h-12 transition-colors text-elements-lowEmphasis hover:bg-surface-elevated"
    // };

    // let withdraw_class = if *tab.read() == StakeTab::Withdraw {
    //     "flex-1 h-12 transition-colors text-elements-highEmphasis border-b-2 border-white"
    // } else {
    //     "flex-1 h-12 transition-colors text-elements-lowEmphasis hover:bg-surface-elevated"
    // };

    let deposit_class = if *tab.read() == StakeTab::Deposit {
        "flex-1 h-12 transition-colors rounded-full text-elements-highEmphasis font-semibold bg-controls-secondary"
    } else {
        "flex-1 h-12 transition-colors rounded-full text-elements-lowEmphasis font-semibold"
    };

    let withdraw_class = if *tab.read() == StakeTab::Withdraw {
        "flex-1 h-12 transition-colors rounded-full text-elements-highEmphasis font-semibold bg-controls-secondary"
    } else {
        "flex-1 h-12 transition-colors rounded-full text-elements-lowEmphasis font-semibold"
    };

    rsx! {
        Row {
            // class: "w-full mb-4",
            class: "w-full mb-4 rounded-full border-elevated bg-surface-elevated border border-gray-800 p-1",
            gap: 1,
            button {
                class: "{deposit_class}",
                onclick: move |_| tab.set(StakeTab::Deposit),
                "Deposit"
            }
            button {
                class: "{withdraw_class}",
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
pub(crate) fn StakeButton(enabled: Signal<bool>) -> Element {
    let colors = if *enabled.read() {
        "controls-primary"
    } else {
        "bg-controls-disabled text-on-onDisabled"
    };
    rsx! {
        button {
            class: "h-12 w-full rounded-full {colors}",
            disabled: !*enabled.read(),
            onclick: move |_| {
                // TODO: Implement staking logic
            },
            span {
                class: "mx-auto my-auto font-semibold",
                "Submit"
            }
        }
    }
}

#[component]
pub(crate) fn WithdrawButton(enabled: Signal<bool>) -> Element {
    let colors = if *enabled.read() {
        "controls-primary"
    } else {
        "bg-controls-disabled text-on-onDisabled"
    };
    rsx! {
        button {
            class: "h-12 w-full rounded-full {colors}",
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

