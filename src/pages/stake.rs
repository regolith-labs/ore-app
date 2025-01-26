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
                subtitle: "Provide market liquidity and earn yield."
            }
            // YieldOverview {}
            VaultPreview {}
            StakeTable {}
        }
    }
}

fn VaultPreview() -> Element {
    rsx! {
        Link {
            class: "flex flex-col sm:flex-row gap-4 sm:justify-between rounded px-8 py-8 bg-surface-elevated border-elevated",
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

fn YieldOverview() -> Element {
    // TODO Get all stake accounts
    // TODO Calculate total claimable yield
    // TODO Provide claim button
    rsx! {
        Row {
            class: "mx-5 sm:mx-8 py-8 justify-between",
            // div {
            //     class: "flex w-full",
            //     OreValue {
            //         class: "mx-auto my-auto",
            //         ui_amount_string: "2.324330".to_string(),
            //     }
            // }
            div {
                class: "flex w-full",
                span {
                    class: "text-elements-midEmphasis font-bold text-2xl sm:text-3xl my-auto mx-auto",
                    "0.04%"
                }   
            }
            div {
                class: "flex w-full",
                OreValueGold {
                    class: "mx-auto my-auto",
                    ui_amount_string: "2.324330".to_string(),
                }   
            }
        }
    }
}