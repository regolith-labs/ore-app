use std::str::FromStr;

use dioxus::prelude::*;
use ore_api::state::Proof;
use ore_boost_api::state::{Boost, Stake};
use ore_types::request::TransactionType;
use steel::Pubkey;

use crate::{
    components::*,
    config::{BoostMeta, LpType, LISTED_BOOSTS_BY_MINT},
    gateway::{GatewayResult, UiTokenAmount},
    hooks::{
        on_transaction_done, use_boost, use_boost_apy, use_boost_proof, use_liquidity_pair,
        use_lp_deposit_transaction, use_stake, use_token_balance,
        use_token_balances_for_liquidity_pair,
    },
    pages::{Multiplier, StakeYield, TotalStakers},
    utils::LiquidityPair,
};

#[component]
pub fn Pair(lp_mint: String) -> Element {
    let lp_mint = Pubkey::from_str(&lp_mint).unwrap();
    let boost_meta = LISTED_BOOSTS_BY_MINT.get(&lp_mint).unwrap();
    let mut liquidity_pair = use_liquidity_pair(lp_mint);
    let mut lp_balance = use_token_balance(lp_mint);
    let mut boost = use_boost(lp_mint);
    let mut boost_proof = use_boost_proof(lp_mint);
    let mut stake = use_stake(lp_mint);
    let (token_a_balance, token_b_balance) = use_token_balances_for_liquidity_pair(liquidity_pair);

    // Refresh data if successful transaction
    on_transaction_done(move |_sig| {
        stake.restart();
        boost.restart();
        boost_proof.restart();
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
                    boost_proof,
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
    boost_proof: Resource<GatewayResult<Proof>>,
    stake: Resource<GatewayResult<Stake>>,
) -> Element {
    rsx! {
        Col {
            class: "w-full h-full mx-auto max-w-2xl px-5 sm:px-8",
            gap: 0,
            Subheading {
                class: "mb-4",
                title: "Account"
            }
            Deposits {
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
                boost_proof,
                stake,
            }
        }
    }
}

#[component]
fn Deposits(
    liquidity_pair: Resource<GatewayResult<LiquidityPair>>,
    stake: Resource<GatewayResult<Stake>>,
) -> Element {
    rsx! {
        TitledRow {
            title: "Deposits",
            description: "The amount of liquidity you have deposited in the protocol. These assets are \"productive\" and automatically earn trading fees from market activity.",
            value: rsx! {
                if let Some(Ok(liquidity_pair)) = liquidity_pair.cloned() {
                    if let Some(stake) = stake.cloned() {
                        if let Ok(stake) = stake {
                            if stake.balance > 0 {
                                LiquidityPairStakeValue {
                                    class: "pb-0 sm:pb-4",
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
fn UnstakedLp(
    boost_meta: BoostMeta,
    boost: Resource<GatewayResult<Boost>>,
    lp_balance: Resource<GatewayResult<UiTokenAmount>>,
    liquidity_pair: Resource<GatewayResult<LiquidityPair>>,
    stake: Resource<GatewayResult<Stake>>,
) -> Element {
    let err = use_signal(|| None);
    let lp_deposit_tx = use_lp_deposit_transaction(boost, stake);

    rsx! {
        if let Some(Ok(liquidity_pair)) = liquidity_pair.cloned() {
            if let Some(Ok(lp_balance)) = lp_balance.cloned() {
                if lp_balance.ui_amount.unwrap_or(0.0) > 0.0 {
                    TitledRow {
                        title: "Unstaked",
                        description: format!("You have {} LP tokens that are not staked in the boost program. Deposit these tokens to earn ORE yield.", lp_balance.amount),
                        value: rsx! {
                            LiquidityPairStakeValue {
                                stake_balance: lp_balance.amount.parse::<u64>().unwrap_or(0),
                                liquidity_pair: liquidity_pair,
                                with_decimal_units: true,
                            }
                        }
                    }
                    Col {
                        class: "py-4 px-0 sm:px-3",
                        SubmitButton {
                            class: "controls-tertiary",
                            title: "Deposit {boost_meta.ticker}",
                            transaction: lp_deposit_tx,
                            err: err,
                            tx_type: TransactionType::BoostDeposit
                        }
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
    liquidity_pair: Resource<GatewayResult<LiquidityPair>>,
) -> Element {
    rsx! {
        Col {
            class: "w-full h-full mx-auto max-w-2xl px-5 sm:px-8",
            gap: 0,
            Subheading {
                class: "mb-4",
                title: "Boost"
            }
            Apy {
                boost_meta: boost_meta.clone(),
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
pub fn Apy(boost_meta: BoostMeta) -> Element {
    let apy = use_boost_apy(boost_meta.lp_mint);
    rsx! {
        TitledRow {
            title: "APY",
            description: "An estimated annualized percentage yield, derived from the trailing 7 days of returns, divided by the current notional value of all deposits in the protocol. This estimate in no way guarantees future returns.",
            value: rsx! {
                if let Ok(apy) = apy.cloned() {
                    span {
                        class: "text-elements-highEmphasis font-medium",
                        "{apy:.0}%"
                    }
                } else {
                    LoadingValue {}
                }
            }
        }
    }
}

#[component]
fn Protocol(boost_meta: BoostMeta) -> Element {
    rsx! {
        TitledRow {
            title: "Protocol",
            description: "The underlying protocol managing all deposited liquidity. This protocol deploys deposited assets into a strategy to earn trading fees from market activity.",
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
            description: "The total amount of liquidity deposited in the protocol.",
            resource: liquidity_pair,
            com: |liquidity_pair| rsx! {
                LiquidityPairValue {
                    class: "pb-0 sm:pb-4",
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
            description: "The notional value of all liquidity currently deposited in the protocol, denominated in US dollars.",
            resource: liquidity_pair,
            com: |liquidity_pair| rsx! {
                UsdValue {
                    ui_amount_string: liquidity_pair.total_value_usd.to_string(),
                }
            }
        }
    }
}
