use dioxus::prelude::*;

use crate::components::Row;

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