use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;

use crate::{
    components::{CircleStackIcon, Col, OreValue, PlayIcon, Row, Table, TableHeader, TableRowLink},
    hooks::use_ore_balance,
    route::Route,
};

pub fn Mine() -> Element {
    rsx! {
        Col {
            class: "w-full pb-20 sm:pb-16",
            gap: 8,
            Row {
                class: "justify-between sm:hidden mx-5 sm:mx-8 h-10 font-wide text-2xl font-semibold",
                span {
                    class: "my-auto",
                    "Mine"
                }
                StartButton {}
            }
            MiningYield {}
            PoolTable {}
        }
    }
}

fn MiningYield() -> Element {
    let balance = use_ore_balance();
    rsx! {
        Col {
            class: "sm:gap-4 px-5 sm:px-8",
            gap: 2,
            span {
                class: "font-medium text-xs sm:text-sm text-gray-700",
                "Yield"
            }
            Row {
                class: "justify-between align-top",
                match balance.cloned() {
                    None => {
                        rsx! {
                            span {
                                class: "h-10 w-64 loading rounded"
                            }
                        }
                    }
                    Some(_balance) => {
                        rsx! {
                            OreValue {
                                ui_amount_string: "0.000"
                            }
                        }
                    }
                }
                Row {
                    gap: 4,
                    ClaimButton {}
                    span {
                        class: "hidden sm:flex",
                        StartButton {}
                    }
                }
            }
        }
    }
}

fn StartButton() -> Element {
    rsx! {
        Link {
            to: Route::Pay {},
            class: "controls-square controls-primary",
            PlayIcon {
                class: "h-5 w-5 mx-auto my-auto"
            }
        }
    }
}

fn ClaimButton() -> Element {
    rsx! {
        Link {
            to: Route::Pay {},
            class: "controls-square controls-secondary",
            CircleStackIcon {
                class: "h-5 w-5 mx-auto my-auto"
            }
        }
    }
}

fn PoolTable() -> Element {
    // TODO Read from config file
    let listed_pools = vec![
        Pool {
            address: Pubkey::new_unique(),
            name: "Ec1ipse".to_owned(),
            description: "".to_owned(),
            image: "https://pbs.twimg.com/profile_images/1736827532312211456/V0bvyS_2_400x400.jpg"
                .to_owned(),
        },
        Pool {
            address: Pubkey::new_unique(),
            name: "Rush".to_owned(),
            description: "".to_owned(),
            image: "https://pbs.twimg.com/profile_images/1825694276929368064/GJcGr3rR_400x400.jpg"
                .to_owned(),
        },
    ];

    rsx! {
        Table {
            TableHeader {
                left: "Pool",
                right: vec!["Hashpower".to_string(), "Multiplier".to_string()]
            }
            for pool in listed_pools {
                PoolRow { pool: pool }
            }
        }
    }
}

#[component]
fn PoolRow(pool: Pool) -> Element {
    rsx! {
        TableRowLink {
            to: Route::Pool { pool: pool.name.clone() },
            left: rsx! {
                Row {
                    gap: 4,
                    img {
                        class: "w-10 h-10 my-auto bg-gray-900 rounded border border-gray-800",
                        src: "{pool.image}"
                    }
                    Col {
                        class: "my-auto",
                        span {
                            class: "font-medium",
                            "{pool.name}"
                        }
                    }
                }
            },
            right: vec![
                rsx! {
                    Row {
                        class: "text-right my-auto",
                        span {
                            class: "flex w-28 sm:w-40 justify-end",
                            "64480"
                        }
                        span {
                            class: "w-28 sm:w-40",
                            "2.4x",
                        }
                    }
                }
            ]
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Pool {
    address: Pubkey,
    name: String,
    description: String,
    image: String,
}
