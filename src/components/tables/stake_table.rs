use std::collections::HashMap;

use dioxus::prelude::*;
use num_format::{Locale, ToFormattedString};
use ore_api::consts::{MINT_ADDRESS, TOKEN_DECIMALS};
use ore_boost_api::state::{Boost, Stake};
use solana_extra_wasm::program::spl_token::{amount_to_ui_amount, amount_to_ui_amount_string};
use steel::Pubkey;

use crate::{
    components::{Col, NullValue, OreValueSmall, Row, Table, TableCellLoading, TableHeader, TableRowLink, TokenValueSmall, UsdValueSmall}, config::{BoostMeta, Token, LISTED_BOOSTS, LISTED_TOKENS}, gateway::GatewayResult, hooks::{use_boost, use_ore_quote, LiquidityPair}, route::Route
};

#[component]
pub fn StakeTable(
    stake_accounts: HashMap<Pubkey, Resource<GatewayResult<Stake>>>,
    liquidity_pairs: HashMap<Pubkey, Resource<GatewayResult<LiquidityPair>>>
) -> Element {
    rsx! {
        Col {
            span {
                class: "text-elements-highEmphasis font-semibold text-2xl px-5 sm:px-8 mb-4",
                "Boosts"
            }
            Table {
                class: "mx-0 sm:mx-8",
                header: rsx! {
                    TableHeader {
                        left: "Idle",
                        right_1: "Multiplier",
                        right_2: "TVL",
                        right_3: "Yield",
                    }
                },
                rows: rsx! {
                    if let Some(stake) = stake_accounts.get(&MINT_ADDRESS) {
                        IdleTableRow {
                            stake: *stake
                        }
                    }
                    TableHeader {
                        class: "mt-4",
                        left: "Pairs",
                        right_1: "",
                        right_2: "",
                        right_3: "",
                    }
                    for boost_meta in LISTED_BOOSTS.iter() {
                        if let Some(stake) = stake_accounts.get(&boost_meta.lp_mint) {
                            if let Some(liquidity_pair) = liquidity_pairs.get(&boost_meta.lp_mint) {
                                StakeTableRow {
                                    boost_meta: boost_meta.clone(),
                                    stake: *stake,
                                    liquidity_pair: *liquidity_pair
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
fn IdleTableRow(
    stake: Resource<GatewayResult<Stake>>
) -> Element {
    let token = Token::ore();
    let boost = use_boost(token.mint);
    rsx! {
        TableRowLink {
            to: Route::Idle {},
            left: rsx! {
                IdleTableRowTitle {
                    token,
                    stake
                }
            },
            right_1: rsx! {
                StakeTableRowMultiplier {
                    boost,
                    stake
                }
            },
            right_2: rsx! {
                IdleTableRowTVL {
                    boost,
                    stake
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
fn StakeTableRow(
    boost_meta: BoostMeta,
    stake: Resource<GatewayResult<Stake>>,
    liquidity_pair: Resource<GatewayResult<LiquidityPair>>
) -> Element {
    let boost = use_boost(boost_meta.lp_mint);
    rsx! {
        TableRowLink {
            to: Route::Pair { lp_mint: boost_meta.lp_mint.to_string() },
            left: rsx! {
                StakeTableRowTitle {
                    ticker: boost_meta.ticker.clone(),
                    pair_mint: boost_meta.pair_mint,
                    stake: stake.clone(),
                    liquidity_pair: liquidity_pair.clone(),
                }
            },
            right_1: rsx! {
                StakeTableRowMultiplier {
                    boost,
                    stake
                }
            },
            right_2: rsx! {
                StakeTableRowTVL {
                    liquidity_pair,
                    boost,
                    stake
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
fn IdleTableRowTitle(token: Token, stake: Resource<GatewayResult<Stake>>) -> Element {
    let balance = if let Some(Ok(stake)) = stake.cloned() {
        Some(
            amount_to_ui_amount_string(stake.balance, TOKEN_DECIMALS)
                .trim_end_matches("0")
                .trim_end_matches(".")
                .to_string()
        )
    } else {
        None
    };
    rsx! {
        Row {
            class: "gap-4 my-auto",
            img {
                class: "w-8 h-8 rounded-full shrink-0 my-auto",
                src: "{token.image}",
            }
            Col {
                span {
                    class: "font-semibold my-auto",
                    "{token.ticker}"
                }
                if let Some(balance) = balance {
                    span {
                        class: "font-medium text-xs text-elements-lowEmphasis",
                        "{balance}"
                    }
                }
            }
        }
    }
}


#[component]
fn StakeTableRowTitle(
    ticker: String, 
    pair_mint: Pubkey, 
    stake: Resource<GatewayResult<Stake>>, 
    liquidity_pair: Resource<GatewayResult<LiquidityPair>>
) -> Element {
    let token = LISTED_TOKENS.get(&pair_mint).cloned();
    let ore_balance = if let Some(Ok(liquidity_pair)) = liquidity_pair.cloned() {
        if let Some(Ok(stake)) = stake.cloned() {
            if stake.balance > 0 {
                let (ore_amount_f64, _token_amount_f64, _token_ticker, _token_decimals) = liquidity_pair.get_stake_amounts(stake.balance);
                Some(
                    format!("{:.1$}", ore_amount_f64, TOKEN_DECIMALS as usize)
                        .trim_end_matches("0")
                        .trim_end_matches(".")
                        .to_string()
                )
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };
    rsx! {
        Row {
            class: "gap-4 my-auto",
            if let Some(token) = token {
                img {
                    class: "w-8 h-8 rounded-full shrink-0 my-auto",
                    src: "{token.image}",
                }
            } else {
                img {
                    class: "w-8 h-8 rounded-full shrink-0 my-auto",
                    src: "", // TODO Unknown token icon
                }
            }
            img {
                class: "w-8 h-8 rounded-full shrink-0 my-auto -ml-6",
                src: asset!("/public/icon.png"),
            }
            Col {
                span {
                    class: "font-semibold my-auto",
                    "{ticker}"
                }
                if let Some(ore_balance) = ore_balance {
                    span {
                        class: "font-medium text-xs text-elements-lowEmphasis",
                        "{ore_balance}"
                    }
                }
            }
        }
    }
}

#[component]
fn StakeTableRowMultiplier(
    boost: Resource<GatewayResult<Boost>>,
    stake: Resource<GatewayResult<Stake>>
) -> Element {
    // Get user's percentage of total deposits
    let user_percentage = use_resource(move || async move {
        let Some(Ok(boost)) = boost.cloned() else {
            return None;
        };
        let Some(Ok(stake)) = stake.cloned() else {
            return None;
        };
        if stake.balance == 0 {
            return None;
        }
        if boost.total_deposits == 0 {
            return None;
        }
        let pct = stake.balance as f64 / boost.total_deposits as f64 * 100.0;
        let pct = if pct < 0.01 {
            // Find first non-zero decimal place
            let mut decimals = 0;
            let mut val = pct;
            while val < 1.0 {
                val *= 10.0;
                decimals += 1;
            }
            (pct * 10f64.powi(decimals)).floor() / 10f64.powi(decimals)
        } else {
            (pct * 10.0).floor() / 10.0 // One decimal place
        };
        Some(pct)
    });

    rsx! {
        if let Some(Ok(boost)) = boost.cloned() {
            Col {
                span {
                    class: "text-right my-auto font-medium",
                    "{boost.multiplier as f64 / ore_boost_api::consts::BOOST_DENOMINATOR as f64}x"
                }
                if let Some(Some(percentage)) = user_percentage.cloned() {
                    span {
                        class: "text-right my-auto font-medium text-elements-lowEmphasis text-xs",
                        "{percentage}%"
                    }
                }
            }
        } else {
            TableCellLoading {}
        }
    }
}

#[component]
fn IdleTableRowTVL(
    boost: Resource<GatewayResult<Boost>>,
    stake: Resource<GatewayResult<Stake>>
) -> Element {
    let usdc = Token::usdc();
    let quote = use_ore_quote(usdc.mint);
    let tvl = use_resource(move || async move {
        let Some(Ok(boost)) = *boost.read() else {
            return None;
        };
        let Some(Ok(quote)) = quote.cloned() else {
            return None;
        };
        let usdc_quote_f64 = amount_to_ui_amount(quote.out_amount, usdc.decimals);
        let total_deposits_f64 = amount_to_ui_amount(boost.total_deposits, TOKEN_DECIMALS);
        Some(usdc_quote_f64 * total_deposits_f64)
    });

    let user_tvl = use_resource(move || async move {
        let Some(Some(tvl)) = tvl.cloned() else {
            return None;
        };
        let Some(Ok(boost)) = boost.cloned() else {
            return None;
        };
        let Some(Ok(stake)) = stake.cloned() else {
            return None;
        };
        if stake.balance > 0 {
            let user_tvl = (tvl * (stake.balance as f64 / boost.total_deposits as f64)).floor() as u64;
            Some(user_tvl.to_formatted_string(&Locale::en))
        } else {
            None
        }
    });

    rsx! {
        if let Some(Some(tvl)) = tvl.cloned() {
            Col {
                UsdValueSmall {
                    amount: tvl.to_string(),
                }
                if let Some(Some(user_tvl)) = user_tvl.cloned() {
                    span {
                        class: "text-right my-auto font-medium text-elements-lowEmphasis text-xs",
                        "${user_tvl}"
                    }
                }
            }
        } else {
            TableCellLoading {}
        }
    }
}

#[component]
fn StakeTableRowTVL(
    liquidity_pair: Resource<GatewayResult<LiquidityPair>>,
    boost: Resource<GatewayResult<Boost>>,
    stake: Resource<GatewayResult<Stake>>
) -> Element {
    let user_tvl = use_resource(move || async move {
        let Some(Ok(liquidity_pair)) = liquidity_pair.cloned() else {
            return None;
        };
        let Some(Ok(boost)) = boost.cloned() else {
            return None;
        };
        let Some(Ok(stake)) = stake.cloned() else {
            return None;
        };
        if stake.balance > 0 {
            let user_tvl = (liquidity_pair.total_value_usd * (stake.balance as f64 / boost.total_deposits as f64)).floor() as u64;
            Some(user_tvl.to_formatted_string(&Locale::en))
        } else {
            None
        }
    });

    rsx! {
        if let Some(Ok(liquidity_pair)) = liquidity_pair.cloned() {
            Col {
                UsdValueSmall {
                    amount: liquidity_pair.total_value_usd.to_string(),
                }
                if let Some(Some(user_tvl)) = user_tvl.cloned() {
                    span {
                        class: "text-right my-auto font-medium text-elements-lowEmphasis text-xs",
                        "${user_tvl}"
                    }
                }
            }
        } else {
            TableCellLoading {}
        }
    }
}

#[component]
fn StakeTableRowBasis(liquidity_pair: Resource<GatewayResult<LiquidityPair>>) -> Element {
    rsx! {
        if let Some(Ok(liquidity_pair)) = liquidity_pair.cloned() {
            Col {
                gap: 2,
                // TokenValueSmall {
                //     class: "ml-auto",
                //     amount: liquidity_pair.balance_a.to_string(),
                //     ticker: liquidity_pair.token_a.clone(),
                // }
                OreValueSmall {
                    ui_amount_string: liquidity_pair.balance_b_f64.to_string(),
                }
                // TokenValueSmall {
                //     class: "ml-auto",
                //     amount: liquidity_pair.balance_b.to_string(),
                //     ticker: liquidity_pair.token_b.clone(),
                // }
            }
        } else {
            TableCellLoading {}
        }
    }
}

#[component]
fn StakeTableRowLiquidity(liquidity_pair: Resource<GatewayResult<LiquidityPair>>) -> Element {
    rsx! {
        if let Some(Ok(liquidity_pair)) = liquidity_pair.cloned() {
            Col {
                gap: 2,
                TokenValueSmall {
                    class: "ml-auto",
                    amount: liquidity_pair.balance_a_f64.to_string(),
                    ticker: liquidity_pair.token_a.ticker.clone(),
                }
                // TokenValueSmall {
                //     class: "ml-auto",
                //     amount: liquidity_pair.balance_b.to_string(),
                //     ticker: liquidity_pair.token_b.clone(),
                // }
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
                if stake.rewards > 0 {
                    OreValueSmall {
                        class: "text-elements-gold",
                        ui_amount_string: amount_to_ui_amount_string(stake.rewards, TOKEN_DECIMALS),
                        small_units: true,
                        abbreviated: true,
                    }
                } else {
                    NullValue {}
                }
            } else {
                NullValue {}
            }
        } else {
            TableCellLoading {}
        }
    }
}