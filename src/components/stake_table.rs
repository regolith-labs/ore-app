use dioxus::prelude::*;
use ore_api::consts::TOKEN_DECIMALS;
use ore_boost_api::state::{Boost, Stake};
use solana_extra_wasm::program::spl_token::amount_to_ui_amount_string;
use steel::Pubkey;

use crate::{
    components::{Col, OreValueSmallAbbreviated, Row, Table, TableCellLoading, TableHeader, TableRowLink, TokenValueSmall}, config::{BoostMeta, Token, LISTED_BOOSTS, LISTED_TOKENS}, gateway::GatewayResult, hooks::{use_boost, use_boost_deposits, use_stake}, route::Route
};

pub fn StakeTable() -> Element {
    rsx! {
        Table {
            class: "mx-0 sm:mx-8",
            header: rsx! {
                TableHeader {
                    left: "Pair",
                    right_1: "Multiplier",
                    right_2: "TVL",
                    right_3: "Yield",
                }
            },
            rows: rsx! {
                for boost_meta in LISTED_BOOSTS.iter() {
                    StakeTableRow {
                        boost_meta: boost_meta.clone(),
                    }
                }
            }
        }
    }
}

#[component]
fn StakeTableRow(boost_meta: BoostMeta) -> Element {
    let boost = use_boost(boost_meta.lp_mint);
    let stake = use_stake(boost_meta.lp_mint);
    rsx! {
        TableRowLink {
            to: Route::Pair { lp_mint: boost_meta.lp_mint.to_string() },
            left: rsx! {
                StakeTableRowTitle {
                    ticker: boost_meta.ticker.clone(),
                    pair_mint: boost_meta.pair_mint,
                }
            },
            right_1: rsx! {
                StakeTableRowMultiplier {
                    boost
                }
            },
            right_2: rsx! {
                StakeTableRowDeposits {
                    boost,
                    boost_meta
                }
            },
            right_3: rsx! {
                StakeTableRowYield {
                    boost,
                    stake,
                }
            },
        }
    }
}

#[component]
fn StakeTableRowTitle(ticker: String, pair_mint: Pubkey) -> Element {
    let token = LISTED_TOKENS.get(&pair_mint).cloned();
    rsx! {
        Row {
            class: "gap-4 my-auto",
            if let Some(token) = token {
                img {
                    class: "w-8 h-8 rounded-full shrink-0",
                    src: "{token.image}",
                }
            } else {
                img {
                    class: "w-8 h-8 rounded-full shrink-0",
                    src: "", // TODO Unknown token icon
                }
            }
            img {
                class: "w-8 h-8 rounded-full shrink-0 -ml-6",
                src: asset!("/public/icon.png"),
            }
            span {
                class: "font-semibold my-auto",
                "{ticker}"
            }
        }
    }
}

#[component]
fn StakeTableRowMultiplier(boost: Resource<GatewayResult<Boost>>) -> Element {
    rsx! {
        if let Some(Ok(boost)) = boost.cloned() {
            span {
                class: "text-right my-auto font-medium",
                "{boost.multiplier as f64 / ore_boost_api::consts::BOOST_DENOMINATOR as f64}x"
            }
        } else {
            TableCellLoading {}
        }
    }
}

#[component]
fn StakeTableRowDeposits(boost: Resource<GatewayResult<Boost>>, boost_meta: BoostMeta) -> Element {
    let boost_deposits = use_boost_deposits(boost_meta);
    rsx! {
        if let Some(Ok(boost_deposits)) = boost_deposits.cloned() {
            Col {
                gap: 2,
                TokenValueSmall {
                    class: "ml-auto",
                    amount: boost_deposits.balance_a.to_string(),
                    ticker: boost_deposits.token_a.clone(),
                }
                TokenValueSmall {
                    class: "ml-auto",
                    amount: boost_deposits.balance_b.to_string(),
                    ticker: boost_deposits.token_b.clone(),
                }
            }
        } else {
            TableCellLoading {}
        }
    }
}

#[component]
fn StakeTableRowYield(boost: Resource<GatewayResult<Boost>>, stake: Resource<GatewayResult<Stake>>) -> Element {
    rsx! {
        if let Some(stake) = stake.cloned() {
            if let Ok(stake) = stake {
                OreValueSmallAbbreviated {
                    ui_amount_string: amount_to_ui_amount_string(stake.rewards, TOKEN_DECIMALS),
                }
            } else {
                span {
                    class: "font-medium",
                    "–"
                }
            }
        } else {
            TableCellLoading {}
        }
    }
}