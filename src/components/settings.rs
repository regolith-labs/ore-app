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
    hooks::{use_appearance, use_explorer, use_pubkey, use_sol_balance},
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
    let appearance = use_appearance(cx);

    // TODO use_concurrency()
    let cores = "12".to_string();
    // let cores = if let Some(window) = window() {
    //     window.navigator().hardware_concurrency().to_string()
    // } else {
    //     "Unknown".to_string()
    // };

    let container_class = "flex flex-row gap-8 justify-between w-full sm:px-1";
    let section_title_class = "text-lg md:text-2xl font-bold";
    let data_title_class = "font-medium text-sm opacity-50 my-auto";

    render! {
        div {
            class: "flex flex-col gap-16 w-full",
            div {
                class: "flex flex-col gap-4 w-full",
                h2 {
                    "Settings"
                }
                h2 {
                    class: "{section_title_class}",
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
                        "Private key"
                    }
                    button {
                        class: "flex flex-row shrink font-medium px-2 py-1 text-nowrap hover-100 active-200 rounded",
                        "Export"
                    }
                }
                div {
                    class: "{container_class}",
                    p {
                        class: "{data_title_class}",
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
                            if let Ok(a) = Appearance::from_str(e.value.as_str()) {
                                *appearance.write() = a;
                            }
                        },
                        option { initial_selected: appearance.read().eq(&Appearance::Light), value: "{Appearance::Light}", "{Appearance::Light}" }
                        option { initial_selected: appearance.read().eq(&Appearance::Dark), value: "{Appearance::Dark}", "{Appearance::Dark}" }
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
                            if let Ok(e) = Explorer::from_str(e.value.as_str()) {
                                *explorer.write() = e;
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
                    "System"
                }
                div {
                    class: "{container_class}",
                    p {
                        class: "{data_title_class}",
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
