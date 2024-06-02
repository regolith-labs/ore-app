mod confirm;
mod done;
mod edit;
mod preview;

use confirm::*;
use done::*;
use edit::*;
use preview::*;

use dioxus::prelude::*;

use crate::{gateway::AsyncResult, hooks::use_proof};

pub enum ClaimStep {
    Edit,
    Confirm,
    Done,
}

pub fn Claim() -> Element {
    let proof = *use_proof().read();
    let claim_step = use_signal(|| ClaimStep::Edit);
    let amount_input = use_signal(|| "".to_string());
    let parsed_amount: u64 = match amount_input.read().parse::<f64>() {
        Ok(n) => (n * 10f64.powf(ore::TOKEN_DECIMALS.into())) as u64,
        Err(_) => 0,
    };

    let amount = match &proof {
        AsyncResult::Ok(proof) => proof.balance,
        _ => 0,
    };

    let e = match *claim_step.read() {
        ClaimStep::Edit => {
            rsx! {
                ClaimEdit {
                    claim_step: claim_step,
                    amount_input: amount_input,
                    max_rewards: amount,
                    parsed_amount: parsed_amount,
                }
            }
        }
        ClaimStep::Confirm => {
            rsx! {
                ClaimConfirm {
                    claim_step: claim_step.clone(),
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
