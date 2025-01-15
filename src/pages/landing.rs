use dioxus::prelude::*;

use crate::{components::*, hooks::use_ore_supply, route::Route};

pub fn Landing() -> Element {
    rsx! {
        Col {
            class: "relative flex h-full w-full overflow-y-auto mx-auto pb-20 sm:pb-16",
            Col {
                class: "flex flex-col w-full h-full min-h-[calc(100vh-2rem)] sm:min-h-[calc(100vh-8rem)] justify-between bg-[url('/assets/dot-grid.png')] bg-auto bg-no-repeat bg-top",
                Hero {}
                MarqeeSection {}
            }
            LiquiditySection {}
            SupplySection {}
            MiningSection {}
        }
    }
}

fn Hero() -> Element {
    rsx! {
        Col {
            class: "mx-auto my-auto pt-0 px-5 gap-16",
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
                    "The hard money standard for tokenized assets and commodities."
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

fn MarqeeSection() -> Element {
    rsx! {
        div {
            class: "relative flex w-full mt-8 overflow-clip",

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
            class: "w-full mt-24 sm:mt-32 pt-0 pb-64 sm:pb-64 px-3 sm:px-8 md:px-16 lg:px-24 bg-[url('/assets/blurchart-1920.png')] bg-contain bg-no-repeat bg-center",
            Col {
                class: "gap-2 sm:gap-4 mx-auto w-full max-w-[96rem]",
                span {
                    class: "font-wide font-semibold text-2xl sm:text-3xl text-elements-highEmphasis text-center sm:text-left",
                    "Solving for liquidity."
                }
                span {
                    class: "max-w-xl sm:text-lg text-elements-lowEmphasis text-center sm:text-left",
                    "ORE works with issuers of novel defi assets to provide market liquidity and connect with a community of likeminded investors."
                }
            }
        }
    }
}

fn SupplySection() -> Element {
    let circulating_supply = use_ore_supply();
    rsx! {
        Col {   
            class: "w-full py-16 gap-12",
            Col {
                class: "px-3 sm:px-8",
                gap: 4,
                span {
                    class: "font-wide font-semibold mx-auto text-2xl sm:text-3xl text-elements-highEmphasis",
                    "Supply is limited."
                }
                span {
                    class: "max-w-xl text-center sm:text-lg text-elements-lowEmphasis mx-auto my-auto",
                    "ORE follows a fixed emissions schedule and deflationary supply curve."
                }
            }
            Row {
                class: "mx-auto gap-16 sm:gap-32",
                if let Some(Ok(circulating_supply)) = circulating_supply.cloned() {
                SupplyValue {
                    title: "CURRENT",
                    value: circulating_supply.ui_amount_string,
                    }
                } else {
                    SupplyValue {
                        title: "CURRENT",
                        value: None,
                    }
                }
                SupplyValue {
                    title: "TOTAL",
                    value: Some("5000000".to_string()),
                }
            }
        }
    }
}

#[component]
fn SupplyValue(title: String, value: Option<String>) -> Element {
    rsx! {
        Col {
            gap: 2,
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
            span {
                class: "font-wide font-semibold text-xs sm:text-sm text-center text-nowrap text-elements-lowEmphasis mx-auto my-auto",
                "{title}"
            }
        }
    }
}


fn MiningSection() -> Element {
    rsx! {
        div {
            class: "mt-24 sm:mt-40 pt-0 pb-64 sm:pb-64 px-3 sm:px-8 md:px-16 lg:px-24",
            div {
                class: "flex flex-col-reverse md:flex-row w-full max-w-[96rem] mx-auto",
                Col {
                    class: "w-full gap-2 sm:gap-4 -mt-64 md:mt-0",
                    span {
                        class: "font-wide font-semibold text-2xl sm:text-3xl text-elements-highEmphasis",
                        "Mine everywhere."
                    }
                    span {
                        class: "max-w-xl sm:text-lg text-elements-lowEmphasis",
                        "No advanced hardware required."
                    }
                }
                img {
                    class: "w-96 mx-auto md:mx-0 lg:mx-16 xl:mx-32",
                    src: "/assets/demo-miner.png",
                }
            }
        }
    }
}