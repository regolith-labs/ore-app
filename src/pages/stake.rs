use dioxus::prelude::*;

use crate::components::*;

pub fn Stake() -> Element {
    rsx! {
        Col {
            class: "w-full h-full pb-20 sm:pb-16",
            gap: 8,
            Heading {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                title: "Stake",
                subtitle: "Provide liquidity for traders and earn yield."
            }
            StakeTable {}
        }
    }
}

fn AccountSummary() -> Element {
    rsx! {

    }
}

fn _YieldOverview() -> Element {
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
