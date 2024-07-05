use std::borrow::BorrowMut;

use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::transaction::Transaction;
use solana_extra_wasm::program::spl_token::amount_to_ui_amount;

use crate::{
    components::{BackButton, InvokeSignature, OreIcon},
    hooks::use_wallet_adapter::InvokeSignatureStatus,
};

use super::UpgradeStep;

#[component]
pub fn UpgradeConfirm(upgrade_step: Signal<UpgradeStep>, tx: Transaction, amount: u64) -> Element {
    let invoke_signature_signal = use_signal(|| InvokeSignatureStatus::Start);
    if let InvokeSignatureStatus::Done(sig) = *invoke_signature_signal.read() {
        upgrade_step.set(UpgradeStep::Done(sig));
    };
    rsx! {
        div { class: "flex flex-col h-full grow gap-12",
            div { class: "flex flex-col gap-3",
                BackButton {
                    onclick: move |_| {
                        upgrade_step.borrow_mut().set(UpgradeStep::Edit);
                    }
                }
                h2 { "Confirm upgrade" }
                p { class: "text-lg", "Please review your upgrade information for correctness." }
                p { class: "text-sm text-gray-300 dark:text-gray-700",
                    "Once confirmed, this transaction cannot be undone."
                }
            }
            div { class: "flex flex-col gap-8",
                div { class: "flex flex-col gap-2",
                    p { "Amount" }
                    div { class: "flex flex-row gap-2",
                        OreIcon { class: "my-auto w-5 h-5" }
                        p { class: "text-2xl",
                            "{amount_to_ui_amount(amount, ore_api::consts::TOKEN_DECIMALS_V1)}"
                        }
                    }
                }
            }
            div { class: "flex flex-col mt-auto sm:flex-row gap-2",
                InvokeSignature { tx: tx, signal: invoke_signature_signal, start_msg: "Upgrade" }
            }
        }
    }
}
