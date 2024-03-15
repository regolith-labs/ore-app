mod send_confirm;
mod send_done;
mod send_edit;

use std::str::FromStr;

use dioxus::prelude::*;
use send_confirm::*;
use send_done::*;
use send_edit::*;
#[cfg(feature = "web")]
use solana_client_wasm::solana_sdk::pubkey::Pubkey;
#[cfg(feature = "desktop")]
use solana_sdk::pubkey::Pubkey;

pub enum SendStep {
    Edit,
    Confirm,
    Done,
}

#[component]
pub fn Send(cx: Scope, to: Option<String>) -> Element {
    let send_step = use_state(cx, || SendStep::Edit);
    let amount_input = use_state(cx, || "".to_string());
    let recipient_input = use_state(cx, || to.clone().unwrap_or("".to_string()));
    let memo_input = use_state(cx, || "".to_string());

    let parsed_amount: u64 = match amount_input.get().parse::<f64>() {
        Ok(n) => (n * 10f64.powf(ore::TOKEN_DECIMALS.into())) as u64,
        Err(_) => 0,
    };

    let recipient = Pubkey::from_str(recipient_input.get()).ok();

    let memo = memo_input.get().trim().to_string();

    render! {
        div {
            class: "flex flex-col grow gap-y-8 my-auto",
            match send_step.get() {
                SendStep::Edit => {
                    render! {
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
                    render! {
                        SendConfirm {
                            send_step: send_step,
                            amount: parsed_amount,
                            memo: memo,
                            recipient: recipient.unwrap(),
                        }
                    }
                }
                SendStep::Done => {
                    render! {
                        SendDone {}
                    }
                }
            }
        }
    }
}
