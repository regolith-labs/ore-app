use std::borrow::BorrowMut;

use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;
use solana_extra_wasm::program::spl_token::amount_to_ui_amount;

use crate::{
    components::{BackButton, OreIcon, Spinner},
    hooks::{use_gateway, use_ore_balance},
};

use super::SendStep;

#[component]
pub fn SendConfirm(
    send_step: Signal<SendStep>,
    amount: u64,
    recipient: Pubkey,
    memo: String,
) -> Element {
    let mut is_busy = use_signal(|| false);
    let mut ore_balance = use_ore_balance();
    let gateway = use_gateway();

    rsx! {
        div {
            class: "flex flex-col h-full grow gap-12",
            div {
                class: "flex flex-col gap-3",
                BackButton {
                    onclick: move |_| {
                        send_step.borrow_mut().set(SendStep::Edit);
                    }
                }
                h2 {
                    "Confirm transfer"
                }
                p {
                    class: "text-lg",
                    "Please review your transfer information for correctness."
                }
                p {
                    class: "text-sm text-gray-300 dark:text-gray-700",
                    "Once confirmed, this transaction cannot be undone."
                }
            }
            div {
                class: "flex flex-col gap-8",
                div {
                    class: "flex flex-col gap-2",
                    p {
                        "Amount"
                    }
                    div {
                        class: "flex flex-row gap-2",
                        OreIcon {
                            class: "my-auto w-5 h-5"
                        }
                        p {
                            class: "text-2xl",
                            "{amount_to_ui_amount(amount, ore_api::consts::TOKEN_DECIMALS)}"
                       }
                    }
                }
                div {
                    class: "flex flex-col gap-2",
                    p {
                        "To"
                    }
                    p {
                        class: "text-2xl",
                        "{recipient.to_string()}"
                    }
                }
                div {
                    class: "flex flex-col gap-2",
                    p {
                        "Memo"
                    }
                    p {
                        class: "text-2xl",
                        "{memo}"
                    }
                }
            }
            div {
                class: "flex flex-col mt-auto sm:flex-row gap-2",
                button {
                    class: "w-full py-3 rounded font-semibold transition-colors text-white bg-green-500 hover:bg-green-600 active:enabled:bg-green-700",
                    disabled: *is_busy.read(),
                    onclick: move |_| {
                        let gateway = gateway.clone();
                        let memo = memo.clone();
                        is_busy.set(true);
                        spawn(async move {
                            match gateway.transfer_ore(amount, recipient, memo).await {
                                Ok(sig) => {
                                    ore_balance.restart();
                                    is_busy.set(false);
                                    send_step.set(SendStep::Done);
                                }
                                Err(err) => {
                                    // TODO Handle error
                                    is_busy.set(false);
                                    log::error!("Failed to send: {:?}", err);
                                }
                            }
                        });
                    },
                    if *is_busy.read() {
                        Spinner {
                            class: "mx-auto"
                        }
                    } else {
                        "Confirm"
                    }
                }
            }
        }
    }
}
