use std::str::FromStr;

use dioxus::prelude::*;
use ore_api::consts::TOKEN_DECIMALS;
use ore_boost_api::state::{Boost, Stake};
use solana_extra_wasm::program::spl_token::amount_to_ui_amount_string;
use steel::Pubkey;

use crate::{
    components::*, 
    config::{BoostMeta, LpType, LISTED_BOOSTS_BY_MINT}, 
    gateway::{GatewayResult, UiTokenAmount}, 
    hooks::{on_transaction_done, use_boost, use_boost_claim_transaction, use_liquidity_pair, use_lp_deposit_transaction, use_stake, use_token_balance, use_liquidity_pair_balances, LiquidityPair}
};

#[component]
pub fn Pair(lp_mint: String) -> Element {
    let lp_mint = Pubkey::from_str(&lp_mint).unwrap();
    let boost_meta = LISTED_BOOSTS_BY_MINT.get(&lp_mint).unwrap();
    let boost = use_boost(lp_mint);
    let liquidity_pair = use_liquidity_pair(boost_meta.clone());
    let stake = use_stake(lp_mint);
    let lp_balance = use_token_balance(lp_mint);
    let (token_a_balance, token_b_balance) = use_liquidity_pair_balances(liquidity_pair);
    
    rsx! {
        Col {
            class: "w-full h-full pb-20 sm:pb-16",
            gap: 8,
            Heading {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                title: format!("Stake {}", boost_meta.ticker.clone()),
                subtitle: format!("Manage your {} position.", boost_meta.name.clone())
            }
            Col {
                class: "w-full h-full gap-16",
                PairStakeForm {
                    class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                    boost_meta: boost_meta.clone(),
                    liquidity_pair: liquidity_pair,
                    lp_balance: lp_balance,
                    stake: stake,
                    token_a_balance: token_a_balance,
                    token_b_balance: token_b_balance,
                }
                AccountMetrics {
                    boost_meta: boost_meta.clone(),
                    liquidity_pair: liquidity_pair,
                    lp_balance: lp_balance,
                    token_a_balance: token_a_balance,
                    token_b_balance: token_b_balance,
                    boost,
                    stake
                }
                SummaryMetrics {
                    boost,
                    liquidity_pair,
                    boost_meta: boost_meta.clone()
                }
            }
        }
    }
}

#[component]
fn AccountMetrics(
    boost_meta: BoostMeta,
    liquidity_pair: Resource<GatewayResult<LiquidityPair>>,
    lp_balance: Resource<GatewayResult<UiTokenAmount>>,
    token_a_balance: Resource<GatewayResult<UiTokenAmount>>,
    token_b_balance: Resource<GatewayResult<UiTokenAmount>>,
    boost: Resource<GatewayResult<Boost>>,
    stake: Resource<GatewayResult<Stake>>,
) -> Element {
    let err = use_signal(|| None);

    // Refresh data if successful transaction
    on_transaction_done(move |_sig| {
        token_a_balance.restart();
        token_b_balance.restart();
        stake.restart();
        boost.restart();
    });

    // Build transactions
    let claim_tx = use_boost_claim_transaction(boost, stake);
    let lp_deposit_tx = use_lp_deposit_transaction(boost, stake);

    rsx! {
        Col {
            class: "w-full h-full mx-auto max-w-2xl px-5 sm:px-8 gap-8",
            span {
                class: "text-elements-highEmphasis font-semibold text-2xl",
                "Account"
            }
            Row {
                class: "w-full justify-between px-4",
                span {
                    class: "text-elements-lowEmphasis font-medium",
                    "Deposits"
                }
                if let Some(Ok(liquidity_pair)) = liquidity_pair.cloned() {
                    if let Some(stake) = stake.cloned() {
                        if let Ok(stake) = stake {
                            if stake.balance > 0 {
                                LiquidityPairStakeValue {
                                    stake_balance: stake.balance,
                                    liquidity_pair: liquidity_pair.clone(),
                                    with_decimal_units: true,
                                }
                            } else {
                                NullValue {}
                            }
                        } else {
                            NullValue {}
                        }
                    } else {
                        LoadingValue {}
                    }
                } else {
                    LoadingValue {}
                }
            }
            if let Some(Ok(liquidity_pair)) = liquidity_pair.cloned() {
                if let Some(Ok(stake)) = stake.cloned() {
                    if stake.balance_pending > 0 {
                        Row {
                            class: "w-full justify-between px-4",
                            span {
                                class: "text-elements-lowEmphasis font-medium",
                                "Deposits (pending)"
                            }
                            LiquidityPairStakeValue {
                                stake_balance: stake.balance_pending,
                                liquidity_pair: liquidity_pair.clone(),
                                with_decimal_units: true,
                            }
                        }
                    }
                }
            }
            if let Some(Ok(liquidity_pair)) = liquidity_pair.cloned() {
                if let Some(Ok(lp_balance)) = lp_balance.cloned() {
                    if lp_balance.ui_amount.unwrap_or(0.0) > 0.0 {
                        Row {
                            class: "w-full justify-between px-4",
                            span {
                                class: "text-elements-lowEmphasis font-medium",
                                "Unstaked"
                            }
                            LiquidityPairStakeValue {
                                stake_balance: lp_balance.amount.parse::<u64>().unwrap_or(0),
                                liquidity_pair: liquidity_pair.clone(),
                                with_decimal_units: true,
                            }
                        }
                        SubmitButton {
                            class: "controls-tertiary",
                            title: "Deposit {boost_meta.ticker}",
                            transaction: lp_deposit_tx.clone(),
                            err: err.clone(),
                        }
                    }
                }
            }
            Row {
                class: "w-full justify-between px-4",
                span {
                    class: "text-elements-lowEmphasis font-medium",
                    "Yield"
                }
                if let Some(stake) = stake.cloned() {
                    if let Ok(stake) = stake {
                        if stake.rewards > 0 {
                            OreValue {
                                ui_amount_string: amount_to_ui_amount_string(stake.rewards, TOKEN_DECIMALS),
                                with_decimal_units: true,
                                size: TokenValueSize::Small,
                                gold: true,
                            }
                        } else {
                            NullValue {}
                        }
                    } else {
                        NullValue {}
                    }
                } else {
                    LoadingValue {}
                }
            }
            ClaimButton {
                transaction: claim_tx.clone(),
            }
        }
    }
}

#[component]
fn SummaryMetrics(
    boost: Resource<GatewayResult<Boost>>,
    boost_meta: BoostMeta,
    liquidity_pair: Resource<GatewayResult<LiquidityPair>>
) -> Element {
    rsx! {
        Col {
            class: "w-full h-full mx-auto max-w-2xl px-5 sm:px-8",
            gap: 8,
            span {
                class: "text-elements-highEmphasis font-semibold text-2xl",
                "Boost"
            }
            Row {
                class: "w-full justify-between px-4",
                span {
                    class: "text-elements-lowEmphasis font-medium",
                    "Multiplier"
                }
                if let Some(Ok(boost)) = boost.read().as_ref() {
                    span {
                        class: "text-elements-highEmphasis font-medium",
                        "{boost.multiplier as f64 / ore_boost_api::consts::BOOST_DENOMINATOR as f64}x"
                    }
                } else {
                    LoadingValue {}
                }   
            }
            Row {
                class: "w-full justify-between px-4",
                span {
                    class: "text-elements-lowEmphasis font-medium",
                    "Protocol"
                }
                a {
                    class: "text-elements-highEmphasis font-medium hover:underline",
                    href: match boost_meta.lp_type {
                        LpType::Kamino => format!("https://app.kamino.finance/liquidity/{}", boost_meta.lp_id),
                        LpType::Meteora => format!("https://app.meteora.ag/pools/{}", boost_meta.lp_id),
                    },
                    target: "_blank",
                    "{boost_meta.lp_type}"
                }
            }
            Row {
                class: "w-full justify-between px-4",
                span {
                    class: "text-elements-lowEmphasis font-medium",
                    "Total deposits"
                }
                if let Some(Ok(liquidity_pair)) = liquidity_pair.read().as_ref() {
                    LiquidityPairValue {
                        liquidity_pair: liquidity_pair.clone(),
                        with_decimal_units: true,
                    }
                } else {
                    LoadingValue {}
                }   
            }
            Row {
                class: "w-full justify-between px-4",
                span {
                    class: "text-elements-lowEmphasis font-medium",
                    "Total stakers"
                }
                if let Some(Ok(boost)) = boost.read().as_ref() {
                    span {
                        class: "text-elements-highEmphasis font-medium",
                        "{boost.total_stakers}"
                    }
                } else {
                    LoadingValue {}
                }   
            }
            Row {
                class: "w-full justify-between px-4",
                span {
                    class: "text-elements-lowEmphasis font-medium",
                    "TVL"
                }
                if let Some(Ok(liquidity_pair)) = liquidity_pair.read().as_ref() {
                    UsdValue {
                        ui_amount_string: liquidity_pair.total_value_usd.to_string(),
                    }
                } else {
                    LoadingValue {}
                }   
            }
        }
    }
}
