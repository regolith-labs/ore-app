mod confirm;
mod done;
mod edit;

use confirm::*;
use done::*;
use edit::*;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;

use crate::{
    components::BackButton,
    staker::{ISC_LP_MINT, ISC_LP_MINT_DECIMALS},
};

use dioxus::prelude::*;

pub enum StakeStep {
    Edit,
    Confirm,
    Done,
}

#[derive(PartialEq, Clone)]
pub struct Stake {
    pub mint: Pubkey,
    pub decimals: u8,
    pub name: String,
}

pub fn Stake() -> Element {
    let nav = navigator();
    let step = use_signal(|| StakeStep::Edit);
    let amount_input_ore = use_signal(|| "".to_string());
    let amount_input_isc = use_signal(|| "".to_string());
    let amount_input_usdc = use_signal(|| "".to_string());
    let parsed_amount_ore: u64 = match amount_input_ore.read().parse::<f64>() {
        Ok(n) => (n * 10f64.powf(ore_api::consts::TOKEN_DECIMALS.into())) as u64,
        Err(_) => 0,
    };
    let parsed_amount_isc: u64 = match amount_input_isc.read().parse::<f64>() {
        Ok(n) => (n * 10f64.powf(ore_api::consts::TOKEN_DECIMALS.into())) as u64,
        Err(_) => 0,
    };
    let parsed_amount_usdc: u64 = match amount_input_usdc.read().parse::<f64>() {
        Ok(n) => (n * 10f64.powf(ore_api::consts::TOKEN_DECIMALS.into())) as u64,
        Err(_) => 0,
    };
    let stake_ore = Stake {
        mint: ore_api::consts::MINT_ADDRESS,
        decimals: ore_api::consts::TOKEN_DECIMALS,
        name: "ISC LP".to_string(),
    };
    let stake_isc = Stake {
        mint: ISC_LP_MINT,
        decimals: ISC_LP_MINT_DECIMALS,
        name: "ISC LP".to_string(),
    };
    let stake_usdc = Stake {
        mint: ISC_LP_MINT,
        decimals: ISC_LP_MINT_DECIMALS,
        name: "USDC LP".to_string(),
    };

    let e = match *step.read() {
        StakeStep::Edit => {
            rsx! {
                div {
                    class: "flex flex-col h-full grow justify-between gap-6",
                    BackButton {
                        onclick: move |_| {
                            nav.go_back()
                        }
                    }
                    div {
                        class: "flex flex-col gap-3",
                        h2 {
                            "Stake"
                        }
                    }
                    StakeEdit {
                        step,
                        amount_input: amount_input_ore,
                        parsed_amount: parsed_amount_ore,
                        stake: stake_ore
                    }
                    StakeEdit {
                        step,
                        amount_input: amount_input_isc,
                        parsed_amount: parsed_amount_isc,
                        stake: stake_isc
                    }
                    StakeEdit {
                        step,
                        amount_input: amount_input_usdc,
                        parsed_amount: parsed_amount_usdc,
                        stake: stake_usdc
                    }
                }
            }
        }
        // TODO: signal for parsed amount
        StakeStep::Confirm => {
            rsx! {
                StakeConfirm {
                    step,
                    amount: parsed_amount_isc,
                    stake: stake_isc,
                }
            }
        }
        StakeStep::Done => {
            rsx! {
                StakeDone {
                    stake: stake_isc
                }
            }
        }
    };

    e
}
