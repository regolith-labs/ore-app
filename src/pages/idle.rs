use dioxus::prelude::*;
use ore_api::consts::TOKEN_DECIMALS;
use ore_boost_api::state::{Boost, Stake};
use solana_extra_wasm::program::spl_token::amount_to_ui_amount_string;

use crate::{components::*, gateway::{GatewayResult, UiTokenAmount}, hooks::{on_transaction_done, use_boost, use_boost_claim_transaction, use_ore_balance, use_stake}};

pub fn Idle() -> Element {
    let ore_balance = use_ore_balance();
    let ore_boost = use_boost(ore_api::consts::MINT_ADDRESS);
    let ore_stake = use_stake(ore_api::consts::MINT_ADDRESS);

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
                    ore_balance,
                    ore_stake,
                }
                AccountDetails {
                    ore_boost: ore_boost,
                    ore_balance,
                    ore_stake,
                }
                BoostDetails {
                    ore_boost,
                }
            }
        }
    }
}

#[component]
fn AccountDetails(
    ore_boost: Resource<GatewayResult<Boost>>,
    ore_balance: Resource<GatewayResult<UiTokenAmount>>,
    ore_stake: Resource<GatewayResult<Stake>>
) -> Element {
    // Refresh data if successful transaction
    on_transaction_done(move |_sig| {
        ore_balance.restart();
        ore_stake.restart();
        ore_boost.restart();
    });

    // Build claim transaction
    let claim_tx = use_boost_claim_transaction(ore_boost, ore_stake);

    rsx! {
        Col {
            class: "w-full h-full mx-auto max-w-2xl px-5 sm:px-8",
            gap: 8,
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
                if let Some(Ok(stake)) = ore_stake.read().as_ref() {
                    if stake.balance > 0 {
                        OreValue {
                            ui_amount_string: amount_to_ui_amount_string(stake.balance, TOKEN_DECIMALS),
                            with_decimal_units: true,
                            size: TokenValueSize::Small,
                        }
                    } else {
                        NullValue {}
                    }
                } else {
                    LoadingValue {}
                }
            }
            if let Some(Ok(stake)) = ore_stake.read().as_ref() {
                if stake.balance_pending > 0 {
                    Row {
                        class: "w-full justify-between px-4",
                        span {
                            class: "text-elements-lowEmphasis font-medium",
                            "Deposits (pending)"
                        }
                        OreValue {
                            ui_amount_string: amount_to_ui_amount_string(stake.balance_pending, TOKEN_DECIMALS),
                            with_decimal_units: true,
                            size: TokenValueSize::Small,
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
                if let Some(Ok(stake)) = ore_stake.read().as_ref() {
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
fn BoostDetails(
    ore_boost: Resource<GatewayResult<Boost>>
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
                    "Total deposits"
                }
                if let Some(Ok(boost)) = ore_boost.read().as_ref() {
                    OreValue {
                        ui_amount_string: amount_to_ui_amount_string(boost.total_deposits, TOKEN_DECIMALS),
                        with_decimal_units: true,
                        size: TokenValueSize::Small,
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
                if let Some(Ok(boost)) = ore_boost.read().as_ref() {
                    span {
                        class: "text-elements-highEmphasis font-medium",
                        "{boost.total_stakers}"
                    }
                } else {
                    LoadingValue {}
                }   
            }
        }
    }
}