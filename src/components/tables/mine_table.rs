use crate::{
    components::*,
    hooks::{use_miner_events, MiningEvent},
};
use chrono::{DateTime, Local};
use dioxus::prelude::*;
use ore_api::consts::TOKEN_DECIMALS;
use solana_sdk::signature::Signature;

use crate::solana::spl_token::amount_to_ui_amount_string;

pub fn MineTable() -> Element {
    let miner_events = use_miner_events();
    let events_guard = miner_events.read();
    let events = events_guard.iter().collect::<Vec<_>>();
    let mut info_hidden = use_signal(|| true);
    rsx! {
        Col {
            class: "w-full",
            gap: 0,
            button {
                class: "flex flex-row gap-2 px-5 w-min sm:px-8 group hover:cursor-pointer",
                onclick: move |_| info_hidden.set(!info_hidden.cloned()),
                Subheading {
                    class: "my-auto",
                    title: "Activity"
                }
                InfoIcon {
                    class: "h-4 w-4 shrink-0 text-elements-lowEmphasis group-hover:text-elements-highEmphasis transition-all duration-300 ease-in-out my-auto",
                }
            }
            InfoText {
                class: "text-wrap px-5 sm:px-8",
                text: "This table displays your recent mining activity from the current session.",
                hidden: info_hidden,
            }
            if events.is_empty() {
                Row {
                    class: "text-elements-lowEmphasis font-medium w-full min-w-max mt-4 sm:px-8 px-5",
                    span {
                        "No activity yet"
                    }
                }
            } else {
                Table {
                    class: "mt-4 mx-0 sm:mx-8",
                    header: rsx! {
                        TableHeader {
                            left: "Transaction",
                            right_1: "Time",
                            right_2: "Score",
                            right_3: "Reward",
                            help_left: "Recent transactions submitted by the mining pool.",
                            help_right_1: "Timestamp of the mining pool's submitted transaction.",
                            help_right_2: "Difficulty score of your best submitted solution.",
                            help_right_3: "Amount of ORE earned by your solution. 1 ORE = 10^11 grams",
                            help_hidden: info_hidden,
                        }
                    },
                    rows: rsx! {
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

#[component]
fn MineTableRow(event: MiningEvent) -> Element {
    rsx! {
        TableRowExternalLink {
            to: format!("https://solscan.io/tx/{}", event.signature),
            left: rsx! {
                MineTableRowTx {
                    signature: event.signature
                }
            },
            right_1: rsx! {
                MineTableRowDate {
                    timestamp: event.timestamp
                }
            },
            right_2: rsx! {
                MineTableRowScore {
                    pool_score: event.difficulty,
                    member_score: event.member_difficulty
                }
            },
            right_3: rsx! {
                MineTableRowReward {
                    net_reward: event.net_reward,
                    member_reward: event.member_reward
                }
            },
        }
    }
}

#[component]
fn MineTableRowDate(timestamp: u64) -> Element {
    let datetime = DateTime::from_timestamp(timestamp as i64, 0)
        .unwrap()
        .with_timezone(&Local);
    // let date = datetime.format("%b %d");
    let time = datetime.format("%I:%M %p");
    rsx! {
        Col {
            span {
                class: "font-medium my-auto",
                "{time}"
            }
            // span {
            //     class: "text-xs font-medium text-elements-lowEmphasis",
            //     "{time}"
            // }
        }
    }
}

#[component]
fn MineTableRowTx(signature: Signature) -> Element {
    let len = signature.to_string().len();
    let first_four = &signature.to_string()[0..4];
    let last_four = &signature.to_string()[len - 4..len];
    rsx! {
        span {
            class: "font-medium my-auto",
            "{first_four}...{last_four}"
        }
    }
}

#[component]
fn MineTableRowScore(pool_score: u64, member_score: u64) -> Element {
    rsx! {
        Col {
            span {
                class: "font-medium my-auto",
                "{member_score}"
            }
            // span {
            //     class: "font-medium text-xs text-elements-lowEmphasis",
            //     "{member_score}"
            // }
        }
    }
}

#[component]
fn MineTableRowReward(net_reward: u64, member_reward: u64) -> Element {
    // let percentage = format_percentage((member_reward as f64 / net_reward as f64) * 100.0);
    rsx! {
        Col {
            // OreValue {
            //     class: "text-right ml-auto",
            //     ui_amount_string: amount_to_ui_amount_string(member_reward, TOKEN_DECIMALS),
            //     with_decimal_units: true,
            //     size: TokenValueSize::Small,
            //     gold: true,
            //     // color_override: "text-elements-highEmphasis",
            // }
            // OreValue {
            //     class: "text-right ml-auto",
            //     ui_amount_string: amount_to_ui_amount_string(member_reward, TOKEN_DECIMALS),
            //     with_decimal_units: true,
            //     size: TokenValueSize::XSmall,
            //     color_override: "text-elements-lowEmphasis",
            // }
            span {
                class: "text-right ml-auto font-medium text-elements-gold",
                "{member_reward} grams"
            }
        }
    }
}
