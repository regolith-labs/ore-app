use dioxus::prelude::*;

use crate::components::Row;

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum TradeTab {
    Swap,
    Recurring,
}

#[component]
pub(crate) fn TradeTabs(tab: Signal<TradeTab>) -> Element {
    let common_class =
        "flex-1 h-12 transition-colors rounded-full font-semibold hover:cursor-pointer";

    let swap_class = if *tab.read() == TradeTab::Swap {
        "text-elements-highEmphasis bg-controls-secondary"
    } else {
        "text-elements-lowEmphasis"
    };

    let recurring_class = if *tab.read() == TradeTab::Recurring {
        "text-elements-highEmphasis bg-controls-secondary"
    } else {
        "text-elements-lowEmphasis"
    };

    rsx! {
        Row {
            class: "w-full mb-4 rounded-full border-elevated bg-surface-elevated border border-gray-800 p-1",
            gap: 1,
            button {
                class: "{common_class} {swap_class}",
                onclick: move |_| tab.set(TradeTab::Swap),
                "Swap"
            }
            button {
                class: "{common_class} {recurring_class}",
                onclick: move |_| tab.set(TradeTab::Recurring),
                "Recurring"
            }
        }
    }
}
