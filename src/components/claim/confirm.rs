use std::borrow::BorrowMut;

use dioxus::prelude::*;
use solana_extra_wasm::program::spl_token::amount_to_ui_amount;

use crate::{
    components::{BackButton, OreIcon, Spinner},
    hooks::{use_gateway, use_ore_balance, use_priority_fee, use_proof, use_pubkey, PriorityFee},
};

use super::ClaimStep;

#[component]
pub fn ClaimConfirm(amount: u64, claim_step: Signal<ClaimStep>) -> Element {
    let mut is_busy = use_signal(|| false);
    let mut priority_fee = use_priority_fee();
    let mut balance = use_ore_balance();
    let mut proof = use_proof();
    let pubkey = use_pubkey();
    let gateway = use_gateway();

    rsx! {
        div {
            class: "flex flex-col h-full grow justify-between",
            div {
                class: "flex flex-col gap-3",
                BackButton {
                    onclick: move |_| {
                        claim_step.borrow_mut().set(ClaimStep::Edit);
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
                        "{amount_to_ui_amount(amount, ore_api::consts::TOKEN_DECIMALS)}"
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
                            "Add a priority fee to increase your chances of landing a transaction."
                        }
                    }
                    div {
                        class: "flex flex-row flex-shrink h-min gap-1 shrink mb-auto",
                        input {
                            disabled: *is_busy.read(),
                            class: "bg-transparent text-right px-1 mb-auto font-semibold",
                            dir: "rtl",
                            step: 100_000,
                            min: 0,
                            max: 50_000_000,
                            r#type: "number",
                            value: "{priority_fee.read().0}",
                            oninput: move |e| {
                                if let Ok(v) = e.value().parse::<u64>() {
                                    priority_fee.set(PriorityFee(v));
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
                        disabled: *is_busy.read(),
                        onclick: move |_| {
                            is_busy.set(true);
                            let gateway = gateway.clone();
                            spawn({
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
                                            balance.restart();
                                            proof.restart();
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
}
