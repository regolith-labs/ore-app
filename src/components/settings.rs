use dioxus::prelude::*;
use dioxus_router::components::Link;
use solana_client_wasm::solana_sdk::native_token::LAMPORTS_PER_SOL;

use crate::{
    gateway::AsyncResult,
    hooks::{use_pubkey, use_sol_balance},
    route::Route,
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

// Phase 2 (social)
// TODO Username
// TODO Profile photo
// TODO Bio
// TODO Contacts

#[component]
pub fn Settings(cx: Scope) -> Element {
    let pubkey = use_pubkey(cx);
    let sol_balance = use_sol_balance(cx);
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
                        "Key"
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
                                    class: "flex w-32 animate-pulse bg-gray-100 rounded",
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
                    p {
                        "Light mode"
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
                        "Metal"
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
