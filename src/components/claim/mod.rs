mod confirm;
mod done;
mod edit;

use confirm::*;
use done::*;
use edit::*;

use dioxus::prelude::*;

use crate::hooks::{use_escrow_proof, use_proof};

pub enum ClaimStep {
    Edit,
    Confirm,
    Done,
}

pub fn Claim() -> Element {
    let proof = use_proof();
    let escrow_proof = use_escrow_proof();
    let claim_step = use_signal(|| ClaimStep::Edit);
    let amount_input = use_signal(|| "".to_string());
    let parsed_amount: u64 = match amount_input.read().parse::<f64>() {
        Ok(n) => (n * 10f64.powf(ore_api::consts::TOKEN_DECIMALS.into())) as u64,
        Err(_) => 0,
    };

    // TODO Calc if escrow proof is present

    let max_rewards = escrow_proof
        .read()
        .and_then(|p| p.ok())
        .map(|p| p.balance)
        .unwrap_or_else(|| {
            proof
                .read()
                .and_then(|p| p.ok())
                .map(|p| p.balance)
                .unwrap_or_else(|| 0)
        });

    let e = match *claim_step.read() {
        ClaimStep::Edit => {
            rsx! {
                ClaimEdit {
                    claim_step,
                    amount_input,
                    max_rewards,
                    parsed_amount,
                }
            }
        }
        ClaimStep::Confirm => {
            rsx! {
                ClaimConfirm {
                    claim_step,
                    amount: parsed_amount,
                }
            }
        }
        ClaimStep::Done => {
            rsx! {
                ClaimDone {}
            }
        }
    };

    e
}
