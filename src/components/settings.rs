use std::{fmt, io, str::FromStr};

use dioxus::prelude::*;
use dioxus_router::components::Link;
use serde::{Deserialize, Serialize};
#[cfg(feature = "web")]
use solana_client_wasm::solana_sdk::native_token::LAMPORTS_PER_SOL;
#[cfg(feature = "desktop")]
use solana_sdk::native_token::LAMPORTS_PER_SOL;

use crate::{
    components::Copyable,
    gateway::AsyncResult,
    hooks::{
        use_appearance, use_appearance_persistant, use_explorer, use_explorer_persistant,
        use_pubkey, use_sol_balance,
    },
    route::Route,
};

// Phase 1 (core)
// TODO Export private key
// TODO CPU benchmark

// Phase 2 (social)
// TODO Username
// TODO Profile photo
// TODO Bio
// TODO Contacts

#[component]
pub fn Settings(cx: Scope) -> Element {
    let pubkey = use_pubkey(cx);
    let sol_balance = use_sol_balance(cx);
    let explorer = use_explorer(cx);
    let explorer_persistent = use_explorer_persistant(cx);
    let appearance = use_appearance(cx);
    let appearance_persistent = use_appearance_persistant(cx);

    // TODO use_concurrency()
    let cores = "12".to_string();
    // let cores = if let Some(window) = window() {
    //     window.navigator().hardware_concurrency().to_string()
    // } else {
    //     "Unknown".to_string()
    // };

    render! {
        div {
            class: "flex flex-col gap-16",
            div {
                class: "flex flex-col gap-4",
                h1 {
                    "Account"
                }
                div {
                    class: "flex flex-row justify-between w-full",
                    p {
                        class: "font-bold",
                        "Address"
                    }
                    Copyable {
                        value: pubkey.to_string(),
                        Link {
                            class: "font-mono px-2 py-1 rounded hover-100 active-200 transition-colors",
                            to: Route::User {
                                id: pubkey.to_string()
                            },
                            "{pubkey}"
                        }
                    }
                }
                div {
                    class: "flex flex-row justify-between w-full",
                    p {
                        class: "font-bold text-nowrap",
                        "Private key"
                    }
                    button {
                        class: "font-medium shrink px-2 py-1 text-nowrap hover-100 active-200 rounded",
                        "Export"
                    }
                }
                div {
                    class: "flex flex-row justify-between w-full",
                    p {
                        class: "font-bold text-nowrap",
                        "Solana balance"
                    }
                    match sol_balance {
                        AsyncResult::Ok(balance) => {
                            let balance_f = (balance as f64) / (LAMPORTS_PER_SOL as f64);
                            render! {
                                p {
                                    "{balance_f} SOL"
                                }
                            }
                        }
                        _ => {
                            render! {
                                div {
                                    class: "flex w-32 loading rounded",
                                }
                            }
                        }
                    }
                }
            }
            div {
                class: "flex flex-col gap-4",
                h1 {
                    "Display"
                }
                div {
                    class: "flex flex-row justify-between",
                    p {
                        class: "font-bold",
                        "Appearance"
                    }
                    select {
                        class: "text-right dark:bg-black dark:text-white",
                        onchange: move |e| {
                            if let Ok(a) = Appearance::from_str(e.value.as_str()) {
                                *appearance.write() = a;
                                appearance_persistent.set(a);
                            }
                        },
                        option { initial_selected: appearance.read().eq(&Appearance::Light), value: "{Appearance::Light}", "{Appearance::Light}" }
                        option { initial_selected: appearance.read().eq(&Appearance::Dark), value: "{Appearance::Dark}", "{Appearance::Dark}" }
                    }
                }
                div {
                    class: "flex flex-row justify-between",
                    p {
                        class: "font-bold",
                        "Explorer"
                    }
                    select {
                        class: "text-right dark:bg-black dark:text-white",
                        onchange: move |e| {
                            if let Ok(e) = Explorer::from_str(e.value.as_str()) {
                                *explorer.write() = e;
                                explorer_persistent.set(e);
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
                h1 {
                    "System"
                }
                div {
                    class: "flex flex-row justify-between gap-8",
                    p {
                        class: "font-bold",
                        "CPU"
                    }
                    p {
                        "{cores} core"
                    }
                }
                // div {
                //     class: "flex flex-row justify-between",
                //     p {
                //         class: "font-bold",
                //         "GPU"
                //     }
                //     p {
                //         "Coming soon"
                //     }
                // }
                // div {
                //     class: "flex flex-col mt-16 w-full text-sm justify-center",
                //     p {
                //         "You are mining from a web browser. "
                //         Link {
                //             class: "font-semibold underline-offset-2 underline",
                //             to: Route::Download {},
                //             "Install the app. →"
                //         }
                //     }
                // }
                // div {
                //     class: "flex flex-row justify-between",
                //     p {
                //         class: "font-bold",
                //         "Power"
                //     }
                //     p {
                //         "10%"
                //     }
                // }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq)]
pub enum Explorer {
    #[default]
    Solana,
    SolanaFm,
    Solscan,
    Xray,
}

impl fmt::Display for Explorer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Explorer::Solana => write!(f, "Solana Explorer"),
            Explorer::SolanaFm => write!(f, "SolanaFM"),
            Explorer::Solscan => write!(f, "Solscan"),
            Explorer::Xray => write!(f, "Xray"),
        }
    }
}

impl FromStr for Explorer {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Solana Explorer" => Ok(Explorer::Solana),
            "SolanaFM" => Ok(Explorer::SolanaFm),
            "Solscan" => Ok(Explorer::Solscan),
            "Xray" => Ok(Explorer::Xray),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Unknown explorer",
            )),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq)]
pub enum Appearance {
    #[default]
    Light,
    Dark,
}

impl FromStr for Appearance {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Light mode" => Ok(Appearance::Light),
            "Dark mode" => Ok(Appearance::Dark),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Unknown appearance",
            )),
        }
    }
}

impl fmt::Display for Appearance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Appearance::Light => write!(f, "Light mode"),
            Appearance::Dark => write!(f, "Dark mode"),
        }
    }
}
