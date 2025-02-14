use dioxus::prelude::*;
use crate::{
    components::*,         
    hooks::{use_miner_events, MiningEvent}
};
use chrono::{DateTime, Local};
use solana_sdk::signature::Signature;
use ore_api::consts::TOKEN_DECIMALS;
use solana_extra_wasm::program::spl_token::amount_to_ui_amount_string;

pub fn MineTable() -> Element {
    let miner_events = use_miner_events();    
    rsx! {
        Col {
            gap: 8,
            Subheading {
                class: "px-5 sm:px-8",
                title: "Mining Log"
            }
            Table {
                class: "mx-0 sm:mx-8",
                header: rsx! {
                    TableHeader {
                        left: "Time",
                        right_1: "Transaction",
                        right_2: "Score",
                        right_3: "Reward",
                    }
                },
                rows: rsx! {
                    {
                        let events_guard = miner_events.read();
                        let events = events_guard.iter().collect::<Vec<_>>();
                        if events.is_empty() {
                            rsx! {
                                tr {
                                    td {
                                        colspan: "4",
                                        class: "flex flex-row justify-start pt-16 text-elements-lowEmphasis px-5 sm:px-3",
                                        "No activity yet"
                                    }
                                }
                            }
                        } else {
                            rsx! {
                                for event in events {
                                    MineTableRow {
                                        event: event.clone()
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn MineTableRow(
    event: MiningEvent
) -> Element {
    rsx! {
        TableRowExternalLink {
            to: format!("https://solscan.io/tx/{}", event.signature),
            left: rsx! {
                MineTableRowTitle {
                    timestamp: event.timestamp
                }
            },
            right_1: rsx! {
                MineTableRowTx {
                    signature: event.signature
                }
            },
            right_2: rsx! {
                MineTableRowScore {
                    pool_score: event.difficulty,
                    member_score: event.member_difficulty
                }
            },
            right_3: rsx! {
                MineTableRowYield {
                    net_reward: event.net_reward,
                    member_reward: event.member_reward
                }
            },
        }
    }
}

#[component]
fn MineTableRowTitle(
    timestamp: u64
) -> Element {    
    let datetime = DateTime::from_timestamp(timestamp as i64, 0)
        .unwrap()
        .with_timezone(&Local);    
    let date = datetime.format("%b %d");
    let time = datetime.format("%I:%M %p");
    rsx! {
        Col {
            span {
                class: "font-medium my-auto",
                "{date}"
            }            
            span {
                class: "text-xs text-elements-lowEmphasis",
                "{time}"
            }                        
        }
    }
}

#[component]
fn MineTableRowTx(
    signature: Signature
) -> Element {
    let len = signature.to_string().len();
    let first_four = &signature.to_string()[0..4];
    let last_four = &signature.to_string()[len - 4..len];
    rsx! {
        Col {
            span {
                class: "text-right my-auto font-medium",
                "{first_four}...{last_four}"
            }                
        }
    }
}

#[component]
fn MineTableRowScore(    
    pool_score: u64,
    member_score: u64
) -> Element {    
    rsx! {
        Col {
            span {
                class: "font-medium my-auto",
                "{pool_score}"
            }            
            span {
                class: "text-xs text-elements-lowEmphasis",
                "{member_score}"
            }                        
        }
    }
}

#[component]
fn MineTableRowYield(
    net_reward: u64,
    member_reward: u64
) -> Element {
    rsx! {
        Col {
            OreValue {
                class: "text-right ml-auto font-medium",
                ui_amount_string: amount_to_ui_amount_string(net_reward, TOKEN_DECIMALS),
                with_decimal_units: true,
                size: TokenValueSize::Small,
                gold: false,
                abbreviated: true,
            }
            OreValue {
                class: "text-right ml-auto",
                ui_amount_string: amount_to_ui_amount_string(member_reward, TOKEN_DECIMALS),
                with_decimal_units: true,
                size: TokenValueSize::XSmall,
                gold: true,
                abbreviated: true,
            }
        }
        
    }
}