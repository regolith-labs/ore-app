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
    hooks::{on_transaction_done, use_boost, use_boost_claim_transaction, use_boost_deposits, use_lp_deposit_transaction, use_ore_balance, use_stake, use_token_balance, BoostDeposits}
};

#[component]
pub fn Pair(lp_mint: String) -> Element {
    let lp_mint = Pubkey::from_str(&lp_mint).unwrap();
    let boost_meta = LISTED_BOOSTS_BY_MINT.get(&lp_mint).unwrap();
    let boost = use_boost(lp_mint);
    let boost_deposits = use_boost_deposits(boost_meta.clone());
    let stake = use_stake(lp_mint);
    let pair_balance = use_token_balance(boost_meta.pair_mint);
    let ore_balance = use_ore_balance();
    let lp_balance = use_token_balance(lp_mint);

    // TODO Get the boost
    // TODO Show error if boost is not listed
    
    rsx! {
        Col {
            class: "w-full h-full pb-20 sm:pb-16",
            gap: 8,
            Heading {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                title: boost_meta.name.clone(),
                subtitle: "Manage your liquidity position."
            }
            Col {
                class: "w-full h-full gap-16",
                PairStakeForm {
                    class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                    boost_meta: boost_meta.clone(),
                    boost_deposits: boost_deposits,
                    lp_balance: lp_balance,
                    stake: stake,
                    token_a_balance: pair_balance,
                    token_b_balance: ore_balance,
                }
                AccountMetrics {
                    boost_meta: boost_meta.clone(),
                    boost_deposits: boost_deposits,
                    ore_balance,
                    lp_balance,
                    boost,
                    stake
                }
                SummaryMetrics {
                    boost,
                    boost_deposits,
                    boost_meta: boost_meta.clone()
                }
            }
        }
    }
}

#[component]
fn AccountMetrics(
    boost_meta: BoostMeta,
    boost_deposits: Resource<GatewayResult<BoostDeposits>>,
    lp_balance: Resource<GatewayResult<UiTokenAmount>>,
    ore_balance: Resource<GatewayResult<UiTokenAmount>>,
    boost: Resource<GatewayResult<Boost>>,
    stake: Resource<GatewayResult<Stake>>,
) -> Element {
    let err = use_signal(|| None);

    // Refresh data if successful transaction
    on_transaction_done(move |_sig| {
        ore_balance.restart();
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
                if let Some(Ok(boost_deposits)) = boost_deposits.read().as_ref() {
                    if let Some(stake) = stake.read().as_ref() {
                        if let Ok(stake) = stake {
                            if stake.balance > 0 {
                                PairStakeValue {
                                    shares: stake.balance,
                                    boost_deposits: boost_deposits.clone(),
                                    small_units: Some(true),
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
            if let Some(Ok(boost_deposits)) = boost_deposits.read().as_ref() {
                if let Some(Ok(stake)) = stake.read().as_ref() {
                    if stake.balance_pending > 0 {
                        Row {
                            class: "w-full justify-between px-4",
                            span {
                                class: "text-elements-lowEmphasis font-medium",
                                "Deposits (pending)"
                            }
                            PairStakeValue {
                                shares: stake.balance_pending,
                                boost_deposits: boost_deposits.clone(),
                                small_units: Some(true),
                            }
                        }
                    }
                }
            }
            if let Some(Ok(boost_deposits)) = boost_deposits.read().as_ref() {
                if let Some(Ok(lp_balance)) = lp_balance.read().as_ref() {
                    if lp_balance.ui_amount.unwrap_or(0.0) > 0.0 {
                        Row {
                            class: "w-full justify-between px-4",
                            span {
                                class: "text-elements-lowEmphasis font-medium",
                                "Unstaked"
                            }
                            PairStakeValue {
                                shares: lp_balance.amount.parse::<u64>().unwrap_or(0),
                                boost_deposits: boost_deposits.clone(),
                                small_units: Some(true),
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
                if let Some(stake) = stake.read().as_ref() {
                    if let Ok(stake) = stake {
                        if stake.rewards > 0 {
                            OreValueSmall {
                                class: "text-elements-gold",
                                ui_amount_string: amount_to_ui_amount_string(stake.rewards, TOKEN_DECIMALS),
                                small_units: true,
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
    boost_deposits: Resource<GatewayResult<BoostDeposits>>
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
                if let Some(Ok(boost_deposits)) = boost_deposits.read().as_ref() {
                    PairValue {
                        boost_deposits: boost_deposits.clone(),
                        small_units: true,
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
                if let Some(Ok(boost_deposits)) = boost_deposits.read().as_ref() {
                    UsdValueSmall {
                        amount: boost_deposits.total_value_usd.to_string(),
                    }
                } else {
                    LoadingValue {}
                }   
            }
        }
    }
}
