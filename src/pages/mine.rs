use dioxus::prelude::*;

use crate::{
    components::*,
    hooks::{
        get_cutoff, get_updated_challenge, post_solution, use_gateway, use_member_db, use_miner,
        use_wallet, GetPubkey, Pool, POOLS,
    },
    route::Route,
};

pub fn Mine() -> Element {
    // on off button
    let mut is_gold = use_signal(|| false);
    // register with first pool
    let pool = POOLS.first().unwrap();
    let pool_url = &pool.url;
    // channel to and from miner
    let (from_miner, mut to_miner) = use_miner();
    // pool member account
    let member = use_member_db(pool_url.clone());
    // last challenge timestamp
    let mut last_hash_at = use_signal(|| 0);
    // user wallet
    let wallet = use_wallet();

    // restart miner
    use_effect(move || {
        if let true = *is_gold.read() {
            to_miner.restart();
        }
    });

    // next challenge resource
    let challenge = use_resource(move || {
        let gateway = use_gateway();
        let pool_url = pool_url.clone();
        let member = &*member.read();
        let member = member.clone();
        let last_hash_at = *last_hash_at.read();
        async move {
            if let Some(Ok(member)) = member {
                get_updated_challenge(
                    &gateway.http,
                    pool_url.as_str(),
                    member.authority.as_str(),
                    last_hash_at,
                )
                .await
            } else {
                Err(crate::gateway::GatewayError::AccountNotFound)
            }
        }
    });

    // challenge sender
    use_effect(move || {
        let is_gold = *is_gold.read();
        let member = &*member.read();
        let member = member.clone();
        let challenge = *challenge.read();
        if let (Some(Ok(member)), Some(Ok(challenge)), true) = (member, challenge, is_gold) {
            spawn(async move {
                let gateway = use_gateway();
                let cutoff_time =
                    get_cutoff(&gateway.rpc, challenge.challenge.lash_hash_at, 5).await;
                match cutoff_time {
                    Ok(cutoff_time) => {
                        to_miner.send(ore_miner_web::InputMessage {
                            member,
                            challenge,
                            cutoff_time,
                        });
                    }
                    Err(err) => {
                        log::error!("{:?}", err);
                    }
                }
            });
        }
    });

    // solutions receiver
    use_effect(move || {
        let pubkey = wallet.get_pubkey();
        let from_miner_read = &*from_miner.read();
        if let ore_miner_web::OutputMessage::Solution(solution) = from_miner_read {
            let gateway = use_gateway();
            let solution = solution.clone();
            log::info!("solution received: {:?}", solution);
            if let Ok(pubkey) = pubkey {
                spawn(async move {
                    let _ = post_solution(&gateway.http, pool_url, &pubkey, &solution).await;
                });
            }
        }
        if let ore_miner_web::OutputMessage::Expired(lha) = from_miner_read {
            log::info!("expired: {}", lha);
            // there may be many workers with the same lha observation
            // only update on the first expiration
            let peek = *last_hash_at.peek();
            if lha > &peek {
                log::info!("updating lha: {:?}:{:?}", peek, lha);
                last_hash_at.set(*lha);
            }
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
            Miner { is_gold, member_db: member,pool: pool.clone() }
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
