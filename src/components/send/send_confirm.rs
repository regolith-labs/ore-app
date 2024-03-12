use dioxus::prelude::*;
#[cfg(feature = "web")]
use solana_client_wasm::solana_sdk::pubkey::Pubkey;
#[cfg(feature = "desktop")]
use solana_sdk::pubkey::Pubkey;

use crate::{
    components::{BackButton, OreIcon, Spinner},
    hooks::{use_gateway, use_ore_balance_handle},
};

use super::SendStep;

#[derive(Props)]
pub struct SendConfirmProps<'a> {
    pub send_step: &'a UseState<SendStep>,
    pub amount: u64,
    pub recipient: Pubkey,
    pub memo: String,
}

#[component]
pub fn SendConfirm<'a>(cx: Scope<'a, SendConfirmProps<'a>>) -> Element {
    let is_busy = use_state(cx, || false);
    let recipient = cx.props.recipient;
    let amount = cx.props.amount;
    let send_step = cx.props.send_step;
    let memo = cx.props.memo.clone();
    let memo_ = memo.clone();
    let amountf = (cx.props.amount as f64) / 10f64.powf(ore::TOKEN_DECIMALS.into());
    let gateway = use_gateway(cx);
    let balance_ = use_ore_balance_handle(cx);

    render! {
        div {
            class: "flex flex-col h-full grow gap-12",
            div {
                class: "flex flex-col gap-3",
                BackButton {
                    onclick: move |_| {
                        send_step.set(SendStep::Edit);
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
                            "{amountf}"
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
                        "{memo_}"
                    }
                }
            }
            div {
                class: "flex flex-col mt-auto sm:flex-row gap-2",
                button {
                    class: "w-full py-3 rounded font-semibold transition-colors text-white bg-green-500 hover:bg-green-600 active:enabled:bg-green-700",
                    disabled: *is_busy.get(),
                    onclick: move |_| {
                        is_busy.set(true);
                        let balance_ = balance_.clone();
                        let memo = memo.clone();
                        let send_step = send_step.clone();
                        let is_busy = is_busy.clone();
                        let gateway = gateway.clone();
                        cx.spawn(async move {
                            match gateway.transfer_ore(amount, recipient, memo).await {
                                Ok(sig) => {
                                    log::info!("Transfer: {:?}", sig);
                                    balance_.restart();
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
                    if *is_busy.get() {
                        render! {
                            Spinner {
                                class: "mx-auto"
                            }
                        }
                    } else {
                        render! {
                            "Confirm"
                        }
                    }
                }
            }
        }
    }
}
