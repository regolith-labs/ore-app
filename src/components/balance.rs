use dioxus::prelude::*;
use dioxus_router::components::Link;
#[cfg(feature = "web")]
use solana_extra_wasm::account_decoder::parse_token::UiTokenAmount;
#[cfg(feature = "desktop")]
use solana_account_decoder::parse_token::UiTokenAmount;

use crate::{components::OreIcon, gateway::AsyncResult, route::Route};

#[component]
pub fn Balance(cx: Scope, balance: AsyncResult<UiTokenAmount>) -> Element {
    render! {
        div {
            class: "flex flex-row w-full min-h-16 rounded justify-between",
            match balance {
                AsyncResult::Ok(b) => {
                    render! {
                        div {
                            class: "flex flex-col grow gap-4",
                            h2 {
                                class: "text-lg md:text-2xl font-bold",
                                "Balance"
                            }
                            div {
                                class: "flex flex-row grow justify-between",
                                div {
                                    class: "flex flex-row my-auto gap-2.5 md:gap-4",
                                    OreIcon {
                                        class: "my-auto w-7 h-7 sm:w-8 sm:h-8 md:w-10 md:h-10"
                                    }
                                    h1 {
                                        class: "text-3xl sm:text-4xl md:text-5xl",
                                        "{b.real_number_string_trimmed()}"
                                    }
                                }
                                SendButton {}
                            }
                        }
                    }
                }
                _ => {
                    render! {
                        div {
                            class: "flex flex-row grow loading rounded",
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn SendButton(cx: Scope) -> Element {
    render! {
        Link {
            to: Route::Send {},
            class: "flex h-10 w-10 my-auto rounded-full justify-center text-2xl font-black transition-colors bg-black text-white dark:bg-white dark:text-black",
            span {
                class: "my-auto bg-transparent",
                "↑"
            }
        }
    }
}
