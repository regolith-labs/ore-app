use dioxus::prelude::*;

use crate::{components::*, hooks::use_boosts, route::Route};

pub fn Stake() -> Element {
    rsx! {
        Col {
            class: "w-full h-full pb-20 sm:pb-16",
            gap: 4,
            Heading {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                title: "Stake",
                subtitle: "Earn yield by providing liquidity for traders."
            }
            StakeTable {}
            // StakeForm {
            //     class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
            //     mint: Pubkey::new_unique(),
            // }
        }
    }
}

fn StakeTable() -> Element {
    let boosts = use_boosts();
    rsx! {
        Table {
            header: rsx! {
                TableHeader {
                    left: "Pair",
                    right_1: "Multiplier",
                    right_2: "Deposits",
                    right_3: "Yield",
                }
            },
            rows: rsx! {
                if let Some(Ok(boosts)) = boosts.cloned() {
                    for boost in boosts {
                        TableRowLink {
                            to: Route::Landing {},
                            left: rsx! {
                                span {
                                    class: "w-min sm:w-56",
                                    "{boost.mint.to_string()[..8].to_string()}"
                                }
                            },
                            right_1: rsx! {
                                span {
                                    class: "text-right w-40 my-auto",
                                    "{boost.multiplier as f64 / 1000f64}x"
                                }
                            },
                            right_2: rsx! {
                                span {
                                    class: "text-right w-40 my-auto",
                                    "Y"
                                }
                            },
                            right_3: rsx! {
                                span {
                                    class: "text-right w-40 my-auto",
                                    "Z"
                                }
                            },
                        }
                    }
                } else {
                    "None"
                }
            }
        }
    }
}
