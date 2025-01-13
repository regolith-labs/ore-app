use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;

use crate::components::*;

pub fn Stake() -> Element {
    rsx! {
        Col {
            class: "w-full h-full pb-20 sm:pb-16",
            gap: 4,
            Heading {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                title: "Stake",
                subtitle: "Earn yield by providing market liquidity for traders."
            }
            StakeForm {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                mint: Pubkey::new_unique(),
            }
        }
    }
}
