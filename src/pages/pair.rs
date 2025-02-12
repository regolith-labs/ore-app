use std::str::FromStr;

use dioxus::prelude::*;
use ore_boost_api::state::{Boost, Stake};
use steel::Pubkey;

use crate::{
    components::*, 
    config::{BoostMeta, LpType, LISTED_BOOSTS_BY_MINT}, gateway::{GatewayResult, UiTokenAmount}, 
    hooks::{on_transaction_done, use_boost, use_liquidity_pair, use_lp_deposit_transaction, use_stake, use_token_balance, use_token_balances_for_liquidity_pair}, 
    pages::{Multiplier, StakeYield, TotalStakers},
    utils::LiquidityPair
};

#[component]
pub fn Pair(lp_mint: String) -> Element {
    let lp_mint = Pubkey::from_str(&lp_mint).unwrap();
    let boost_meta = LISTED_BOOSTS_BY_MINT.get(&lp_mint).unwrap();
    let mut liquidity_pair = use_liquidity_pair(boost_meta.clone());
    let mut lp_balance = use_token_balance(lp_mint);
    let mut boost = use_boost(lp_mint);
    let mut stake = use_stake(lp_mint);
    let (token_a_balance, token_b_balance) = use_token_balances_for_liquidity_pair(liquidity_pair);
    
    // Refresh data if successful transaction
    on_transaction_done(move |_sig| {
        stake.restart();
        boost.restart();
        liquidity_pair.restart();
        lp_balance.restart();
    });

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
                class: "gap-16",
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
                    boost,
                    stake
                }
                BoostMetrics {
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
    boost: Resource<GatewayResult<Boost>>,
    stake: Resource<GatewayResult<Stake>>,
) -> Element {
    rsx! {
        Col {
            class: "w-full h-full mx-auto max-w-2xl px-5 sm:px-8 gap-8",
            Subheading {
                title: "Account"
            }
            Deposits {
                liquidity_pair,
                stake,
            }
            PendingDeposits {
                liquidity_pair,
                stake,
            }
            UnstakedLp {
                boost_meta,
                boost,
                lp_balance,
                liquidity_pair,
                stake,
            }
            StakeYield {
                boost,
                stake,
            }
        }
    }
}

#[component]
fn Deposits(liquidity_pair: Resource<GatewayResult<LiquidityPair>>, stake: Resource<GatewayResult<Stake>>) -> Element {
    rsx! {
        TitledRow {
            title: "Deposits",
            value: rsx! {
                if let Some(Ok(liquidity_pair)) = liquidity_pair.cloned() {
                    if let Some(stake) = stake.cloned() {
                        if let Ok(stake) = stake {
                            if stake.balance > 0 {
                                LiquidityPairStakeValue {
                                    stake_balance: stake.balance,
                                    liquidity_pair: liquidity_pair,
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
        }
    }
}

#[component]
fn PendingDeposits(liquidity_pair: Resource<GatewayResult<LiquidityPair>>, stake: Resource<GatewayResult<Stake>>) -> Element {
    rsx! {
        if let Some(Ok(liquidity_pair)) = liquidity_pair.cloned() {
            if let Some(Ok(stake)) = stake.cloned() {
                if stake.balance_pending > 0 {
                    TitledRow {
                        title: "Deposits (pending)",
                        value: rsx! {
                            LiquidityPairStakeValue {
                                stake_balance: stake.balance_pending,
                                liquidity_pair: liquidity_pair,
                                with_decimal_units: true,
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn UnstakedLp(
    boost_meta: BoostMeta,
    boost: Resource<GatewayResult<Boost>>,
    lp_balance: Resource<GatewayResult<UiTokenAmount>>,
    liquidity_pair: Resource<GatewayResult<LiquidityPair>>,
    stake: Resource<GatewayResult<Stake>>
) -> Element {
    let err = use_signal(|| None);
    let lp_deposit_tx = use_lp_deposit_transaction(boost, stake);

    rsx! {
        if let Some(Ok(liquidity_pair)) = liquidity_pair.cloned() {
            if let Some(Ok(lp_balance)) = lp_balance.cloned() {
                if lp_balance.ui_amount.unwrap_or(0.0) > 0.0 {
                    TitledRow {
                        title: "Unstaked",
                        value: rsx! {
                            LiquidityPairStakeValue {
                                stake_balance: lp_balance.amount.parse::<u64>().unwrap_or(0),
                                liquidity_pair: liquidity_pair,
                                with_decimal_units: true,
                            }
                        }
                    }
                    SubmitButton {
                        class: "controls-tertiary",
                        title: "Deposit {boost_meta.ticker}",
                        transaction: lp_deposit_tx,
                        err: err,
                    }
                }
            }
        }
    }
}

#[component]
fn BoostMetrics(
    boost: Resource<GatewayResult<Boost>>,
    boost_meta: BoostMeta,
    liquidity_pair: Resource<GatewayResult<LiquidityPair>>
) -> Element {
    rsx! {
        Col {
            class: "w-full h-full mx-auto max-w-2xl px-5 sm:px-8",
            gap: 8,
            Subheading {
                title: "Boost"
            }
            Multiplier {
                boost,
            }
            Protocol {
                boost_meta,
            }
            TotalDeposits {
                liquidity_pair,
            }
            TotalStakers {
                boost,
            }
            Tvl {
                liquidity_pair,
            }
        }
    }
}

#[component]
fn Protocol(boost_meta: BoostMeta) -> Element {
    rsx! {
        TitledRow {
            title: "Protocol",
            value: rsx! {
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
        }
    }
}

#[component]
fn TotalDeposits(liquidity_pair: Resource<GatewayResult<LiquidityPair>>) -> Element {
    rsx! {
        TitledResourceRow {
            title: "Total deposits",
            resource: liquidity_pair,
            com: |liquidity_pair| rsx! {
                LiquidityPairValue {
                    liquidity_pair: liquidity_pair.clone(),
                    with_decimal_units: true,
                }
            }
        }  
    }
}

#[component]
fn Tvl(liquidity_pair: Resource<GatewayResult<LiquidityPair>>) -> Element {
    rsx! {
        TitledResourceRow {
            title: "TVL",
            resource: liquidity_pair,
            com: |liquidity_pair| rsx! {
                UsdValue {
                    ui_amount_string: liquidity_pair.total_value_usd.to_string(),
                }
            }
        }
    }
}