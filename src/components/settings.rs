use std::{fmt, io, str::FromStr};

use dioxus::prelude::*;
use dioxus_router::components::Link;
use is_url::is_url;
use serde::{Deserialize, Serialize};
use solana_client_wasm::solana_sdk::native_token::{lamports_to_sol, LAMPORTS_PER_SOL};

use crate::{
    components::{Appearance, BackupKeypairWarning, Copyable},
    gateway::{AsyncResult, RPC_URL},
    hooks::{
        use_appearance, use_explorer, use_pubkey, use_rpc_url, use_show_backup_warning,
        use_sol_balance, Explorer, RpcUrl,
    },
    route::Route,
};

pub fn Settings() -> Element {
    let mut explorer = use_explorer();
    let mut appearance = use_appearance();
    let mut show_backup_warning = use_show_backup_warning();
    let pubkey = use_pubkey();
    let sol_balance = use_sol_balance();

    let mut rpc_url = use_rpc_url();
    let mut rpc_url_input = use_signal(|| rpc_url.read().0.clone());
    let mut rpc_url_error = use_signal::<Option<String>>(|| None);
    let is_rpc_url_edited = rpc_url.read().0.ne(&*rpc_url_input.read());

    let container_class = "flex flex-row gap-8 justify-between w-full sm:px-1";
    let section_title_class = "text-lg md:text-2xl font-bold";
    let data_title_class = "font-medium text-sm opacity-50 my-auto";

    rsx! {
        div {
            class: "flex flex-col gap-16 w-full pb-24",
            div {
                class: "flex flex-col gap-4 w-full",
                h2 {
                    "Settings"
                }
                if cfg!(feature = "web") && show_backup_warning.read().0 {
                    div {
                        class: "mt-8",
                        BackupKeypairWarning {}
                    }
                }
                h2 {
                    class: "{section_title_class} mt-8",
                    "Account"
                }
                div {
                    class: "{container_class}",
                    p {
                        class: "{data_title_class}",
                        "Address"
                    }
                    Copyable {
                        value: pubkey.to_string(),
                        Link {
                            class: "font-mono sm:px-2 py-1 rounded hover-100 active-200 transition-colors truncate font-medium",
                            to: Route::User {
                                id: pubkey.to_string()
                            },
                            "{pubkey}"
                        }
                    }
                }
                div {
                    class: "{container_class}",
                    p {
                        class: "{data_title_class}",
                        "Balance"
                    }
                    match *sol_balance.read() {
                        AsyncResult::Ok(balance) => {
                            rsx! {
                                p {
                                    "{lamports_to_sol(balance.0)} SOL"
                                }
                            }
                        }
                        _ => {
                            rsx! {
                                div {
                                    class: "flex w-32 loading rounded",
                                }
                            }
                        }
                    }
                }
                div {
                    class: "{container_class}",
                    p {
                        class: "{data_title_class}",
                        "Keypair"
                    }
                    div {
                        class: "flex flex-row gap-2 -mr-2",
                        Link {
                            to: Route::ImportKey {},
                            class: "font-semibold hover-100 active-200 transition-colors px-4 py-1 rounded",
                            "Import"
                        }
                        Link {
                            to: Route::ExportKey {},
                            class: "font-semibold hover-100 active-200 transition-colors px-4 py-1 rounded",
                            "Export"
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
            div {
                class: "flex flex-col gap-4",
                h2 {
                    class: "{section_title_class}",
                    "Network"
                }
                div {
                    class: "{container_class}",
                    p {
                        class: "{data_title_class} mb-auto py-1",
                        "RPC"
                    }
                    div {
                        class: "flex flex-col gap-2",
                        input {
                            autofocus: false,
                            class: "w-full max-w-96 text-right placeholder-gray-300 dark:placeholder-gray-800 bg-transparent",
                            value: "{rpc_url_input}",
                            placeholder: "{RPC_URL}",
                            oninput: move |evt| {
                                let s = evt.value();
                                rpc_url_input.set(s.clone());
                                if !is_url(&s) {
                                    rpc_url_error.set(Some("Invalid url".to_string()));
                                } else {
                                    rpc_url_error.set(None);
                                }
                            },
                        }
                        if let Some(err_str) = rpc_url_error.read().clone() {
                            p {
                                class: "text-sm text-red-500 text-right",
                                "{err_str}"
                            }
                        }
                        div {
                            class: "flex flex-row gap-2",
                            if rpc_url.read().0.ne(RPC_URL) {
                                button {
                                    class: "hover-100 active-200 rounded shrink ml-auto transition-colors px-2 py-1",
                                    onclick: move |_| {
                                        rpc_url.set(RpcUrl(RPC_URL.to_string()));
                                        rpc_url_input.set(RPC_URL.to_string());
                                        rpc_url_error.set(None);
                                    },
                                    "Default"
                                }
                            }
                            if is_rpc_url_edited && rpc_url_error.read().is_none() {
                                button {
                                    class: "bg-green-500 hover:bg-green-600 active:bg-green-700 text-white rounded shrink ml-auto transition-colors px-2 py-1",
                                    onclick: move |_| {
                                        rpc_url.set(RpcUrl(rpc_url_input.read().clone()));
                                    },
                                    "Save"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
