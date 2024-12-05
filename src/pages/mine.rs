use dioxus::prelude::*;

use crate::{
    hooks::{POOLS, Pool},
    components::*,
    route::Route,
};

pub fn Mine() -> Element {
    let mut is_gold = use_signal(|| false);

    rsx! {
        Col {
            class: "w-full h-full pb-20 sm:pb-16 max-w-2xl mx-auto px-5 sm:px-8",
            gap: 4,
            Heading {
                class: "w-full",
                title: "Mine",
                subtitle: "Forge new ORE by mining with your phone or computer."
            }
            button {
                class: "w-fit",
                onclick: move |_| is_gold.set(!is_gold.cloned()),
                Orb {
                    is_gold: *is_gold.read()
                }
            }
        }
    }
}

fn PoolTable() -> Element {
    rsx! {
        Col {
            gap: 2, 
            Table {
                header: rsx! {
                    TableHeader {
                        left: "Pool",
                        right_1: "Hashpower",
                        right_2: "Multiplier",
                        right_3: "Yield",
                    }
                },
                rows: rsx! {
                    for pool in POOLS.iter() {
                        PoolRow { pool: pool.clone() }
                    }
                }
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
                        class: "w-10 h-10 my-auto bg-gray-900 rounded",
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
            right_1: rsx! {
                span {
                    "64480"
                }
            },
            right_2: rsx! {
                span {
                    "2.4x",
                }
            },
            right_3: rsx! {
                span {
                    class: "text-elements-gold",
                    OreValueSmall {
                        ui_amount_string: "2.054"
                    }
                }
            },
        }
    }
}
