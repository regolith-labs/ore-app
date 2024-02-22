use std::{fmt, io, str::FromStr};

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use solana_client_wasm::solana_sdk::native_token::LAMPORTS_PER_SOL;

use crate::{
    gateway::AsyncResult,
    hooks::{use_appearance, use_appearance_persistant, use_pubkey, use_sol_balance},
};

// Phase 1 (core)
// TODO Address
// TODO Solana balance
// TODO Export private key
// TODO Display (dark mode)
// TODO Performance
// TODO Total hashes
// TODO Total rewards
// TODO Time since registration

// TODO Preferred explorer

// Phase 2 (social)
// TODO Username
// TODO Profile photo
// TODO Bio
// TODO Contacts

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

#[component]
pub fn Settings(cx: Scope) -> Element {
    let pubkey = use_pubkey(cx);
    let sol_balance = use_sol_balance(cx);
    let selected_explorer = use_state(cx, || Explorer::Solana);
    // let selected_appearance = use_state(cx, || Appearance::Light);
    // let appearance = use_shared_state::<Appearance>(cx).unwrap();
    let appearance = use_appearance(cx);
    let appearance_persistent = use_appearance_persistant(cx);

    render! {
        div {
            class: "flex flex-col gap-16",
            div {
                class: "flex flex-col gap-4",
                h1 {
                    "Account"
                }
                div {
                    // TODO Copy + QR code buttons
                    class: "flex flex-row justify-between w-full",
                    p {
                        class: "font-bold",
                        "Address"
                    }
                    div {
                        class: "flex flex-col",
                        p {
                            "{pubkey}"
                        }
                    }
                }
                div {
                    // TODO Copy + QR code buttons
                    class: "flex flex-row justify-between w-full",
                    p {
                        class: "font-bold",
                        "Private key"
                    }
                    div {
                        class: "flex flex-col",
                        button {
                            class: "font-medium shrink ml-auto text-nowrap hover:text-black hover:underline",
                            "Export"
                        }
                    }
                }
                div {
                    class: "flex flex-row justify-between w-full",
                    p {
                        class: "font-bold",
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
                            selected_explorer.set(match e.value.as_str() {
                                "Solana Explorer" => Explorer::Solana,
                                "SolanaFM" => Explorer::SolanaFm,
                                "Solscan" => Explorer::Solscan,
                                "Xray" => Explorer::Xray,
                                _ => Explorer::Solana
                            });
                        },
                        option { value: "{Explorer::Solana}", "{Explorer::Solana}" }
                        option { value: "{Explorer::SolanaFm}", "{Explorer::SolanaFm}" }
                        option { value: "{Explorer::Solscan}", "{Explorer::Solscan}" }
                        option { value: "{Explorer::Xray}", "{Explorer::Xray}" }
                    }
                }
            }
            div {
                class: "flex flex-col gap-4",
                h1 {
                    "System"
                }
                div {
                    class: "flex flex-row justify-between",
                    p {
                        class: "font-bold",
                        "Runtime"
                    }
                    p {
                        "Chromium web browser"
                    }
                }
                div {
                    class: "flex flex-row justify-between",
                    p {
                        class: "font-bold",
                        "CPU"
                    }
                    p {
                        "12 core"
                    }
                }
                div {
                    class: "flex flex-row justify-between",
                    p {
                        class: "font-bold",
                        "GPU"
                    }
                    p {
                        "Coming soon"
                    }
                }
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
