mod confirm;
mod done;
mod edit;

use confirm::*;
use done::*;
use edit::*;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;

use crate::staker::{ISC_LP_MINT, ISC_LP_MINT_DECIMALS};

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
    let step = use_signal(|| StakeStep::Edit);
    let amount_input = use_signal(|| "".to_string());
    let parsed_amount: u64 = match amount_input.read().parse::<f64>() {
        Ok(n) => (n * 10f64.powf(ore_api::consts::TOKEN_DECIMALS.into())) as u64,
        Err(_) => 0,
    };
    let isc_stake = Stake {
        mint: ISC_LP_MINT,
        decimals: ISC_LP_MINT_DECIMALS,
        name: "ISC LP".to_string(),
    };

    let e = match *step.read() {
        StakeStep::Edit => {
            rsx! {
                StakeEdit {
                    step,
                    amount_input,
                    parsed_amount,
                    stake: isc_stake
                }
            }
        }
        StakeStep::Confirm => {
            rsx! {
                StakeConfirm {
                    step,
                    amount: parsed_amount,
                    stake: isc_stake,
                }
            }
        }
        StakeStep::Done => {
            rsx! {
                StakeDone {
                    stake: isc_stake
                }
            }
        }
    };

    e
}
