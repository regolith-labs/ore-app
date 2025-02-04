use dioxus::prelude::*;
use ore_api::consts::TOKEN_DECIMALS;
use ore_boost_api::state::{Boost, Stake};
use solana_extra_wasm::program::spl_token::{amount_to_ui_amount, amount_to_ui_amount_string};
use steel::Pubkey;

use crate::{
    components::{Col, NullValue, OreValueSmall, Row, Table, TableCellLoading, TableHeader, TableRowLink, TokenValueSmall, UsdValueSmall}, config::{BoostMeta, Token, LISTED_BOOSTS, LISTED_TOKENS}, gateway::GatewayResult, hooks::{use_boost, use_liquidity_pair, use_ore_quote, use_stake, LiquidityPair}, route::Route
};

pub fn StakeTable() -> Element {
    rsx! {
        Col {
            span {
                class: "text-elements-highEmphasis font-semibold text-2xl px-5 sm:px-8 mb-4",
                "Boosts"
            }
            // IdleTable {},
            PairsTable {},
        }
    }
}

fn IdleTable() -> Element {
    rsx! {
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
                IdleTableRow {}
            }
        }
    }
}

fn PairsTable() -> Element {
    rsx! {
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
                IdleTableRow {}
                TableHeader {
                    class: "mt-4",
                    left: "Liquidity pairs",
                    right_1: "",
                    right_2: "",
                    right_3: "",
                }
                for boost_meta in LISTED_BOOSTS.iter() {
                    StakeTableRow {
                        boost_meta: boost_meta.clone(),
                    }
                }
            }
        }
    }
}

fn IdleTableRow() -> Element {
    let token = Token::ore();
    let boost = use_boost(token.mint);
    let stake = use_stake(token.mint);
    rsx! {
        TableRowLink {
            to: Route::Idle {},
            left: rsx! {
                IdleTableRowTitle {
                    token
                }
            },
            right_1: rsx! {
                StakeTableRowMultiplier {
                    boost
                }
            },
            right_2: rsx! {
                IdleTableRowTVL {
                    boost
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
fn StakeTableRow(boost_meta: BoostMeta) -> Element {
    let boost = use_boost(boost_meta.lp_mint);
    let stake = use_stake(boost_meta.lp_mint);
    let liquidity_pair = use_liquidity_pair(boost_meta.clone());
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
                StakeTableRowTVL {
                    liquidity_pair
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
fn IdleTableRowTitle(token: Token) -> Element {
    rsx! {
        Row {
            class: "gap-4 my-auto",
            img {
                class: "w-8 h-8 rounded-full shrink-0",
                src: "{token.image}",
            }
            span {
                class: "font-semibold my-auto",
                "{token.ticker}"
            }
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
fn IdleTableRowTVL(boost: Resource<GatewayResult<Boost>>) -> Element {
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
    rsx! {
        if let Some(Some(tvl)) = tvl.cloned() {
            Col {
                gap: 2,
                UsdValueSmall {
                    amount: tvl.to_string(),
                }
            }
        } else {
            TableCellLoading {}
        }
    }
}

#[component]
fn StakeTableRowTVL(liquidity_pair: Resource<GatewayResult<LiquidityPair>>) -> Element {
    rsx! {
        if let Some(Ok(liquidity_pair)) = liquidity_pair.cloned() {
            Col {
                gap: 2,
                UsdValueSmall {
                    amount: liquidity_pair.total_value_usd.to_string(),
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