use crate::{
    components::*,
    gateway::GatewayResult,
    hooks::{
        use_boost_apr, use_boost_claim_transaction, use_boost_wss, use_claimable_yield,
        use_ore_balance_wss, use_ore_price, use_stake_wss,
    },
    solana::spl_token::{amount_to_ui_amount, amount_to_ui_amount_string},
};
use dioxus::prelude::*;
use ore_api::consts::TOKEN_DECIMALS;
use ore_boost_api::state::{Boost, Stake};
use ore_types::request::TransactionType;

pub fn Idle() -> Element {
    let balance = use_ore_balance_wss();
    let boost = use_boost_wss(ore_api::consts::MINT_ADDRESS);
    let stake = use_stake_wss(ore_api::consts::MINT_ADDRESS);

    rsx! {
        Col {
            class: "w-full h-full pb-20 sm:pb-16",
            gap: 8,
            Heading {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                title: "Stake ORE",
                subtitle: "Stake unpaired ORE and earn the idle yield rate."
            }
            Col {
                gap: 16,
                IdleStakeForm {
                    class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                    balance,
                    stake,
                }
                AccountMetrics {
                    boost,
                    stake,
                }
                BoostMetrics {
                    boost,
                }
            }
        }
    }
}

#[component]
fn AccountMetrics(
    boost: Signal<GatewayResult<Boost>>,
    stake: Signal<GatewayResult<Stake>>,
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
fn Deposits(stake: Signal<GatewayResult<Stake>>) -> Element {
    rsx! {
        TitledSignalRow {
            title: "Deposits",
            description: "The amount of ORE you have deposited in the protocol. This ORE is \"idle\" and thus earns the native idle yield rate.",
            signal: stake,
            com: |stake| {
                rsx! {
                    if stake.balance > 0 {
                        OreValue {
                            ui_amount_string: amount_to_ui_amount_string(stake.balance, TOKEN_DECIMALS),
                            with_decimal_units: true,
                            size: TokenValueSize::Small,
                        }
                    } else {
                        NullValue {}
                    }
                }
            }
        }
    }
}

#[component]
pub fn StakeYield(
    boost: Signal<GatewayResult<Boost>>,
    stake: Signal<GatewayResult<Stake>>,
) -> Element {
    // Build claim transaction
    let claimable_yield = use_claimable_yield(boost, stake);
    let claim_tx = use_boost_claim_transaction(boost, stake);

    rsx! {
        TitledRow {
            title: "Yield",
            description: "The amount of ORE you have earned and may claim. Yield is not automatically compounded.",
            value: rsx! {
                if *claimable_yield.read() > 0 {
                    OreValue {
                        ui_amount_string: amount_to_ui_amount_string(*claimable_yield.read(), TOKEN_DECIMALS),
                        with_decimal_units: true,
                        size: TokenValueSize::Small,
                        gold: true,
                    }
                } else {
                    NullValue {}
                }
            }
        }
        ClaimButton {
            transaction: claim_tx.clone(),
            tx_type: TransactionType::BoostClaim,
        }
    }
}

#[component]
fn BoostMetrics(boost: Signal<GatewayResult<Boost>>) -> Element {
    rsx! {
        Col {
            class: "w-full h-full mx-auto max-w-2xl px-5 sm:px-8",
            gap: 0,
            Subheading {
                class: "mb-4",
                title: "Boost"
            }
            Apr {}
            TotalDeposits {
                boost,
            }
            TotalStakers {
                boost,
            }
            Tvl {
                boost,
            }
            Weight {
                boost,
            }
        }
    }
}

#[component]
pub fn Apr() -> Element {
    let apr = use_boost_apr(ore_api::consts::MINT_ADDRESS);
    rsx! {
        TitledRow {
            title: "APR",
            description: "The annualized percentage rate returned to stakers, derived from the last 7 days of yield divided by the current notional value of total deposits in the protocol. This estimate in no way guarantees future returns.",
            value: rsx! {
                if let Ok(apr) = apr.cloned() {
                    span {
                        class: "text-elements-highEmphasis font-medium",
                        "{apr:.0}%"
                    }
                } else {
                    LoadingValue {}
                }
            }
        }
    }
}

#[component]
pub fn Weight(boost: Signal<GatewayResult<Boost>>) -> Element {
    rsx! {
        TitledSignalRow {
            title: "Weight",
            description: "The weight given to this boost. The higher the weight, the more ORE will be shared with stakers in the protocol.",
            signal: boost,
            com: |boost| rsx! {
                span {
                    class: "text-elements-highEmphasis font-medium",
                    "{boost.weight as f64}"
                }
            }
        }
    }
}

#[component]
fn TotalDeposits(boost: Signal<GatewayResult<Boost>>) -> Element {
    rsx! {
        TitledSignalRow {
            title: "Total deposits",
            description: "The total amount of ORE currently deposited in this protocol.",
            signal: boost,
            com: |boost| {
                rsx! {
                    OreValue {
                        ui_amount_string: amount_to_ui_amount_string(boost.total_deposits, TOKEN_DECIMALS),
                        with_decimal_units: true,
                        size: TokenValueSize::Small,
                    }
                }
            }
        }
    }
}

#[component]
pub fn TotalStakers(boost: Signal<GatewayResult<Boost>>) -> Element {
    rsx! {
        TitledSignalRow {
            title: "Total stakers",
            description: "The total number of stakers participating in the protocol.",
            signal: boost,
            com: |boost| {
                rsx! {
                    span {
                        class: "text-elements-highEmphasis font-medium",
                        "{boost.total_stakers}"
                    }
                }
            }
        }
    }
}

#[component]
fn Tvl(boost: Signal<GatewayResult<Boost>>) -> Element {
    let ore_price = use_ore_price();
    rsx! {
        TitledRow {
            title: "TVL",
            description: "The notional value of all ORE currently deposited in this protocol, denominated in US dollars.",
            value: rsx! {
                if let Some(ore_price) = ore_price.cloned() {
                    if let Ok(boost) = boost.cloned() {
                        UsdValue {
                            ui_amount_string: (amount_to_ui_amount(boost.total_deposits, TOKEN_DECIMALS) * ore_price.0).to_string(),
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
