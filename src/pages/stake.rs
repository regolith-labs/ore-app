use dioxus::prelude::*;

use crate::{components::*, route::Route};

pub fn Stake() -> Element {
    rsx! {
        Col {
            class: "w-full h-full pb-20 sm:pb-16",
            gap: 8,
            Heading {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                title: "Stake",
                subtitle: "Earn yield by providing liquidity for traders."
            }
            VaultPreview {}
            StakeTable {}
            // StakeForm {
            //     class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
            //     mint: Pubkey::new_unique(),
            // }
        }
    }
}

fn VaultPreview() -> Element {
    rsx! {
        Link {
            class: "flex flex-row justify-between rounded px-8 py-8 bg-surface-elevated",
            to: Route::Vault {},
            Col {
                gap: 2,
                span {
                    class: "font-semibold text-2xl",
                    "Vault"
                }
                span {
                    class: "text-elements-midEmphasis",
                    "Stake unpaired ORE to earn the idle yield rate."
                }
            }
            div {
                class: "flex bg-white my-auto px-4 py-2 rounded-full",
                span {
                    class: "mx-auto my-auto font-semibold text-black",
                    "Stake now"
                }
            }
        }
    }
}

fn StakeOverview() -> Element {
    // TODO Get all stake accounts
    // TODO Calculate total claimable yield
    // TODO Provide claim button
    rsx! {}
}