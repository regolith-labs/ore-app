use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;

use crate::components::{Breadcrumbs, Swap};

#[component]
pub fn Asset(asset: String) -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-4 w-full px-5 sm:px-8",
            Breadcrumbs {}
            span {
                class: "font-wide text-2xl font-semibold",
                "{asset}"
            }
            div {
                class: "flex flex-row gap-4",
                div {
                    class: "flex w-full h-64 bg-gray-800 rounded"
                }
                span {
                    class: "hidden lg:flex",
                    Swap {
                        mint_a: Pubkey::new_unique(),
                        mint_b: Pubkey::new_unique(),
                    }
                }
            }
        }
    }
}
