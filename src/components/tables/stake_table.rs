use dioxus::prelude::*;
use num_format::{Locale, ToFormattedString};
use ore_api::consts::{MINT_ADDRESS, TOKEN_DECIMALS};
use ore_boost_api::state::{Boost, Stake};
use steel::Pubkey;

use crate::{
    components::*,
    config::{BoostMeta, Token, LISTED_BOOSTS, LISTED_TOKENS},
    gateway::GatewayResult,
    hooks::{use_all_liquidity_pairs, use_all_stakes, use_boost, use_boost_apy, use_boost_tvl},
    route::Route,
    solana::spl_token::amount_to_ui_amount_string,
    utils::{format_percentage, LiquidityPair},
};

pub fn StakeTable() -> Element {
    let stake_accounts = use_all_stakes();
    let liquidity_pairs = use_all_liquidity_pairs();
    rsx! {
        Col {
            gap: 8,
            Subheading {
                class: "px-5 sm:px-8",
                title: "Boosts"
            }
            Table {
                class: "mx-0 sm:mx-8",
                header: rsx! {
                    TableHeader {
                        left: "Stake",
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
fn IdleTableRow(stake: Resource<GatewayResult<Stake>>) -> Element {
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
                    mint_address: MINT_ADDRESS,
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
    liquidity_pair: Resource<GatewayResult<LiquidityPair>>,
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
                    boost_meta: boost_meta.clone(),
                    boost,
                    stake,
                    liquidity_pair
                }
            },
            right_3: rsx! {
                StakeTableRowYield {
                    mint_address: boost_meta.lp_mint,
                    boost,
                    stake,
                }
            },
        }
    }
}

#[component]
fn IdleTableRowTitle(token: Token, stake: Resource<GatewayResult<Stake>>) -> Element {
    let balance = use_resource(move || async move {
        let Some(Ok(stake)) = stake.cloned() else {
            return None;
        };
        Some(format_token_amount(
            amount_to_ui_amount_string(stake.balance, TOKEN_DECIMALS),
            Some(true),
            Some(true),
        ))
    });

    rsx! {
        Row {
            class: "my-auto",
            gap: 4,
            img {
                class: "w-8 h-8 rounded-full shrink-0 my-auto",
                src: "{token.image}",
            }
            Col {
                Row {
                    class: "my-auto",
                    gap: 2,
                    span {
                        class: "font-semibold my-auto h-min",
                        "{token.ticker}"
                    }
                    span {
                        class: "font-medium my-auto text-xs text-elements-midEmphasis/50 px-1.5 py-0 rounded bg-elements-lowEmphasis/40",
                        "Idle"
                    }
                }
                if let Some(Some(balance)) = balance.cloned() {
                    span {
                        class: "font-medium text-xs text-elements-lowEmphasis",
                        "{balance} ORE"
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
    liquidity_pair: Resource<GatewayResult<LiquidityPair>>,
) -> Element {
    let token = LISTED_TOKENS.get(&pair_mint).cloned();

    let ore_balance = use_resource(move || async move {
        let Some(Ok(liquidity_pair)) = liquidity_pair.cloned() else {
            return None;
        };
        let Some(Ok(stake)) = stake.cloned() else {
            return None;
        };
        if stake.balance == 0 {
            return None;
        };
        let (ore_amount_f64, _token_amount_f64, _token_ticker, _token_decimals) =
            liquidity_pair.get_stake_amounts(stake.balance);
        Some(format_token_amount(
            ore_amount_f64.to_string(),
            Some(true),
            Some(true),
        ))
    });

    rsx! {
        Row {
            class: "gap-4 my-auto",
            img {
                class: "w-8 h-8 rounded-full shrink-0 my-auto",
                src: asset!("/public/icon.png"),
            }
            if let Some(token) = token {
                img {
                    class: "w-8 h-8 rounded-full shrink-0 my-auto -ml-5",
                    src: "{token.image}",
                }
            } else {
                img {
                    class: "w-8 h-8 rounded-full shrink-0 my-auto -ml-5",
                    src: "", // TODO Unknown token icon
                }
            }
            Col {
                span {
                    class: "font-semibold my-auto",
                    "{ticker}"
                }
                if let Some(Some(ore_balance)) = ore_balance.cloned() {
                    span {
                        class: "font-medium text-xs text-elements-lowEmphasis",
                        "{ore_balance} ORE"
                    }
                }
            }
        }
    }
}

#[component]
fn StakeTableRowMultiplier(
    boost: Resource<GatewayResult<Boost>>,
    stake: Resource<GatewayResult<Stake>>,
) -> Element {
    let user_percentage = use_memo(move || {
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
        let pct = (stake.balance + stake.balance_pending) as f64
            / (boost.total_deposits + stake.balance_pending) as f64
            * 100.0;
        Some(format_percentage(pct))
    });

    rsx! {
        if let Some(Ok(boost)) = boost.cloned() {
            Col {
                span {
                    class: "text-right my-auto font-medium",
                    "{boost.multiplier as f64 / ore_boost_api::consts::BOOST_DENOMINATOR as f64}x"
                }
                if let Some(percentage) = user_percentage.cloned() {
                    span {
                        class: "text-right my-auto font-medium text-elements-lowEmphasis text-xs",
                        "{percentage}"
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
    stake: Resource<GatewayResult<Stake>>,
) -> Element {
    let boost_tvl = use_boost_tvl(MINT_ADDRESS);

    let user_tvl = use_memo(move || {
        let Ok(boost_tvl) = boost_tvl.cloned() else {
            return None;
        };
        let Some(Ok(boost)) = boost.cloned() else {
            return None;
        };
        let Some(Ok(stake)) = stake.cloned() else {
            return None;
        };
        if stake.balance > 0 || stake.balance_pending > 0 {
            let total_balance = stake.balance + stake.balance_pending;
            let total_deposits = boost.total_deposits + stake.balance_pending;
            let user_tvl =
                (boost_tvl * (total_balance as f64 / total_deposits as f64)).floor() as u64;
            Some(user_tvl.to_formatted_string(&Locale::en))
        } else {
            None
        }
    });

    rsx! {
        if let Ok(boost_tvl) = boost_tvl.cloned() {
            Col {
                UsdValue {
                    ui_amount_string: boost_tvl.to_string(),
                }
                if let Some(user_tvl) = user_tvl.cloned() {
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
    boost_meta: BoostMeta,
    boost: Resource<GatewayResult<Boost>>,
    stake: Resource<GatewayResult<Stake>>,
    liquidity_pair: Resource<GatewayResult<LiquidityPair>>,
) -> Element {
    let boost_tvl = use_boost_tvl(boost_meta.lp_mint);

    let user_tvl = use_memo(move || {
        let Some(Ok(liquidity_pair)) = liquidity_pair.cloned() else {
            return None;
        };
        let Some(Ok(stake)) = stake.cloned() else {
            return None;
        };
        if stake.balance > 0 || stake.balance_pending > 0 {
            let total_balance = stake.balance + stake.balance_pending;
            // let total_deposits = boost.total_deposits + stake.balance_pending;
            let user_tvl = (liquidity_pair.total_value_usd
                * (total_balance as f64 / liquidity_pair.shares as f64))
                .floor() as u64;
            Some(user_tvl.to_formatted_string(&Locale::en))
        } else {
            None
        }
    });

    rsx! {
        if let Ok(boost_tvl) = boost_tvl.cloned() {
            Col {
                UsdValue {
                    ui_amount_string: boost_tvl.to_string(),
                }
                if let Some(user_tvl) = user_tvl.cloned() {
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
fn StakeTableRowYield(
    mint_address: Pubkey,
    boost: Resource<GatewayResult<Boost>>,
    stake: Resource<GatewayResult<Stake>>,
) -> Element {
    let apy = use_boost_apy(mint_address);
    rsx! {
        if let Ok(apy) = apy.cloned() {
            Col {
                span {
                    class: "text-right my-auto font-medium",
                    "{apy:.0}%"
                }
                if let Some(Ok(stake)) = stake.cloned() {
                    if stake.rewards > 0 {
                        OreValue {
                            class: "text-right ml-auto",
                            ui_amount_string: amount_to_ui_amount_string(stake.rewards, TOKEN_DECIMALS),
                            with_decimal_units: true,
                            size: TokenValueSize::XSmall,
                            gold: true,
                            abbreviated: true,
                        }
                    } else {
                        span {
                            class: "text-right ml-auto text-elements-lowEmphasis font-medium text-xs mr-1",
                            "–"
                        }
                    }
                }
            }
        } else {
            TableCellLoading {}
        }
    }
}
