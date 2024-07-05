mod confirm;
mod done;
mod edit;

use confirm::*;
use done::*;
use edit::*;

use std::str::FromStr;

use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;

pub enum SendStep {
    Edit,
    Confirm,
    Done,
}

#[component]
pub fn Send(to: Option<String>) -> Element {
    let send_step = use_signal(|| SendStep::Edit);
    let amount_input = use_signal(|| "".to_string());
    let memo_input = use_signal(|| "".to_string());
    let memo = memo_input.read().trim().to_string();
    let recipient_input = use_signal(|| to.clone().unwrap_or("".to_string()));
    let recipient = Pubkey::from_str(&recipient_input.read()).ok();
    let parsed_amount: u64 = match amount_input.read().parse::<f64>() {
        Ok(n) => (n * 10f64.powf(ore_api::consts::TOKEN_DECIMALS.into())) as u64,
        Err(_) => 0,
    };

    let e = match *send_step.read() {
        SendStep::Edit => {
            rsx! {
                SendEdit {
                    send_step: send_step,
                    parsed_amount: parsed_amount,
                    amount_input: amount_input,
                    recipient_input: recipient_input,
                    memo_input: memo_input
                }
            }
        }
        SendStep::Confirm => {
            rsx! {
                SendConfirm {
                    send_step: send_step,
                    amount: parsed_amount,
                    memo: memo,
                    recipient: recipient.unwrap(),
                }
            }
        }
        SendStep::Done => {
            rsx! {
                SendDone {}
            }
        }
    };

    e
}
