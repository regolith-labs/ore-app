use dioxus::prelude::*;

use crate::components::*;

// TODO Price chart component
// TODO Activity component
pub fn Trade() -> Element {
    rsx! {
        Col {
            class: "w-full h-full pb-20 sm:pb-16",
            gap: 8,
            Heading {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                title: "Trade",
                subtitle: "Swap tokens at the best available price."
            }
            TradeForm {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
            }
            // SwapForm {
            //     class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
            // }
        }
    }
}

#[component]
pub fn TradeForm(class: Option<String>) -> Element {
    let class = class.unwrap_or_default();
    let tab = use_signal(|| TradeTab::Swap);
    rsx! {
        Col {
            class: "{class}",
            TradeTabs {
                tab: tab
            }
            match *tab.read() {
                TradeTab::Swap => rsx! {
                    SwapForm {
                        class: "mx-auto w-full max-w-2xl sm:px-8",
                    }
                },
                TradeTab::Recurring => rsx! {
                    RecurringForm {
                        class: "mx-auto w-full max-w-2xl sm:px-8",
                    }
                }
            }
        }
    }
}
