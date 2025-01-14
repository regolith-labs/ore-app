use dioxus::prelude::*;

use crate::{
    components::*,
    hooks::{
        use_member_db, use_miner, use_miner_is_active, use_wallet, GetPubkey, IsActiveMiner, Pool,
        FIRST_POOL, POOLS,
    },
    route::Route,
};

pub fn Mine() -> Element {
    let wallet = use_wallet();

    // on off button
    let is_active = use_miner_is_active();

    // register with first pool
    let pool = FIRST_POOL;
    let pool_url = &pool.url;
    let member = use_member_db(pool_url.clone());

    // read from miner (as demonstration)
    let (from_miner, _to_miner) = use_miner();
    let mut last_hash_at = use_signal(|| 0);
    use_effect(move || {
        let _pubkey = wallet.get_pubkey();
        let from_miner_read = &*from_miner.read();
        if let ore_miner_web::OutputMessage::Expired(lha) = from_miner_read {
            last_hash_at.set(*lha);
        }
    });

    rsx! {
        Col {
            class: "w-full h-full pb-20 sm:pb-16 max-w-2xl mx-auto px-5 sm:px-8",
            gap: 4,
            Heading {
                class: "w-full",
                title: "Mine",
                subtitle: "Utilize excess compute power to harvest ORE."
            }
            StopStartButton { is_active }
            MinerStatus { member_db: member, pool: pool.clone() }
            div { "{last_hash_at}" }
        }
    }
}

#[component]
fn StopStartButton(is_active: Signal<IsActiveMiner>) -> Element {
    rsx! {
        button {
            class: "relative flex w-[16rem] h-[16rem] mx-auto my-8 sm:my-16 group",
            onclick: move |_| is_active.set(IsActiveMiner(!is_active.cloned().0)),
            OrbMiner {
                class: "absolute top-0 left-0 z-0",
                gold: is_active.read().0
            }
            if !is_active.cloned().0 {
                span {
                    class: "flex flex-row gap-2 my-auto mx-auto bg-white px-4 h-12 text-black rounded-full font-semibold z-10 group-hover:ring group-hover:ring-offset-2",
                    PlayIcon { class: "my-auto h-5" }
                    span {
                        class: "my-auto",
                        "Start mining"
                    }
                }
            }
        }
    }
}

fn PoolTable() -> Element {
    rsx! {
        Col { gap: 2,
            Table {
                header: rsx! {
                    TableHeader { left: "Pool", right_1: "Hashpower", right_2: "Multiplier", right_3: "Yield" }
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
            to: Route::Pool {
                pool: pool.name.clone(),
            },
            left: rsx! {
                Row { gap: 4,
                    img {
                        class: "w-10 h-10 my-auto bg-gray-900 rounded",
                        src: "{pool.image}"
                    }
                    Col { class: "my-auto",
                        span { class: "font-medium", "{pool.name}" }
                    }
                }
            },
            right_1: rsx! {
                span { "64480" }
            },
            right_2: rsx! {
                span { "2.4x" }
            },
            right_3: rsx! {
                span { class: "text-elements-gold",
                    OreValueSmall { ui_amount_string: "2.054" }
                }
            }
        }
    }
}
