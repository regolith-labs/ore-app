use dioxus::prelude::*;

use crate::{
    components::{BackButton, OreIcon, Spinner},
    hooks::use_gateway,
};

use super::ClaimStep;

#[derive(Props)]
pub struct ClaimConfirmProps<'a> {
    pub claim_step: &'a UseState<ClaimStep>,
    pub balance_handle: &'a UseFuture<()>,
    pub amount: u64,
}

#[component]
pub fn ClaimConfirm<'a>(cx: Scope<'a, ClaimConfirmProps<'a>>) -> Element {
    let is_busy = use_state(cx, || false);
    let amount = cx.props.amount;
    let balance_ = cx.props.balance_handle;
    let claim_step = cx.props.claim_step;
    let amountf = (amount as f64) / 10f64.powf(ore::TOKEN_DECIMALS.into());
    let gateway = use_gateway(cx);

    render! {
        div {
            class: "flex flex-col h-full grow justify-between p-4 sm:p-8",
            div {
                class: "flex flex-col gap-3",
                BackButton {
                    onclick: move |_| {
                        claim_step.set(ClaimStep::Edit);
                    }
                }
                h1 {
                    "Confirm claim"
                }
                p {
                    class: "text-black text-lg",
                    "Please review your claim information for correctness."
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
            }
            div {
                class: "flex flex-col sm:flex-row gap-2",
                button {
                    class: "w-full py-3 rounded font-semibold transition-colors text-white bg-green-500 hover:bg-green-600 active:enabled:bg-green-700",
                    disabled: *is_busy.get(),
                    onclick: move |_| {
                        is_busy.set(true);
                        let balance_ = balance_.clone();
                        let claim_step = claim_step.clone();
                        let is_busy = is_busy.clone();
                        let gateway = gateway.clone();
                        cx.spawn(async move {
                            match gateway.claim_ore(amount).await {
                                Ok(_sig) => {
                                    is_busy.set(false);
                                    balance_.restart();
                                    claim_step.set(ClaimStep::Done);
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
