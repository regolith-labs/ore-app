use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;

use crate::components::{Breadcrumbs, Col, OrePrice, Row, SwapForm};

#[component]
pub fn Market(market: String) -> Element {
    rsx! {
        Col {
            class: "w-full px-5 sm:px-8",
            gap: 2,
            Breadcrumbs {}
            Row {
                class: "justify-between",
                span {
                    class: "font-wide text-2xl font-semibold",
                    "{market}"
                }
                OrePrice {
                    ui_amount_string: "1.042",
                }
            }
            Row {
                gap: 4,
                PriceChart {}
                span {
                    class: "hidden lg:flex",
                    SwapForm {
                        mint_a: Pubkey::new_unique(),
                        mint_b: Pubkey::new_unique(),
                    }
                }
            }
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
}

fn PriceChart() -> Element {
    rsx! {
        // TODO
        div {
            class: "flex w-full h-80 bg-gray-800 rounded"
        }
    }
}
