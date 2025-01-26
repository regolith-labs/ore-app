use dioxus::prelude::*;
use ore_api::consts::TOKEN_DECIMALS;
use solana_extra_wasm::program::spl_token::amount_to_ui_amount_string;

use crate::{components::{Col, Heading, OreValueSmallAbbreviated, OreValueSmallWhole, Row, VaultStakeForm}, hooks::{use_boost, use_stake}};

pub fn Vault() -> Element {
    rsx! {
        Col {
            class: "w-full h-full pb-20 sm:pb-16",
            gap: 8,
            Heading {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                title: "Vault",
                subtitle: "Stake unpaired ORE to earn the idle yield rate."
            }
            VaultStakeForm {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
            }
            VaultStats {}
        }
    }
}

fn VaultStats() -> Element {
    let boost = use_boost(ore_api::consts::MINT_ADDRESS);
    let stake = use_stake(ore_api::consts::MINT_ADDRESS);

    let boost_deposits = if let Some(Ok(boost)) = boost.read().as_ref() {
        amount_to_ui_amount_string(boost.total_deposits, TOKEN_DECIMALS)
    } else {
        "0.000".to_string()
    };

    let stake_balance = if let Some(Ok(stake)) = stake.read().as_ref() {
        amount_to_ui_amount_string(stake.balance, TOKEN_DECIMALS)
    } else {
        "0.000".to_string()
    };

    let stake_yield = if let Some(Ok(stake)) = stake.read().as_ref() {
        amount_to_ui_amount_string(stake.rewards, TOKEN_DECIMALS)
    } else {
        "0.000".to_string()
    };

    rsx! {
        Col {
            class: "w-full h-full mx-auto max-w-2xl px-5 sm:px-8 mt-8",
            gap: 4,
            span {
                class: "text-elements-highEmphasis font-medium text-2xl",
                "Summary"
            }
            Row {
                class: "w-full justify-between px-4",
                span {
                    class: "text-elements-lowEmphasis font-medium",
                    "Balance"
                }
                OreValueSmallAbbreviated {
                    class: "text-elements-highEmphasis",
                    ui_amount_string: stake_balance,
                }
            }
            Row {
                class: "w-full justify-between px-4",
                span {
                    class: "text-elements-lowEmphasis font-medium",
                    "Deposits"
                }
                OreValueSmallWhole {
                    class: "text-elements-highEmphasis",
                    ui_amount_string: boost_deposits,
                }
            }
            Row {
                class: "w-full justify-between px-4",
                span {
                    class: "text-elements-lowEmphasis font-medium",
                    "Yield"
                }
                OreValueSmallAbbreviated {
                    class: "text-elements-highEmphasis",
                    ui_amount_string: stake_yield,
                }
            }
        }
    }
}