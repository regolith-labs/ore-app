use dioxus::prelude::*;

use crate::{
    components::{MountWalletAdapter, WarningIcon},
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
    // fetch balances
    let balances_resource = use_wallet_adapter::use_ore_balances();
    let (max_amount, max_amount_str) = match balances_resource.cloned() {
        Some(balances) => balances
            .map(|b| (b.v1.balance(), b.v1.ui_amount_string))
            .unwrap_or((0, "0".to_owned())),
        None => (0, "0".to_owned()),
    };
    let balance_v2_str = match balances_resource.cloned() {
        Some(balances) => balances
            .map(|b| b.v2.ui_amount_string)
            .unwrap_or("0".to_owned()),
        None => "0".to_owned(),
    };
    // build err
    log::info!("max amount: {}", max_amount_str);
    let amount_error_text = if parsed_amount.gt(&max_amount) {
        Some("Amount too large".to_string())
    } else {
        None
    };
    // build disabled
    let is_disabled = amount_input.read().len().eq(&0)
        || amount_input.read().parse::<f64>().is_err()
        || amount_error_text.is_some()
        || wallet_adapter.cloned().eq(&WalletAdapter::Disconnected);
    rsx! {
        div { class: "flex flex-col h-full grow gap-12",
            div { class: "flex flex-col gap-3",
                h2 { "Upgrade" }
                p { class: "text-lg", "Upgrade ORE v1 to v2" }
                MountWalletAdapter {}
                div { "ORE v1 balance: {max_amount_str}" }
                div { "ORE v2 balance: {balance_v2_str}" }
            }
            div { class: "flex flex-col gap-12",
                div { class: "flex flex-col gap-2", "Amount" }
                div { class: "flex flex-row gap-3",
                    input {
                        class: "mx-auto w-full focus:ring-0 outline-none placeholder-gray-200 dark:placeholder-gray-700 bg-transparent text-xl font-medium",
                        value: "{amount_input}",
                        placeholder: "0",
                        oninput: move |e| {
                            let s = e.value();
                            if s.len().eq(&0) || s.parse::<f64>().is_ok() {
                                amount_input.set(s);
                            } else {
                                amount_input.set(s[..s.len() - 1].to_string());
                            }
                        }
                    }
                    button {
                        class: "flex transition-colors w-min text-nowrap py-2 px-4 mx-auto text-center rounded-full text-sm font-medium hover-100 active-200",
                        onclick: move |_| {
                            amount_input.set(max_amount_str.clone());
                        },
                        "Max"
                    }
                }
                if let Some(err) = amount_error_text {
                    p { class: "flex flex-row flex-nowrap gap-1.5 w-min text-nowrap text-red-500 font-semibold text-sm",
                        WarningIcon { class: "w-4 h-4 my-auto" }
                        "{err}"
                    }
                }
            }
            div { class: "flex flex-col sm:flex-row gap-2 mt-auto",
                button {
                    class: "w-full py-3 rounded font-semibold transition-colors hover-100 active-200",
                    onclick: move |_| {
                        nav.go_back();
                    },
                    "Cancel"
                }
                button {
                    class: "w-full py-3 rounded font-semibold transition-colors transition-opacity text-white bg-green-500 hover:bg-green-600 active:bg-green-700 disabled:opacity-20",
                    disabled: is_disabled,
                    onclick: move |_| {
                        spawn(async move {
                            let res = wallet_adapter.read().build_upgrade_tx(parsed_amount).await;
                            match res {
                                Ok(tx) => {upgrade_step.set(UpgradeStep::Confirm(tx))},
                                Err(_) => {upgrade_step.set(UpgradeStep::Edit)}
                            };
                        });
                    },
                    "Review"
                }
            }
        }
    }
}
