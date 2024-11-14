use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;

use crate::components::{PageTitle, SwapTool};

pub fn Swap() -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-8 w-full",
            PageTitle {
                title: "Swap"
            }
            span {
                class: "w-96 mx-auto my-auto",
                SwapTool {
                    mint_a: Pubkey::new_unique(),
                    mint_b: Pubkey::new_unique(),
                }
            }
        }
    }
}
