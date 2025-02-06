use dioxus::prelude::*;
use ore_api::consts::TOKEN_DECIMALS;
use ore_boost_api::state::{Boost, Stake};
use solana_extra_wasm::program::spl_token::amount_to_ui_amount_string;

use crate::{components::*, gateway::{GatewayResult, UiTokenAmount}, hooks::{on_transaction_done, use_boost, use_boost_claim_transaction, use_ore_balance, use_stake}};

pub fn Idle() -> Element {
    let balance = use_ore_balance();
    let boost = use_boost(ore_api::consts::MINT_ADDRESS);
    let stake = use_stake(ore_api::consts::MINT_ADDRESS);

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
                    balance,
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
    balance: Resource<GatewayResult<UiTokenAmount>>,
) -> Element {
    // Refresh data if successful transaction
    on_transaction_done(move |_sig| {
        balance.restart();
        stake.restart();
        boost.restart();
    });

    rsx! {
        Col {
            class: "w-full h-full mx-auto max-w-2xl px-5 sm:px-8",
            gap: 8,
            Subheading {
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
            TitledRow {
                title: "Deposits (pending)",
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

#[component]
pub fn StakeYield(boost: Resource<GatewayResult<Boost>>, stake: Resource<GatewayResult<Stake>>) -> Element {
    // Build claim transaction
    let claim_tx = use_boost_claim_transaction(boost, stake);

    rsx! {
        TitledResourceRow {
            title: "Yield",
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
        }
    }
}

#[component]
fn BoostMetrics(
    boost: Resource<GatewayResult<Boost>>
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
            TotalDeposits {
                boost,
            }
            TotalStakers {
                boost,
            }
        }
    }
}

#[component]
pub fn Multiplier(boost: Resource<GatewayResult<Boost>>) -> Element {
    rsx! {
        TitledResourceRow {
            title: "Multiplier",
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