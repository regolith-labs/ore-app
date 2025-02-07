use dioxus::prelude::*;
use num_format::{Locale, ToFormattedString};
use ore_api::consts::{MINT_ADDRESS, TOKEN_DECIMALS};
use ore_boost_api::state::{Boost, Stake};
use solana_extra_wasm::program::spl_token::{amount_to_ui_amount, amount_to_ui_amount_string};
use steel::Pubkey;

use crate::{
    components::*, 
    config::{BoostMeta, Token, LISTED_BOOSTS, LISTED_TOKENS}, 
    gateway::GatewayResult, 
    hooks::{use_all_liquidity_pairs, use_all_stakes, use_boost, use_ore_price, OrePrice}, 
    route::Route,
    utils::LiquidityPair
};

pub fn MineTable() -> Element {
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
                        right_3: "Yield",
                    }
                },
                rows: rsx! {
                    MineTableRow {}
                    MineTableRow {}
                    MineTableRow {}

                    // if let Some(stake) = stake_accounts.get(&MINT_ADDRESS) {
                    //     IdleTableRow {
                    //         stake: *stake
                    //     }
                    // }
                    // for boost_meta in LISTED_BOOSTS.iter() {
                    //     if let Some(stake) = stake_accounts.get(&boost_meta.lp_mint) {
                    //         if let Some(liquidity_pair) = liquidity_pairs.get(&boost_meta.lp_mint) {
                    //             StakeTableRow {
                    //                 boost_meta: boost_meta.clone(),
                    //                 stake: *stake,
                    //                 liquidity_pair: *liquidity_pair
                    //             }
                    //         }
                    //     }
                    // }
                }
            }
        }
    }
}

#[component]
fn MineTableRow(
    
) -> Element {
    rsx! {
        TableRow {
            left: rsx! {
                MineTableRowTitle {
                    
                }
            },
            right_1: rsx! {
                MineTableRowTx {

                }
            },
            right_2: rsx! {
                MineTableRowScore {

                }
            },
            right_3: rsx! {
                MineTableRowYield {

                }
            },
        }
    }
}

#[component]
fn MineTableRowTitle(
    
) -> Element {    
    rsx! {
        Col {
            class: "gap-4 my-auto",
            span {
                class: "font-semibold my-auto",
                "Nov 12"
            }            
            span {
                class: "font-medium text-xs text-elements-lowEmphasis",
                "10:34PM"
            }                        
        }
    }
}

#[component]
fn MineTableRowTx() -> Element {
    rsx! {
        Col {
            span {
                class: "text-right my-auto font-medium",
                "321kj31b31237813ti13h12"
            }                
        }
    }
}

#[component]
fn MineTableRowScore(    
) -> Element {    
    rsx! {
            Col {          
                span {
                    class: "my-auto font-medium",
                    "37"
                }
            }        
    }
}

#[component]
fn MineTableRowYield(
    
) -> Element {
    rsx! {
        OreValue {
            ui_amount_string: "0".to_string(),
            with_decimal_units: true,
            size: TokenValueSize::Small,
            gold: true,
            abbreviated: true,
        }
    }
}