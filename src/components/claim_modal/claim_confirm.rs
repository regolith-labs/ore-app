use dioxus::prelude::*;

use crate::{
    components::{BackButton, OreIcon, Spinner},
    hooks::{use_gateway, use_priority_fee, use_pubkey, BalanceHandle, PriorityFee},
    metrics::{track, AppEvent},
    ProofHandle,
};

use super::ClaimStep;

#[component]
pub fn ClaimConfirm(cx: Scope, amount: u64, claim_step: UseState<ClaimStep>) -> Element {
    let is_busy = use_state(cx, || false);
    let balance_ = use_context::<BalanceHandle>(cx).unwrap();
    let pubkey = use_pubkey(cx);
    let proof_ = use_context::<ProofHandle>(cx).unwrap();
    let amountf = (*amount as f64) / 10f64.powf(ore::TOKEN_DECIMALS.into());
    let gateway = use_gateway(cx);
    let priority_fee = use_priority_fee(cx);

    render! {
        div {
            class: "flex flex-col h-full grow justify-between",
            div {
                class: "flex flex-col gap-3",
                BackButton {
                    onclick: move |_| {
                        claim_step.set(ClaimStep::Edit);
                    }
                }
                h2 {
                    "Confirm claim"
                }
                p {
                    class: "text-lg",
                    "Please review your claim information for correctness."
                }
                p {
                    class: "text-sm text-gray-300 dark:text-gray-700",
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
                class: "flex flex-col gap-8",
                div {
                    class: "flex flex-row gap-8 justify-between mt-8",
                    div {
                        class: "flex flex-col gap-1",
                        p {
                            class: "font-semibold",
                            "Priority fee"
                        }
                        p {
                            class: "text-xs opacity-80 max-w-96",
                            "When Solana is busy, priority fees can increase the chances of your transactions being accepted."
                        }
                    }
                    div {
                        class: "flex flex-row flex-shrink h-min gap-1 shrink mb-auto",
                        input {
                            disabled: *is_busy.get(),
                            class: "bg-transparent text-right px-1 mb-auto",
                            step: 100_000,
                            min: 0,
                            max: 50_000_000,
                            r#type: "number",
                            value: "{priority_fee.read().0}",
                            oninput: move |e| {
                                if let Ok(v) = e.value.parse::<u64>() {
                                    track(AppEvent::SetPriorityFee, None);
                                    *priority_fee.write() = PriorityFee(v);
                                }
                            }
                        }
                        p {
                            class: "my-auto",
                            "microlamports"
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
                            let proof_ = proof_.clone();
                            let amount = *amount;
                            let claim_step = claim_step.clone();
                            let is_busy = is_busy.clone();
                            let gateway = gateway.clone();
                            let priority_fee = priority_fee.clone();
                            cx.spawn({
                                async move {
                                    // Create associated token account, if needed
                                    'ata: loop {
                                        match gateway
                                            .create_token_account_ore(pubkey)
                                            .await
                                        {
                                                Ok(_) => break 'ata,
                                                Err(err) => log::error!("Failed to create token account: {:?}", err),
                                        }
                                    }

                                    // Claim
                                    match gateway.claim_ore(amount, priority_fee.read().0).await {
                                        Ok(_sig) => {
                                            track(AppEvent::Claim, None);
                                            balance_.restart();
                                            proof_.restart();
                                            is_busy.set(false);
                                            claim_step.set(ClaimStep::Done);
                                        }
                                        Err(_err) => {
                                            // TODO Handle error
                                            is_busy.set(false);
                                            log::error!("Failed to claim!");
                                        }
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
}
