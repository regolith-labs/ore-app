mod confirm;
mod done;
mod edit;

use confirm::*;
use done::*;
use edit::*;

use dioxus::prelude::*;

pub enum StakeStep {
    Edit,
    Confirm,
    Done,
}

pub fn Stake() -> Element {
    let step = use_signal(|| StakeStep::Edit);
    let amount_input = use_signal(|| "".to_string());
    let parsed_amount: u64 = match amount_input.read().parse::<f64>() {
        Ok(n) => (n * 10f64.powf(ore_api::consts::TOKEN_DECIMALS.into())) as u64,
        Err(_) => 0,
    };

    let e = match *step.read() {
        StakeStep::Edit => {
            rsx! {
                StakeEdit {
                    step,
                    amount_input,
                    parsed_amount,
                }
            }
        }
        StakeStep::Confirm => {
            rsx! {
                StakeConfirm {
                    step,
                    amount: parsed_amount,
                }
            }
        }
        StakeStep::Done => {
            rsx! {
                StakeDone {}
            }
        }
    };

    e
}
