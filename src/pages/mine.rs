use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;

use crate::{
    components::{
        CircleStackIcon, Col, OreValue, OreValueSmall, PlayIcon, Row, Table, TableHeader,
        TableRowLink,
    },
    hooks::use_ore_balance,
    route::Route,
};

pub fn Mine() -> Element {
    rsx! {
        Col {
            class: "w-full pb-20 sm:pb-16 gap-8",
            gap: 8,
            Row {
                class: "justify-between sm:hidden mx-5 sm:mx-8 h-10",
                gap: 4,
                span {
                    class: "font-wide text-2xl sm:text-3xl font-semibold align-text-bottom my-auto",
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
            to: Route::Swap {},
            class: "h-10 controls-primary rounded-full px-4 gap-2",
            PlayIcon {
                class: "h-5 w-5 mx-auto my-auto"
            }
            span {
                class: "my-auto",
                "Start"
            }
        }
    }
}

fn ClaimButton() -> Element {
    rsx! {
        Link {
            to: Route::Pay {},
            class: "h-10 controls-secondary rounded-full px-4 gap-2",
            CircleStackIcon {
                class: "h-5 w-5 mx-auto my-auto"
            }
            span {
                class: "my-auto",
                "Claim"
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
                left_width: 40,
                right: vec!["Hashpower".to_string(), "Multiplier".to_string(), "Yield".to_string()]
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
            left_width: 40,
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
                    span {
                        "64480"
                    }
                },
                rsx! {
                    span {
                        "2.4x",
                    }
                },
                rsx! {
                    span {
                        class: "text-elements-gold",
                        OreValueSmall {
                            ui_amount_string: "2.054"
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
