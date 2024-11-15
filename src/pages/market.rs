use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;

use crate::{
    components::{Col, OrePrice, OreValueSmall, Row, SwapForm, XIcon},
    route::Route,
};

#[component]
pub fn Market(market: String) -> Element {
    rsx! {
        Row {
            class: "w-full px-5 sm:px-8 pb-20 sm:pb-16",
            gap:4,
            Col {
                class: "w-full",
                gap: 4,
                Header {
                    market: market
                }
                PriceChart {}
                Actions {}
                Stats {}
            }
            span {
                class: "hidden lg:flex mt-24",
                SwapForm {
                    mint_a: Pubkey::new_unique(),
                    mint_b: Pubkey::new_unique(),
                }
            }
        }
    }
}

#[component]
fn Header(market: String) -> Element {
    rsx! {
        Row {
            class: "justify-between h-10",
            Row {
                gap: 2,
                img {
                    class: "w-10 h-10 rounded-full border border-gray-800 -ml-1 my-auto",
                    src: "https://upload.wikimedia.org/wikipedia/en/b/b9/Solana_logo.png",
                }
                span {
                    class: "font-wide text-2xl sm:text-3xl font-semibold my-auto",
                    "{market}"
                }
            }
            Row {
                gap: 2,
                OrePrice {
                    ui_amount_string: "1.042",
                }
                span {
                    class: "font-medium text-green-500 text-sm mt-auto mb-2 sm:mb-1.5",
                    "0.2%"
                }
            }
        }
    }
}

fn Actions() -> Element {
    rsx! {
        div {
            class: "flex flex-col-reverse sm:flex-row gap-4",
            Info {}
            Row {
                class: "lg:hidden",
                gap: 2,
                SocialButtons {}
                SwapButton {}
            }
        }
    }
}

fn SwapButton() -> Element {
    rsx! {
        Link {
            to: Route::Swap {},
            class: "flex controls-primary h-10 rounded-full w-full sm:w-40",
            span {
                class: "mx-auto my-auto",
                "Swap"
            }
        }
    }
}

fn SocialButtons() -> Element {
    rsx! {
        Link {
            to: Route::Swap {},
            class: "flex controls-secondary h-10 w-10 rounded-full shrink-0",
            XIcon {
                class: "w-5 h-5 mx-auto my-auto"
            }
        }
    }
}

fn Info() -> Element {
    rsx! {
        Col {
            gap: 1,
            class: "px-1",
            span {
                class: "text-xs sm:text-sm font-medium text-elements-lowEmphasis",
                "About"
            }
            span {
                class: "text-elements-midEmphasis w-full",
                "Solana is a highly functional open source project that banks on blockchain technology’s permissionless nature to provide decentralized finance (DeFi) solutions."
            }
        }
    }
}

fn Stats() -> Element {
    rsx! {
        Col {
            gap: 1,
            div {
                class: "flex flex-col sm:flex-row gap-1",
                StatValue {
                    title: "SUPPLY",
                    value: 123
                }
                StatValue {
                    title: "LIQUIDITY",
                    value: 123
                }
            }
            div {
                class: "flex flex-col sm:flex-row gap-1",
                StatValue {
                    title: "VOLUME",
                    value: 123
                }
                StatValue {
                    title: "VALUATION",
                    value: 123
                }
            }
        }
    }
}

#[component]
fn StatValue(title: String, value: u64) -> Element {
    rsx! {
        Row {
            class: "justify-between elevated h-12 w-full px-4 rounded-sm",
            span {
                class: "font-medium my-auto text-xs sm:text-sm text-elements-lowEmphasis",
                "{title}"
            }
            span {
                class: "my-auto text-elements-midEmphasis",
                OreValueSmall {
                    ui_amount_string: "1.202"
                }
            }
        }
    }
}

fn PriceChart() -> Element {
    rsx! {
        Col {
            class: "gap-3",
            // gap: 3,
            Row {
                class: "text-sm text-elements-midEmphasis",
                TimeFrameButton {
                    title: "1D"
                }
                TimeFrameButton {
                    title: "1W"
                }
                TimeFrameButton {
                    title: "1M"
                }
                TimeFrameButton {
                    title: "1Y"
                }
                TimeFrameButton {
                    title: "All"
                }
            }
            div {
                class: "flex w-full h-80 elevated rounded",
                span {
                    class: "text-xs mx-auto my-auto",
                    "Chart goes here"
                }
            }
        }
    }
}

#[component]
fn TimeFrameButton(title: String) -> Element {
    rsx! {
        button {
            class: "py-1 w-10 rounded text-center transition text-elements-lowEmphasis hover:bg-controls-tertiaryHover hover:text-elements-midEmphasis",
            "{title}"
        }
    }
}
