use dioxus::prelude::*;

use crate::{components::*, hooks::use_ore_supply, route::Route};

pub fn Landing() -> Element {
    rsx! {
        Col {
            class: "flex h-full w-full overflow-y-auto mx-auto py-8 pb-20 sm:pb-16",
            Hero {}
            TickerTape {}
            // ChartSection {}
            // LiquiditySection {}
            // SupplyStats {}
        }
    }
}

fn Hero() -> Element {
    rsx! {
        Col {
            class: "mx-auto my-auto px-5 gap-16",
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
                    class: "mx-auto mt-8 font-wide text-xl sm:text-2xl text-center text-elements-midEmphasis",
                    "A hard money standard for onchain commodities."
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

fn TickerTape() -> Element {
    rsx! {
        div {
            class: "relative flex w-full h-12 mt-16",

            // First set of items
            div {
                class: "animate-marquee whitespace-nowrap py-12",
                span {
                    class: "inline-block mx-16 font-wide text-elements-lowEmphasis",
                    "SOL/ORE"
                }
                span {
                    class: "inline-block mx-16 font-wide text-elements-lowEmphasis", 
                    "USDC/ORE"
                }
                span {
                    class: "inline-block mx-16 font-wide text-elements-lowEmphasis",
                    "USDT/ORE"
                }
                span {
                    class: "inline-block mx-16 font-wide text-elements-lowEmphasis",
                    "wBTC/ORE"
                }
                span {
                    class: "inline-block mx-16 font-wide text-elements-lowEmphasis",
                    "HNT/ORE" 
                }
                span {
                    class: "inline-block mx-16 font-wide text-elements-lowEmphasis",
                    "HONEY/ORE"
                }
                span {
                    class: "inline-block mx-16 font-wide text-elements-lowEmphasis",
                    "ISC/ORE"
                }
            }
            
            // Duplicate set for seamless loop
            div {
                class: "absolute top-0 animate-marquee2 whitespace-nowrap py-12",
                span {
                    class: "inline-block mx-16 font-wide text-elements-lowEmphasis",
                    "SOL/ORE"
                }
                span {
                    class: "inline-block mx-16 font-wide text-elements-lowEmphasis",
                    "USDC/ORE" 
                }
                span {
                    class: "inline-block mx-16 font-wide text-elements-lowEmphasis",
                    "USDT/ORE"
                }
                span {
                    class: "inline-block mx-16 font-wide text-elements-lowEmphasis",
                    "wBTC/ORE"
                }
                span {
                    class: "inline-block mx-16 font-wide text-elements-lowEmphasis",
                    "HNT/ORE" 
                }
                span {
                    class: "inline-block mx-16 font-wide text-elements-lowEmphasis",
                    "HONEY/ORE"
                }
                span {
                    class: "inline-block mx-16 font-wide text-elements-lowEmphasis",
                    "ISC/ORE"
                }
            }
        }
    }
}

fn ChartSection() -> Element {
    rsx! {
        Col {
            class: "flex w-full",
            img {
                class: "w-full sm:hidden",
                src: "/assets/blurchart_390.png",
            }
            img {
                class: "w-full hidden sm:block", 
                src: "/assets/blurchart_1920.png",
            }
        }
    }
}

fn LiquiditySection() -> Element {
    rsx! {
        Col {
            // class: "w-full py-32 pb-64 sm:py-48 sm:pb-64 gap-2 sm:gap-4 px-5 bg-[url('/assets/blurchart_390.png')] sm:bg-[url('/assets/blurchart_1920.png')] bg-cover bg-center",
            class: "w-full pt-16 pb-64 sm:pb-64 gap-2 sm:gap-4 px-3 sm:px-6 bg-[url('/assets/blurchart_1920.png')] bg-cover bg-center",

            // class: "w-full sm:py-48 gap-2 sm:gap-4 px-5",
            span {
                class: "font-wide font-semibold text-2xl sm:text-3xl text-elements-highEmphasis",
                "Solving for liquidity."
            }
            span {
                class: "max-w-xl sm:text-lg text-elements-lowEmphasis",
                "ORE works with teams issuing novel defi products such as RWAs and DePIN credits to seed market liquidity and connect with likeminded traders."
            }
        }
    }
}

fn SupplyStats() -> Element {
    let circulating_supply = use_ore_supply();
    rsx! {
        Row {
            class: "mx-auto py-16 gap-16 sm:gap-32",
            if let Some(Ok(circulating_supply)) = circulating_supply.cloned() {
                SupplyValue {
                    title: "Circulating",
                    value: circulating_supply.ui_amount_string,
                }
            } else {
                SupplyValue {
                    title: "Circulating",
                    value: None,
                }
            }
            SupplyValue {
                title: "Total",
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

