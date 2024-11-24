use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;

use crate::components::{BackButton, Col, SwapForm};

pub fn Swap() -> Element {
    rsx! {
        div {
            class: "flex w-full",
            Col {
                class: "w-96 mx-auto my-auto",
                gap: 4,
                Col {
                    gap: 2,
                    BackButton {}
                    span {
                        class: "font-wide text-2xl sm:text-3xl font-semibold",
                        "Swap"
                    }
                    span {
                        class: "text-elements-lowEmphasis",
                        "Sell one token and buy another."
                    }
                }
                SwapForm {
                    mint_a: Pubkey::new_unique(),
                    mint_b: Pubkey::new_unique(),
                }
            }
        }
    }
}
