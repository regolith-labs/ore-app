use std::borrow::BorrowMut;

use dioxus::prelude::*;
use solana_extra_wasm::program::spl_token::amount_to_ui_amount;

use crate::{
    components::{BackButton, OreIcon, Spinner},
    hooks::use_gateway,
};

use super::UpgradeStep;

#[component]
pub fn UpgradeConfirm(upgrade_step: Signal<UpgradeStep>, amount: u64) -> Element {
    let mut is_busy = use_signal(|| false);
    let gateway = use_gateway();

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
                            "{amount_to_ui_amount(amount, ore::TOKEN_DECIMALS_V1)}"
                        }
                    }
                }
            }
            div { class: "flex flex-col mt-auto sm:flex-row gap-2",
                button {
                    class: "w-full py-3 rounded font-semibold transition-colors text-white bg-green-500 hover:bg-green-600 active:enabled:bg-green-700",
                    disabled: *is_busy.read(),
                    onclick: move |_| {
                        is_busy.set(true);
                    },
                    if *is_busy.read() {
                        Spinner { class: "mx-auto" }
                    } else {
                        "Upgrade"
                    }
                }
            }
        }
    }
}
