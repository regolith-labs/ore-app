use std::str::FromStr;

use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::native_token::lamports_to_sol;

use crate::{
    components::Appearance,
    hooks::{
        use_appearance, use_explorer, use_sol_balance,
        use_wallet_adapter::{use_wallet_adapter, WalletAdapter},
        Explorer,
    },
};

pub fn Settings() -> Element {
    let mut explorer = use_explorer();
    let mut appearance = use_appearance();
    let sol_balance = use_sol_balance();
    let wallet_adapter = use_wallet_adapter();

    let container_class = "flex flex-row gap-8 justify-between w-full sm:px-1";
    let section_title_class = "text-lg md:text-2xl font-bold";
    let data_title_class = "font-medium text-sm text-gray-300 my-auto";

    rsx! {
        div {
            class: "flex flex-col gap-16 w-full pb-24",
            if let WalletAdapter::Connected(_) = *wallet_adapter.read() {
                div {
                    class: "flex flex-col gap-4 w-full",
                    h2 {
                        "Settings"
                    }
                    h2 {
                        class: "{section_title_class} mt-8",
                        "Account"
                    }
                    // div {
                    //     class: "{container_class}",
                    //     p {
                    //         class: "{data_title_class}",
                    //         "Address"
                    //     }
                    //     Copyable {
                    //         value: pubkey.to_string(),
                    //         Link {
                    //             class: "font-mono sm:px-2 py-1 rounded hover-100 active-200 transition-colors truncate font-medium",
                    //             to: Route::User {
                    //                 id: pubkey.to_string()
                    //             },
                    //             "{pubkey}"
                    //         }
                    //     }
                    // }
                    div {
                        class: "{container_class}",
                        p {
                            class: "{data_title_class}",
                            "Balance"
                        }
                        if let Some(balance) = *sol_balance.read() {
                            if let Ok(balance) = balance {
                                p {
                                    "{lamports_to_sol(balance)} SOL"
                                }
                            } else {
                                p {
                                    "N/A"
                                }
                            }
                        } else {
                            div {
                                class: "flex w-32 loading rounded",
                            }
                        }
                    }
                }
            }
            div {
                class: "flex flex-col gap-4",
                h2 {
                    class: "{section_title_class}",
                    "Display"
                }
                div {
                    class: "{container_class}",
                    p {
                        class: "{data_title_class}",
                        "Appearance"
                    }
                    select {
                        class: "text-right bg-transparent dark:text-white hover:cursor-pointer py-1",
                        onchange: move |e| {
                            if let Ok(a) = Appearance::from_str(&e.value()) {
                                appearance.set(a);
                            }
                        },
                        option { initial_selected: appearance.read().eq(&Appearance::Dark), value: "{Appearance::Dark}", "{Appearance::Dark}" }
                        option { initial_selected: appearance.read().eq(&Appearance::Light), value: "{Appearance::Light}", "{Appearance::Light}" }
                    }
                }
                div {
                    class: "{container_class}",
                    p {
                        class: "{data_title_class}",
                        "Explorer"
                    }
                    select {
                        class: "text-right bg-transparent dark:text-white hover:cursor-pointer py-1",
                        onchange: move |e| {
                            if let Ok(e) = Explorer::from_str(&e.value()) {
                                explorer.set(e);
                            }
                        },
                        option { initial_selected: explorer.read().eq(&Explorer::Solana), value: "{Explorer::Solana}", "{Explorer::Solana}" }
                        option { initial_selected: explorer.read().eq(&Explorer::SolanaFm), value: "{Explorer::SolanaFm}", "{Explorer::SolanaFm}" }
                        option { initial_selected: explorer.read().eq(&Explorer::Solscan), value: "{Explorer::Solscan}", "{Explorer::Solscan}" }
                        option { initial_selected: explorer.read().eq(&Explorer::Xray), value: "{Explorer::Xray}", "{Explorer::Xray}" }
                    }
                }
            }
        }
    }
}
