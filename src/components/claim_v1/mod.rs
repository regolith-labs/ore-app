mod confirm;
mod done;
mod edit;

use confirm::*;
use done::*;
use edit::*;

use dioxus::prelude::*;

use crate::hooks::use_proof_v1;

pub enum ClaimV1Step {
    Edit,
    Confirm,
    Done,
}

pub fn ClaimV1() -> Element {
    let proof = use_proof_v1();
    let claim_step = use_signal(|| ClaimV1Step::Edit);
    let amount_input = use_signal(|| "".to_string());
    let parsed_amount: u64 = match amount_input.read().parse::<f64>() {
        Ok(n) => (n * 10f64.powf(ore_api_v1::TOKEN_DECIMALS.into())) as u64,
        Err(_) => 0,
    };
    let max_rewards = proof
        .read()
        .and_then(|p| p.ok())
        .map(|p| p.claimable_rewards)
        .unwrap_or_else(|| 0);

    let e = match *claim_step.read() {
        ClaimV1Step::Edit => {
            rsx! {
                ClaimV1Edit {
                    claim_step,
                    amount_input,
                    max_rewards,
                    parsed_amount,
                }
            }
        }
        ClaimV1Step::Confirm => {
            rsx! {
                ClaimV1Confirm {
                    claim_step,
                    amount: parsed_amount,
                }
            }
        }
        ClaimV1Step::Done => {
            rsx! {
                ClaimV1Done {}
            }
        }
    };

    e
}
