use std::str::FromStr;

use dioxus::prelude::*;
use jupiter_swap_api_client::quote::QuoteResponse;
use steel::Pubkey;

use crate::{components::*, gateway::GatewayResult, hooks::{use_ore_quote, use_ore_supply, use_quote}, route::Route};

pub fn Landing() -> Element {
    rsx! {
        Col {
            class: "relative flex h-full justify-between w-full overflow-y-auto mx-auto pt-8 pb-20 sm:pb-16 gap-16 bg-[url('/assets/dot-grid.png')] bg-auto bg-no-repeat bg-top",
            Hero {}
            MarqeeSection {}
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
    let wbtc_mint = Pubkey::from_str("3NZ9JMVBmGAqocybic2c7LQCJScmgsAZ6vQqTDzcqmJh").unwrap();
    let sol_mint = Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap();
    let usdc_mint = Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap();
    let btc_quote = use_ore_quote(wbtc_mint);
    let sol_quote = use_ore_quote(sol_mint);
    let usdc_quote = use_ore_quote(usdc_mint);
    rsx! {
        Col {
            class: "w-full mt-8 overflow-clip",
            span {
                class: "font-wide font-semibold text-elements-lowEmphasis mx-auto",
                "1 ORE ≈"
            }
            div {
                class: "relative flex w-full overflow-clip",
                MarqeePanel {
                    class: "animate-marquee whitespace-nowrap py-8",
                    sol_quote: sol_quote.clone(),
                    usdc_quote: usdc_quote.clone(),
                    btc_quote: btc_quote.clone(),
                }
                MarqeePanel {
                    class: "absolute top-0 animate-marquee2 whitespace-nowrap py-8",
                    sol_quote: sol_quote.clone(),
                    usdc_quote: usdc_quote.clone(),
                    btc_quote: btc_quote.clone(),
                }
            }
        }
    }
}

// TODO: Dynamic gold quote
// TODO: Dynamic oil quote
#[component]
fn MarqeePanel(
    class: String, 
    sol_quote: Resource<GatewayResult<QuoteResponse>>, 
    usdc_quote: Resource<GatewayResult<QuoteResponse>>, 
    btc_quote: Resource<GatewayResult<QuoteResponse>>
) -> Element {
    rsx! {
        div {
            class: "{class} font-wide font-semibold text-elements-highEmphasis",

            // SOL
            img {
                class: "inline-block my-auto w-10 h-10 ml-16 mr-4 ",
                src: "https://upload.wikimedia.org/wikipedia/en/b/b9/Solana_logo.png",
                alt: "Solana logo"
            }
            if let Some(Ok(quote)) = &*sol_quote.value().read_unchecked() {
                span {
                    class: "inline-block my-auto mr-2",
                    "{quote.out_amount as f64 / 1_000_000_000.0:.3}"
                }
                span {
                    class: "inline-block my-auto mr-16 text-elements-lowEmphasis",
                    "SOL"
                }
            } else {
                span {
                    class: "inline-block my-auto h-10 w-16 loading rounded mr-16",
                }
            }

            // USDC
            img {
                class: "inline-block my-auto w-10 h-10 ml-16 mr-4 ",
                src: "https://cdn.prod.website-files.com/66327d2c71b7019a2a9a1b62/667454fd94c7f58e94f4a009_USDC-webclip-256x256.png",
                alt: "USDC logo"
            }
            if let Some(Ok(quote)) = &*usdc_quote.value().read_unchecked() {
                span {
                    class: "inline-block my-auto mr-2",
                    "{quote.out_amount as f64 / 1_000_000.0:.2}"
                }
                span {
                    class: "inline-block my-auto mr-16 text-elements-lowEmphasis",
                    "USD"
                }
            } else {
                span {
                    class: "inline-block my-auto h-10 w-16 loading rounded mr-16",
                }
            }

            // BTC
            img {
                class: "inline-block my-auto w-10 h-10 ml-16 mr-4 ",
                src: "https://upload.wikimedia.org/wikipedia/commons/thumb/4/46/Bitcoin.svg/1200px-Bitcoin.svg.png",
                alt: "BTC logo"
            }
            if let Some(Ok(quote)) = &*btc_quote.value().read_unchecked() {
                span {
                    class: "inline-block my-auto mr-2",
                    "{quote.out_amount as f64 / 100_000_000.0}"
                }
                span {
                    class: "inline-block my-auto mr-16 text-elements-lowEmphasis",
                    "BTC"
                }
            } else {
                span {
                    class: "inline-block my-auto h-10 w-16 loading rounded mr-16",
                }
            }

            // Gold
            img {
                class: "inline-block my-auto h-8 ml-16 mr-4 ",
                src: "https://wallpapers.com/images/hd/gold-bars-transparent-background-v7dez4tziavzufj7.jpg",
                alt: "Gold ingots",
            }
            if let Some(Ok(quote)) = &*usdc_quote.value().read_unchecked() {
                span {
                    class: "inline-block my-auto mr-2",
                    "{quote.out_amount as f64 / 1_000_000.0 / 86.88:.3}"
                }
                span {
                    class: "inline-block my-auto mr-16 text-elements-lowEmphasis",
                    "grams"
                }
            } else {
                span {
                    class: "inline-block my-auto h-10 w-16 loading rounded mr-16",
                }
            }

            // Oil
            img {
                class: "inline-block my-auto w-10 h-10 ml-16 mr-4 rounded-full ",
                src: "https://as2.ftcdn.net/v2/jpg/01/18/15/67/1000_F_118156739_YaqIIHto5LeAsroscdOHwtKWqoWIipv6.jpg",
                // src: "https://img.freepik.com/premium-photo/oil-barrel_172429-567.jpg",
                // src: "https://img.pikbest.com/origin/09/28/78/29spIkbEsTgfS.png!sw800",
                // src: "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcTLDHpRcaduLw3RthgYM6MK0qizLaa6UrIwHQ&s",
                alt: "Crude oil barrel"
            }
            if let Some(Ok(quote)) = &*usdc_quote.value().read_unchecked() {
                span {
                    class: "inline-block my-auto mr-2",
                    "{quote.out_amount as f64 / 1_000_000.0 / 77.83:.2}"
                }
                span {
                    class: "inline-block my-auto mr-16 text-elements-lowEmphasis",
                    "barrels"
                }
            } else {
                span {
                    class: "inline-block my-auto h-10 w-16 loading rounded mr-16",
                }
            }
        }
    }
}

fn _LiquiditySection() -> Element {
    rsx! {
        Col {
            // class: "w-full mt-24 sm:mt-32 pt-0 pb-64 sm:pb-64 px-3 sm:px-8 md:px-16 lg:px-24 bg-[url('/assets/blurchart-1920.png')] bg-contain bg-no-repeat bg-center",
            class: "w-full mt-24 sm:mt-32 pt-0 pb-64 sm:pb-64 px-3 sm:px-8 md:px-16 lg:px-24 relative",
            // div {
            //     class: "absolute inset-0 overflow-hidden",
            //     svg {
            //         class: "w-full h-full",
            //         path {
            //             class: "stroke-purple-500 stroke-[3px] fill-none animate-draw",
            //             d: "M0,200 Q200,180 400,150 T800,100 T1200,50 T1600,0",
            //             filter: "url(#glow)"
            //         }
            //         defs {
            //             filter {
            //                 id: "glow",
            //                 feGaussianBlur {
            //                     stdDeviation: "4",
            //                     result: "coloredBlur"
            //                 }
            //                 feMerge {
            //                     feMergeNode {
            //                         in: "coloredBlur" 
            //                     }
            //                     feMergeNode {
            //                         in: "SourceGraphic"
            //                     }
            //                 }
            //             }
            //         }
            //     }
            // }
            Col {
                class: "gap-2 sm:gap-4 mx-auto w-full max-w-[96rem]",
                span {
                    class: "font-wide font-semibold text-2xl sm:text-3xl text-elements-highEmphasis text-center sm:text-left",
                    "Solving for liquidity."
                }
                span {
                    class: "max-w-xl sm:text-lg text-elements-lowEmphasis text-center sm:text-left",
                    "ORE works with issuers of novel defi tokens to provide market liquidity and connect with a community of likeminded investors."
                }
            }
        }
    }
}

fn _YieldSection() -> Element {
    rsx! {
        Col {
            class: "w-full mt-24 sm:mt-32 pt-0 pb-64 sm:pb-64 px-3 sm:px-8 md:px-16 lg:px-24",
            Col {
                class: "gap-2 sm:gap-4 mx-auto w-full max-w-[96rem]",
                span {
                    class: "font-wide font-semibold text-2xl sm:text-3xl text-elements-highEmphasis text-center sm:text-left",
                    "Earn productive yield."
                }
                span {
                    class: "max-w-xl sm:text-lg text-elements-lowEmphasis text-center sm:text-left",
                    "Unlike Bitcoin and metallic gold, ORE serves as a medium of exchange for defi spot markets and earns yield through onchain trading volume."
                }
            }
        }
    }
}


fn _SupplySection() -> Element {
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
                    "ORE follows a fixed supply curve and deflationary emissions schedule."
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


fn _MiningSection() -> Element {
    rsx! {
        div {
            class: "mt-24 sm:mt-40 pt-0 px-3 sm:px-8 md:px-16 lg:px-24",
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