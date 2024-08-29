use dioxus::prelude::*;

use crate::{
    components::{BackButton, WarningIcon},
    hooks::{
        use_wallet_adapter::{self, WalletAdapter},
        UiTokenAmountBalance,
    },
};

use super::UpgradeStep;

#[component]
pub fn UpgradeEdit(
    upgrade_step: Signal<UpgradeStep>,
    amount_input: Signal<String>,
    parsed_amount: u64,
) -> Element {
    let nav = navigator();
    let wallet_adapter = use_wallet_adapter::use_wallet_adapter();
    let balances_resource = use_wallet_adapter::use_ore_balances();

    let (max_amount, max_amount_str) = match balances_resource.cloned() {
        Some(balances) => balances
            .map(|b| (b.v1.balance(), b.v1.ui_amount_string))
            .unwrap_or((0, "0".to_owned())),
        None => (0, "0".to_owned()),
    };

    let error_text = if parsed_amount.gt(&max_amount) {
        Some("Amount too large".to_string())
    } else {
        None
    };

    // build disabled
    let is_disabled = amount_input.read().len().eq(&0)
        || amount_input.read().parse::<f64>().is_err()
        || error_text.is_some()
        || wallet_adapter.cloned().eq(&WalletAdapter::Disconnected);

    // balance styles
    // let container_class = "flex flex-row gap-8 w-full sm:px-1";
    // let section_title_class = "text-lg md:text-2xl font-bold";
    // let data_title_class = "font-medium text-sm my-auto opacity-50";
    rsx! {
        div {
            class: "flex flex-col h-full grow justify-between",
            div {
                class: "flex flex-col gap-4 -mt-3.5 mb-4",
                BackButton {
                    onclick: move |_| {
                        nav.go_back()
                    }
                }
                div {
                    class: "flex flex-col gap-2",
                    h2 {
                        "Upgrade"
                    }
                    p {
                        class: "text-lg",
                        "Upgrade your OREv1 tokens to ORE."
                    }
                    p {
                        class: "text-sm text-gray-300",
                        "This will burn your OREv1 tokens and mint an equal number of ORE tokens directly to your wallet."
                    }
                }
            }
            div {
                class: "flex flex-col gap-8",
                if let Some(error_text) = error_text {
                    p {
                        class: "flex flex-row flex-nowrap gap-2 text-white w-min mx-auto text-nowrap bg-red-500 text-center font-semibold text-sm rounded py-1 px-2",
                        WarningIcon {
                            class: "w-3.5 h-3.5 my-auto"
                        }
                        "{error_text}"
                    }
                }
                input {
                    autofocus: true,
                    class: "mx-auto w-full text-center focus:ring-0 outline-none placeholder-gray-200 dark:placeholder-gray-800 bg-transparent text-3xl sm:text-4xl md:text-5xl font-medium",
                    value: "{amount_input}",
                    placeholder: "0",
                    oninput: move |evt| {
                        let s = evt.value();
                        if s.len().eq(&0) || s.parse::<f64>().is_ok() {
                            amount_input.set(s);
                        } else {
                            amount_input.set(s[..s.len()-1].to_string());
                        }
                    },
                }
                button {
                    class: "flex transition-colors shrink text-nowrap py-2 px-4 mx-auto text-center text-nowrap rounded-full font-medium hover-100 active-200",
                    onclick: move |_| {
                        amount_input.set(max_amount_str.clone())
                    },
                    "Max"
                }
            }
            button {
                class: "w-full py-3 rounded font-semibold transition-colors transition-opacity text-white bg-green-500 hover:bg-green-600 active:bg-green-700 disabled:opacity-20",
                disabled: is_disabled,
                onclick: move |_| { upgrade_step.set(UpgradeStep::Confirm) },
                "Review"
            }
        }
    }
}
