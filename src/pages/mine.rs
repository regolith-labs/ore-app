use dioxus::prelude::*;

use crate::{
    components::*,
    hooks::{
        get_updated_challenge, post_solution, use_gateway, use_member_db, use_miner, use_wallet,
        GetPubkey, Pool, POOLS,
    },
    route::Route,
};

pub fn Mine() -> Element {
    // register with first pool
    let pool = POOLS.first().unwrap();
    let pool_url = &pool.url;

    let mut is_gold = use_signal(|| false);

    let (from_miner, to_miner) = use_miner();

    let member = use_member_db(pool_url.clone());

    let mut last_hash_at = use_signal(|| 0);

    let wallet = use_wallet();

    let challenge = use_resource(move || {
        let gateway = use_gateway();
        let pool_url = pool_url.clone();
        let pubkey = wallet.get_pubkey();
        let last_hash_at = *last_hash_at.read();
        async move {
            let pubkey = pubkey?;
            get_updated_challenge(&gateway.http, pool_url, pubkey, last_hash_at).await
        }
    });

    use_effect(move || {
        let member = &*member.read();
        let challenge = &*challenge.read();
        if let (Some(Ok(member)), Some(Ok(challenge))) = (member, challenge) {
            to_miner.send(ore_miner_web::InputMessage {
                member: member.clone(),
                challenge: *challenge,
                cutoff_time: 0,
            });
        }
    });

    use_effect(move || {
        let gateway = use_gateway();
        let pubkey = wallet.get_pubkey();
        let from_miner_read = &*from_miner.read();
        if let ore_miner_web::OutputMessage::Solution(solution) = from_miner_read {
            let solution = solution.clone();
            log::info!("solution received: {:?}", solution);
            if let Ok(pubkey) = pubkey {
                spawn(async move {
                    post_solution(&gateway.http, pool_url, &pubkey, &solution).await;
                });
            }
        }
        if let ore_miner_web::OutputMessage::Expired(lha) = from_miner_read {
            log::info!("expired: {}", lha);
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
                subtitle: "Forge new ORE by mining with your phone or computer."
            }
            button {
                class: "w-fit",
                onclick: move |_| is_gold.set(!is_gold.cloned()),
                Orb { is_gold: *is_gold.read() }
            }
            Miner { is_gold, pool: pool.clone() }
            div { "{last_hash_at()}" }
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
