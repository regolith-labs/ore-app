use dioxus::prelude::*;
use num_format::{Locale, ToFormattedString};
use ore_api::{
    consts::{MINT_ADDRESS, TOKEN_DECIMALS},
    state::Proof,
};
use ore_boost_api::state::{Boost, Stake};
use steel::Pubkey;

use crate::{
    components::*,
    config::{BoostMeta, LpType, Token, LISTED_BOOSTS, LISTED_TOKENS},
    gateway::GatewayResult,
    hooks::{
        use_all_liquidity_pairs, use_boost_apr, use_boost_proof_wss, use_boost_tvl, use_boost_wss,
        use_claimable_yield, use_stake_wss,
    },
    route::Route,
    solana::spl_token::amount_to_ui_amount_string,
    utils::{format_time_since, LiquidityPair},
};

pub fn StakeTable() -> Element {
    let liquidity_pairs = use_all_liquidity_pairs();
    let mut info_hidden = use_signal(|| true);
    rsx! {
        Col {
            gap: 0,
            button {
                class: "flex flex-row gap-2 px-5 sm:px-8 w-min group hover:cursor-pointer",
                onclick: move |_| info_hidden.set(!info_hidden.cloned()),
                Subheading {
                    class: "my-auto",
                    title: "Boosts"
                }
                InfoIcon {
                    class: "h-4 w-4 shrink-0 text-elements-lowEmphasis group-hover:text-elements-highEmphasis transition-all duration-300 ease-in-out my-auto",
                }
            }
            InfoText {
                class: "text-wrap px-5 sm:px-8",
                text: "Boosts automatically distribute a portion of all newly mined supply to liquidity providers as yield.",
                hidden: info_hidden,
            }
            Table {
                class: "mt-8 mx-0 sm:mx-8",
                header: rsx! {
                    TableHeader {
                        left: "Stake",
                        right_1: "APR",
                        right_2: "TVL",
                        right_3: "Yield",
                        help_left: "Holders of the assets below are eligible to receive ORE yield.",
                        help_right_1: "Estimated annual percentage rate based on 7d trailing returns.",
                        help_right_2: "Current notional value of assets deposited in the protocol.",
                        help_right_3: "Amount of yield you have earned and may claim.",
                        help_hidden: info_hidden,
                    }
                },
                rows: rsx! {
                    IdleTableRow {}
                    for boost_meta in LISTED_BOOSTS.iter() {
                        if let Some(liquidity_pair) = liquidity_pairs.get(&boost_meta.lp_mint) {
                            StakeTableRow {
                                boost_meta: boost_meta.clone(),
                                liquidity_pair: *liquidity_pair
                            }
                        }
                    }
                }
            }
        }
    }
}

fn IdleTableRow() -> Element {
    let token = Token::ore();
    let stake = use_stake_wss(token.mint);
    let boost = use_boost_wss(token.mint);
    let boost_proof = use_boost_proof_wss(token.mint);
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
                StakeTableRowAPR {
                    mint_address: MINT_ADDRESS,
                    stake,
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
                    boost_proof,
                    stake,
                }
            },
        }
    }
}

#[component]
fn StakeTableRow(
    boost_meta: BoostMeta,
    liquidity_pair: Resource<GatewayResult<LiquidityPair>>,
) -> Element {
    let stake = use_stake_wss(boost_meta.lp_mint);
    let boost = use_boost_wss(boost_meta.lp_mint);
    let boost_proof = use_boost_proof_wss(boost_meta.lp_mint);
    rsx! {
        TableRowLink {
            to: Route::Pair { lp_mint: boost_meta.lp_mint.to_string() },
            left: rsx! {
                StakeTableRowTitle {
                    boost_meta: boost_meta.clone(),
                    pair_mint: boost_meta.pair_mint,
                    stake: stake.clone(),
                    liquidity_pair: liquidity_pair.clone(),
                }
            },
            right_1: rsx! {
                StakeTableRowAPR {
                    mint_address: boost_meta.lp_mint,
                    stake,
                }
            },
            right_2: rsx! {
                StakeTableRowTVL {
                    boost_meta: boost_meta.clone(),
                    stake,
                    liquidity_pair
                }
            },
            right_3: rsx! {
                StakeTableRowYield {
                    mint_address: boost_meta.lp_mint,
                    boost,
                    boost_proof,
                    stake,
                }
            },
        }
    }
}

#[component]
fn IdleTableRowTitle(token: Token, stake: Signal<GatewayResult<Stake>>) -> Element {
    let balance = use_resource(move || async move {
        let Ok(stake) = stake.cloned() else {
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
    boost_meta: BoostMeta,
    pair_mint: Pubkey,
    stake: Signal<GatewayResult<Stake>>,
    liquidity_pair: Resource<GatewayResult<LiquidityPair>>,
) -> Element {
    let token = LISTED_TOKENS.get(&pair_mint).cloned();

    let token_balances = use_resource(move || async move {
        let Some(Ok(liquidity_pair)) = liquidity_pair.cloned() else {
            return None;
        };
        let Ok(stake) = stake.cloned() else {
            return None;
        };
        if stake.balance == 0 {
            return None;
        };
        let (ore_amount_f64, token_amount_f64, token_ticker, _token_decimals) =
            liquidity_pair.get_stake_amounts(stake.balance);
        Some(format!(
            "{} {} / {} {}",
            format_token_amount(ore_amount_f64.to_string(), Some(true), Some(true)),
            "ORE",
            format_token_amount(token_amount_f64.to_string(), Some(true), Some(true)),
            token_ticker,
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
                Row {
                    class: "my-auto",
                    gap: 2,
                    span {
                        class: "font-semibold my-auto",
                        "{boost_meta.ticker}"
                    }
                    match boost_meta.lp_type {
                        LpType::Kamino => rsx! { img {
                            class: "w-4 h-4 shrink-0 my-auto rounded border border-elements-lowEmphasis/40",
                            src: asset!("/public/kamino_logo.jpg"),
                        }},
                        LpType::Meteora => rsx! { img {
                            class: "w-4 h-4 shrink-0 my-auto rounded border border-elements-lowEmphasis/40",
                            src: asset!("/public/meteora_logo.jpg"),
                        }},
                    }
                }
                if let Some(Some(token_balances)) = token_balances.cloned() {
                    span {
                        class: "font-medium text-xs text-elements-lowEmphasis",
                        "{token_balances}"
                    }
                }
            }
        }
    }
}

#[component]
fn IdleTableRowTVL(
    boost: Signal<GatewayResult<Boost>>,
    stake: Signal<GatewayResult<Stake>>,
) -> Element {
    let boost_tvl = use_boost_tvl(MINT_ADDRESS);

    let user_tvl = use_memo(move || {
        let Ok(boost_tvl) = boost_tvl.cloned() else {
            return None;
        };
        let Ok(boost) = boost.cloned() else {
            return None;
        };
        let Ok(stake) = stake.cloned() else {
            return None;
        };
        if stake.balance > 0 {
            let total_balance = stake.balance;
            let total_deposits = boost.total_deposits;
            let user_tvl =
                (boost_tvl * (total_balance as f64 / total_deposits as f64)).floor() as u64;

            Some(format!("${}", user_tvl.to_formatted_string(&Locale::en)))

            // let pct = (stake.balance + stake.balance_pending) as f64
            //     / (boost.total_deposits + stake.balance_pending) as f64
            //     * 100.0;

            // Some(format!(
            //     "${}  •  {}",
            //     user_tvl.to_formatted_string(&Locale::en),
            //     format_percentage(pct)
            // ))
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
                        "{user_tvl}"
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
    stake: Signal<GatewayResult<Stake>>,
    liquidity_pair: Resource<GatewayResult<LiquidityPair>>,
) -> Element {
    let boost_tvl = use_boost_tvl(boost_meta.lp_mint);

    let user_tvl = use_memo(move || {
        let Some(Ok(liquidity_pair)) = liquidity_pair.cloned() else {
            return None;
        };
        let Ok(stake) = stake.cloned() else {
            return None;
        };
        if stake.balance > 0 {
            let total_balance = stake.balance;
            let user_tvl = (liquidity_pair.total_value_usd
                * (total_balance as f64 / liquidity_pair.shares as f64))
                .floor() as u64;

            Some(format!("${}", user_tvl.to_formatted_string(&Locale::en)))

            // let pct = (stake.balance + stake.balance_pending) as f64
            //     / (boost.total_deposits + stake.balance_pending) as f64
            //     * 100.0;

            // Some(format!(
            //     "${}  •  {}",
            //     user_tvl.to_formatted_string(&Locale::en),
            //     format_percentage(pct)
            // ))
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
                        "{user_tvl}"
                    }
                }
            }
        } else {
            TableCellLoading {}
        }
    }
}

#[component]
fn StakeTableRowAPR(mint_address: Pubkey, stake: Signal<GatewayResult<Stake>>) -> Element {
    let apr = use_boost_apr(mint_address);
    rsx! {
        if let Ok(apr) = apr.cloned() {
            Col {
                span {
                    class: "text-right my-auto font-medium",
                    "{apr:.0}%"
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
    boost: Signal<GatewayResult<Boost>>,
    boost_proof: Signal<GatewayResult<Proof>>,
    stake: Signal<GatewayResult<Stake>>,
) -> Element {
    // Calculate rewards
    let claimable_yield = use_claimable_yield(boost, boost_proof, stake);

    rsx! {
        Col {
            if let Ok(stake) = stake.cloned() {
                if *claimable_yield.read() > 0 {
                    Col {
                        OreValue {
                            class: "text-right ml-auto",
                            ui_amount_string: amount_to_ui_amount_string(*claimable_yield.read(), TOKEN_DECIMALS),
                            with_decimal_units: true,
                            size: TokenValueSize::Small,
                            gold: true,
                            abbreviated: true,
                        }
                        span {
                            class: "text-right my-auto font-medium text-elements-lowEmphasis text-xs",
                            "{format_time_since(stake.last_claim_at as u64)}"
                        }
                    }
                } else {
                    span {
                        class: "text-right ml-auto text-elements-midEmphasis font-medium mr-1",
                        "–"
                    }
                }
            } else {
                span {
                    class: "text-right ml-auto text-elements-midEmphasis font-medium mr-1",
                    "–"
                }
            }
        }
    }
}
