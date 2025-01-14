use dioxus::prelude::*;

use crate::{components::*, hooks::use_ore_supply, route::Route};

pub fn Landing() -> Element {
    rsx! {
        Col {
            class: "flex h-full w-full overflow-y-auto mx-auto px-5 py-8 pb-20 sm:pb-16",
            gap: 16,
            Hero {}
            SupplyStats {}
        }
    }
}

fn Hero() -> Element {
    rsx! {
        Col {
            class: "mx-auto my-auto gap-16",
            gap: 16,
            OrbHero {}
            Col {
                class: "mx-auto",
                gap: 2,
                span {
                    class: "mx-auto font-wide font-bold text-4xl sm:text-5xl text-elements-highEmphasis",
                    "Digital gold."
                }
                span {
                    class: "mx-auto font-wide font-bold text-4xl sm:text-5xl text-elements-lowEmphasis",
                    "On Solana."
                }
                span {  
                    class: "mx-auto mt-8 font-wide font-light text-xl sm:text-2xl text-center text-elements-midEmphasis",
                    "A hard money standard for tokenized commodity markets."
                }
            }
            Link {
                class: "flex mx-auto h-16 w-full sm:max-w-xs px-4 rounded-full controls-primary transition-transform hover:scale-105",
                to: Route::Mine {},
                span {
                    class: "font-semibold text-lg sm:text-xl mx-auto my-auto",
                    "Start mining →"
                }
            }
        }
    }
}

fn SupplyStats() -> Element {
    let circulating_supply = use_ore_supply();
    rsx! {
        Row {
            class: "mx-auto gap-16 sm:gap-32",
            if let Some(Ok(circulating_supply)) = circulating_supply.cloned() {
                SupplyValue {
                    title: "Circulating supply",
                    value: circulating_supply.ui_amount_string,
                }
            } else {
                SupplyValue {
                    title: "Circulating supply",
                    value: None,
                }
            }
            SupplyValue {
                title: "Total supply",
                value: Some("5000000".to_string()),
            }
        }
    }
}

#[component]
fn SupplyValue(title: String, value: Option<String>) -> Element {
    rsx! {
        Col {
            gap: 2,
            span {
                class: "font-wide text-sm text-center text-nowrap sm:text-base text-elements-lowEmphasis mx-auto my-auto",
                "{title}"
            }
            if let Some(value) = value {
                OreValueWhole {
                    class: "mx-auto",
                    ui_amount_string: value,
                }
            } else {
                div {
                    class: "h-10 w-32 mx-auto loading rounded"
                }
            }
        }
    }
}
