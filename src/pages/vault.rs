use dioxus::prelude::*;
use ore_api::consts::TOKEN_DECIMALS;
use solana_extra_wasm::program::spl_token::amount_to_ui_amount_string;

use crate::{components::{Col, Heading, OreValue, OreValueSmall, Row, VaultStakeForm}, hooks::{use_boost, use_stake}};

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
            VaultStats {}
            VaultStakeForm {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
            }
        }
    }
}

fn VaultStats() -> Element {
    let boost = use_boost(ore_api::consts::MINT_ADDRESS);
    let stake = use_stake(ore_api::consts::MINT_ADDRESS);

    let stake_balance = if let Some(Ok(stake)) = stake.read().as_ref() {
        amount_to_ui_amount_string(stake.balance, TOKEN_DECIMALS)
    } else {
        "0.000".to_string()
    };

    rsx! {
        Col {
            class: "w-full h-full mx-auto max-w-2xl px-5 sm:px-8",
            gap: 8,
            Row {
                class: "w-full justify-between",
                span {
                    class: "text-elements-midEmphasis font-semibold",
                    "Deposits"
                }
                OreValueSmall {
                    class: "text-elements-highEmphasis",
                    ui_amount_string: stake_balance,
                }
            }
            // Col {
            //     gap: 2,
                
            //     span {
            //         class: "text-elements-midEmphasis",
            //         "Deposits"
            //     }
            // }
        }
    }
}