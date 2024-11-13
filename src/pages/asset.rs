use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;

use crate::components::Breadcrumbs;

#[component]
pub fn Asset(asset: String) -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-4 w-full px-5 sm:px-8",
            Breadcrumbs {}
            span {
                class: "font-wide text-2xl font-semibold",
                "{asset}"
            }
            div {
                class: "flex flex-row gap-4",
                div {
                    class: "flex w-full h-64 bg-gray-800 rounded"
                }
                div {
                    class: "hidden lg:flex flex-col elevated shrink-0 h-min w-96 rounded",
                    SwapInput {
                        mint: Pubkey::new_unique(),
                        mode: SwapInputMode::Sell
                    }
                    SwapInput {
                        mint: Pubkey::new_unique(),
                        mode: SwapInputMode::Buy
                    }
                }
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
enum SwapInputMode {
    Buy,
    Sell,
}

#[component]
fn SwapInput(mint: Pubkey, mode: SwapInputMode) -> Element {
    let border = match mode {
        SwapInputMode::Buy => "",
        SwapInputMode::Sell => "border-b border-gray-800",
    };
    let title = match mode {
        SwapInputMode::Buy => "Buying",
        SwapInputMode::Sell => "Selling",
    };

    rsx! {
        div {
            class: "flex flex-col gap-2 w-full p-4 {border}",
            div {
                class: "flex flex-row justify-between",
                span {
                    class: "text-gray-700 my-auto",
                    "{title}"
                }
                if mode == SwapInputMode::Sell {
                    button {
                        class: "text-xs my-auto py-1 px-3 rounded-full bg-gray-800",
                        onclick: move |_| {
                            // TODO
                        },
                        "Max"
                    }
                }
            }
            div {
                class: "flex flex-row gap-4",
                div {
                    class: "flex flex-row gap-3 my-auto",
                    img {
                        class: "w-8 h-8 rounded-full",
                        src: "https://upload.wikimedia.org/wikipedia/en/b/b9/Solana_logo.png",
                    }
                    span {
                        class: "font-semibold my-auto",
                        "SOL"
                    }
                }
                input {
                    class: "text-3xl placeholder:text-gray-700 font-semibold bg-transparent h-10 w-full outline-none text-right",
                    placeholder: "0"
                }
            }
        }
    }
}
