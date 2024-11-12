use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;

use crate::{
    components::{CircleStackIcon, OreValue, OreValueSmall, PlayIcon},
    hooks::use_ore_balance,
    route::Route,
};

pub fn Mine() -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-8 w-screen",
            span {
                class: "flex flex-row justify-between sm:hidden mx-5 sm:mx-8 h-10 font-wide text-2xl font-semibold",
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
        div {
            class: "flex flex-col gap-2 sm:gap-4 px-5 sm:px-8",
            span {
                class: "font-medium text-xs sm:text-sm text-gray-700",
                "Yield"
            }
            div {
                class: "flex flex-row justify-between align-top",
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
                div {
                    class: "flex flex-row gap-4",
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
                class: "h-6 w-6 mx-auto my-auto"
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
        div {
            class: "flex flex-col sm:mx-5",
            PoolTableHeader {}
            for pool in listed_pools {
                PoolRow {
                    pool: pool
                }
            }
        }
    }
}

fn PoolTableHeader() -> Element {
    rsx! {
        div {
            class: "flex flex-row h-8 h-10 px-5 sm:px-3 justify-between font-medium text-xs sm:text-sm text-gray-700",
            span {
                class: "my-auto",
                "Pool"
            }
            div {
                class: "flex flex-row text-right",
                span {
                    class: "my-auto w-28 sm:w-40",
                    "Boost"
                }
                span {
                    class: "my-auto w-28 sm:w-40",
                    "Hashpower"
                }
            }
        }
    }
}

#[component]
fn PoolRow(pool: Pool) -> Element {
    rsx! {
        Link {
            to: Route::Pool { pool: pool.name.clone() },
            class: "flex flex-row w-full px-5 sm:px-3 py-4 justify-between transition sm:rounded-md hover:bg-gray-900 hover:cursor-pointer",
            div {
                class: "flex flex-row gap-4",
                img {
                    class: "w-10 h-10 my-auto bg-gray-900 rounded border border-gray-800",
                    src: "{pool.image}"
                }
                div {
                    class: "flex flex-col my-auto",
                    span {
                        class: "font-medium",
                        "{pool.name}"
                    }
                    // span {
                    //     class: "font-medium text-gray-700 h-5 text-sm",
                    //     match balance.cloned() {
                    //         None => rsx! {
                    //             div { class: "h-5 w-20 loading rounded"}
                    //         },
                    //         Some(balance) => {
                    //             match balance {
                    //                 Err(_) => rsx!{ "0.00" },
                    //                 Ok(b) => rsx!{ "{b.ui_amount_string}" },
                    //             }
                    //         }
                    //     }
                    // }
                }
            }
            div {
                class: "flex flex-row text-right my-auto",
                span {
                    class: "w-28 sm:w-40",
                    "2.4x",
                }
                span {
                    class: "flex w-28 sm:w-40 justify-end",
                    "64480"
                }
            }
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
