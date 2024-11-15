use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;

use crate::components::{Breadcrumbs, Col, OrePrice, OreValueSmall, Row, SwapForm};

#[component]
pub fn Market(market: String) -> Element {
    rsx! {
        Row {
            class: "w-full px-5 sm:px-8",
            gap:4,
            Col {
                class: "w-full",
                gap: 4,
                Col {
                    gap: 2,
                    // Breadcrumbs {}
                    Row {
                        class: "justify-between",
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
                PriceChart {}
                Actions {}
                Stats {}
            }
            span {
                class: "hidden lg:flex",
                SwapForm {
                    mint_a: Pubkey::new_unique(),
                    mint_b: Pubkey::new_unique(),
                }
            }
        }
    }
}

fn Actions() -> Element {
    rsx! {
        Row {
            class: "justify-end lg:hidden",
            button {
                class: "flex controls-primary h-10 rounded-full w-full sm:w-40",
                span {
                    class: "mx-auto my-auto",
                    "Swap"
                }
            }
        }
    }
}

fn Stats() -> Element {
    rsx! {
        Col {
            gap: 1,
            Row {
                gap: 1,
                StatValue {
                    title: "Market cap",
                    value: 123
                }
                StatValue {
                    title: "Volume",
                    value: 123
                }
            }
            Row {
                gap: 1,
                StatValue {
                    title: "Total supply",
                    value: 123
                }
                StatValue {
                    title: "FDV",
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
                class: "font-medium my-auto text-sm text-elements-lowEmphasis",
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
        // TODO
        Col {
            gap: 2,
            Row {
                class: "text-sm text-elements-midEmphasis ml-auto",
                gap: 1,
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
                class: "flex w-full h-80 bg-gray-800 rounded"
            }
        }
    }
}

#[component]
fn TimeFrameButton(title: String) -> Element {
    rsx! {
        button {
            class: "py-1 w-10 rounded text-elements-midEmphasis",
            "{title}"
        }
    }
}
