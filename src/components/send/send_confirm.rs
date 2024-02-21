use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;

use crate::{
    components::{BackButton, OreIcon, Spinner},
    hooks::use_gateway,
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

    render! {
        div {
            class: "flex flex-col h-full grow justify-between",
            div {
                class: "flex flex-col gap-3",
                BackButton {
                    onclick: move |_| {
                        send_step.set(SendStep::Edit);
                    }
                }
                h1 {
                    "Confirm transfer"
                }
                p {
                    class: "text-black text-lg",
                    "Please review your transfer information for correctness."
                }
                p {
                    class: "text-gray-300 text-sm",
                    "Once confirmed, this transaction cannot be undone."
                }
            }
            div {
                class: "flex flex-col gap-8",
                div {
                    class: "flex flex-row gap-2.5 md:gap-4 mx-auto",
                    OreIcon {
                        class: "my-auto w-7 h-7 sm:w-8 sm:h-8 md:w-10 md:h-10"
                    }
                    p {
                        class: "text-3xl sm:text-4xl md:text-5xl font-semibold",
                        "{amountf}"
                    }
                }
                div {
                    class: "flex flex-row gap-2.5 md:gap-4 mx-auto",
                    p {
                        class: "text-3xl sm:text-4xl md:text-5xl font-semibold",
                        "{recipient.to_string()}"
                    }
                }
                div {
                    class: "flex flex-row gap-2.5 md:gap-4 mx-auto",
                    p {
                        class: "text-3xl sm:text-4xl md:text-5xl font-semibold",
                        "{memo_}"
                    }
                }
            }
            div {
                class: "flex flex-col sm:flex-row gap-2",
                button {
                    class: "w-full py-3 rounded font-semibold transition-colors text-white bg-green-500 hover:bg-green-600 active:enabled:bg-green-700",
                    disabled: *is_busy.get(),
                    onclick: move |_| {
                        is_busy.set(true);
                        // let balance_ = balance_.clone();
                        let memo = memo.clone();
                        let send_step = send_step.clone();
                        let is_busy = is_busy.clone();
                        let gateway = gateway.clone();
                        cx.spawn(async move {
                            match gateway.transfer_ore(amount, recipient, memo).await {
                                Ok(sig) => {
                                    log::info!("Transfer: {:?}", sig);
                                    is_busy.set(false);
                                    send_step.set(SendStep::Done);
                                }
                                Err(_err) => {
                                    // TODO Handle error
                                    is_busy.set(false);
                                    log::error!("Failed to claim!");
                                }
                            }
                        });
                    },
                    if *is_busy.get() {
                        render! {
                            Spinner {}
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
