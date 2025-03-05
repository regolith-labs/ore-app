use crate::{
    components::*,
    gateway::GatewayResult,
    hooks::{
        on_transaction_done, use_boost, use_boost_apy, use_boost_claim_transaction,
        use_ore_balance, use_ore_price, use_stake,
    },
    solana::spl_token::{amount_to_ui_amount, amount_to_ui_amount_string},
};
use dioxus::prelude::*;
use ore_api::consts::TOKEN_DECIMALS;
use ore_boost_api::state::{Boost, Stake};
use ore_types::request::TransactionType;

pub fn Idle() -> Element {
    let mut balance = use_ore_balance();
    let mut boost = use_boost(ore_api::consts::MINT_ADDRESS);
    let mut stake = use_stake(ore_api::consts::MINT_ADDRESS);

    // Refresh data if successful transaction
    on_transaction_done(move |_sig| {
        balance.restart();
        stake.restart();
        boost.restart();
    });

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
    boost: Resource<GatewayResult<Boost>>,
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
                stake,
            }
            PendingDeposits {
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
fn Deposits(stake: Resource<GatewayResult<Stake>>) -> Element {
    rsx! {
        TitledResourceRow {
            title: "Deposits",
            description: "The amount of ORE you have deposited in the boost. This ORE is \"idle\" and thus earns the native idle yield rate.",
            resource: stake,
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
fn PendingDeposits(stake: Resource<GatewayResult<Stake>>) -> Element {
    rsx! {
        if let Some(Ok(stake)) = stake.cloned() {
            if stake.balance_pending > 0 {
                TitledRow {
                    title: "Deposits (pending)",
                    description: "The amount of ORE you have deposited in the boost that is pending to be committed. Pending deposits are automatically committed approximately every hour.",
                    value: rsx! {
                        OreValue {
                            ui_amount_string: amount_to_ui_amount_string(stake.balance_pending, TOKEN_DECIMALS),
                            with_decimal_units: true,
                            size: TokenValueSize::Small,
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn StakeYield(
    boost: Resource<GatewayResult<Boost>>,
    stake: Resource<GatewayResult<Stake>>,
) -> Element {
    // Build claim transaction
    let claim_tx = use_boost_claim_transaction(boost, stake);

    rsx! {
        TitledResourceRow {
            title: "Yield",
            description: "The amount of ORE you have earned and may now claim. Yield is not automatically compounded.",
            resource: stake,
            com: |stake| {
                rsx! {
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
fn BoostMetrics(boost: Resource<GatewayResult<Boost>>) -> Element {
    rsx! {
        Col {
            class: "w-full h-full mx-auto max-w-2xl px-5 sm:px-8",
            gap: 0,
            Subheading {
                class: "mb-4",
                title: "Boost"
            }
            Apy {}
            Multiplier {
                boost,
            }
            TotalDeposits {
                boost,
            }
            TotalStakers {
                boost,
            }
            Tvl {
                boost,
            }
        }
    }
}

#[component]
pub fn Apy() -> Element {
    let apy = use_boost_apy(ore_api::consts::MINT_ADDRESS);
    rsx! {
        TitledRow {
            title: "APY",
            description: "An annualized percentage yield derived from the last 7 days of trailing returns divided by the total value of deposits currently in the protocol. This estimate in no way guarantees future returns.",
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
pub fn Multiplier(boost: Resource<GatewayResult<Boost>>) -> Element {
    rsx! {
        TitledResourceRow {
            title: "Multiplier",
            description: "The multiplier is an indicator of this boost's priority relative to other LP protocols that receive ORE yield. The higher the multiplier, the more ORE will be allocated for stakers in the protocol.",
            resource: boost,
            com: |boost| rsx! {
                span {
                    class: "text-elements-highEmphasis font-medium",
                    "{boost.multiplier as f64 / ore_boost_api::consts::BOOST_DENOMINATOR as f64}x"
                }
            }
        }
    }
}

#[component]
fn TotalDeposits(boost: Resource<GatewayResult<Boost>>) -> Element {
    rsx! {
        TitledResourceRow {
            title: "Total deposits",
            description: "The total amount of ORE deposited by stakers in the protocol.",
            resource: boost,
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
pub fn TotalStakers(boost: Resource<GatewayResult<Boost>>) -> Element {
    rsx! {
        TitledResourceRow {
            title: "Total stakers",
            description: "The total number of stakers participating in the protocol.",
            resource: boost,
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
fn Tvl(boost: Resource<GatewayResult<Boost>>) -> Element {
    let ore_price = use_ore_price();
    rsx! {
        TitledRow {
            title: "TVL",
            description: "The current notional value of all ORE deposited in the protocol, denominated in US dollars.",
            value: rsx! {
                if let Some(ore_price) = ore_price.cloned() {
                    if let Some(Ok(boost)) = boost.cloned() {
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
