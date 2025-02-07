use dioxus::prelude::*;

use crate::{
    components::*, config::{Pool, LISTED_POOLS}, 
    gateway::GatewayResult, 
    hooks::{on_transaction_done, use_member, use_member_record, use_miner, use_miner_claim_transaction, use_miner_is_active, use_pool, use_wallet, IsActiveMiner}, 
    route::Route
};


pub fn Mine() -> Element {
    let wallet = use_wallet();

    // on off button
    let is_active: Signal<IsActiveMiner> = use_miner_is_active();

    // register with first pool
    // let pool = FIRST_POOL;
    let pool = use_pool();
    let member_record = use_member_record(pool);    
    let member = use_member(pool);
    let claim_tx = use_miner_claim_transaction(member.clone());
        
    // TODO: rendering lash-hash-at here
    // to demonstrate that we can read messages from the miner
    let (from_miner, _to_miner) = use_miner();
    let mut last_hash_at: Signal<i64> = use_signal(|| 0);
    use_effect(move || {
        // TODO: fetch current miner state (claim amount, etc)
        let from_miner_read = &*from_miner.read();
        if let ore_miner_types::OutputMessage::Expired(lha) = from_miner_read {
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
                subtitle: "Utilize spare hashpower to harvest ORE."
            }
            StopStartButton { is_active }
            // MinerStatus { member_record: member_record, pool: pool.clone() }
            // if let Some(Ok(member)) = member_onchain.cloned() {
            //     // use member
            // }
            MinerData {claim_tx: claim_tx.clone(), member: member.clone() }
            // TODO: Add activity table
            // div { "{last_hash_at}" }   
        }
    }
}

#[component]
fn MinerData(claim_tx: Resource<Result<solana_sdk::transaction::VersionedTransaction, crate::gateway::GatewayError>>, member: Resource<GatewayResult<ore_pool_api::state::Member>> ) -> Element {    
    on_transaction_done(move |_sig| {
        claim_tx.restart();        
    });
    
    rsx! {
        Row {
            class: "w-full flex-wrap sm:flex-col rounded-xl mx-auto justify-between py-5",
            gap: 2,            
            Col {
                // class: "min-w-56",
                gap: 4,
                span {
                    class: "text-elements-lowEmphasis font-medium",
                    "Hash Power"
                }
                span {
                    class: "font-semibold text-2xl sm:text-3xl",
                    "1230.12"
                }
            }
            Col {
                // class: "min-w-56",
                gap: 4,
                span {
                    class: "text-elements-lowEmphasis font-medium",
                    "Claimable Yield"
                }
                OreValue {
                    size: TokenValueSize::Large,
                    ui_amount_string: if let Some(Ok(member)) = member.cloned() {
                        member.balance.to_string()
                    } else {
                        "0".to_string()
                    },
                }
            }
            Col {
                class: "justify-end min-w-56",
                ClaimButton {
                    transaction: claim_tx.clone(),
                }                
            }
        }
    }
}


// TODO: remove 
// fn ActionButtons() -> Element {
//     rsx! {1
//         Row {
//             class: "mx-auto w-full mt-8",
//             ClaimButton {}
//         }
//     }
// }

// TODO: remove 
// fn ClaimButton() -> Element {
//     rsx! {
//         // replace link with claim logic
//         Link {
//             to: Route::Landing {},
//             class: "flex flex-row h-10 w-min controls-gold rounded-full px-4 gap-2",
//             span {
//                 class: "my-auto text-nowrap",
//                 "Claim Yield"
//             }
//         }
//     }
// }

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
            // cloning to get the value
            if !is_active.cloned().0 {
                span {
                    class: "flex flex-row gap-2 my-auto mx-auto bg-white px-4 h-12 text-black rounded-full font-semibold z-10 group-hover:scale-105 transition-transform",
                    PlayIcon { class: "my-auto h-5" }
                    span {
                        class: "my-auto",
                        "Start mining"
                    }
                }
            } else {
                span {
                    class: "flex flex-row gap-2 my-auto mx-auto bg-gray-300 px-4 h-12 text-white rounded-full font-semibold z-10 group-hover:scale-105 transition-transform border border-white",
                    StopIcon { class: "my-auto h-5"  },
                    span {
                        class: "my-auto",
                        "Stop mining"
                    }
                }
            }
        }
    }
}

fn _PoolTable() -> Element {
    rsx! {
        Col { gap: 2,
            Table {
                header: rsx! {
                    TableHeader { left: "Pool", right_1: "Hashpower", right_2: "Multiplier", right_3: "Yield" }
                },
                rows: rsx! {
                    for pool in LISTED_POOLS.iter() {
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
            to: Route::Landing {},
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
                OreValue {
                    ui_amount_string: "2.054".to_string(),
                    with_decimal_units: true,
                    size: TokenValueSize::Small,
                    gold: true,
                }
            }
        }
    }
}
