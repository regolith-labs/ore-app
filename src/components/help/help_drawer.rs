use crate::components::{Col, CopyIcon, GlobeIcon, PaperAirplaneIcon, Row};
use crate::hooks::{use_wallet, Wallet};
use crate::route::Route;
use dioxus::document::eval;
use dioxus::prelude::*;
use std::str::FromStr;
use {wasm_bindgen_futures, web_sys};

#[component]
pub fn HelpDrawer(on_close: EventHandler<MouseEvent>, drawer_remount: Signal<bool>) -> Element {
    let navigator = use_navigator();

    rsx! {
        div {
            class: "flex flex-col h-full w-screen sm:w-120 elevated elevated-border text-white z-50 relative",
            onclick: move |e| e.stop_propagation(),

            // Header section with fixed content
            div {
                class: "px-4 pt-4 pb-2",

                // Close wallet button
                button {
                    class: "rounded-full text-center py-1 w-8 h-8 flex items-center justify-center bg-surface-floating hover:bg-surface-floating-hover self-center cursor-pointer mb-4",
                    onclick: move |e| {
                        e.stop_propagation();
                        on_close.call(e);
                    },
                    span {
                        class: "text-xl font-semibold",
                        "×"
                    }
                }

                // Action links row
                Row {
                    class: "justify-center items-center mb-4 gap-8",
                    Col {
                        class: "items-center",
                        gap: 2,
                        // a {
                        //     class: "flex items-center justify-center w-12 h-12 rounded-full controls-secondary",
                        //     href: "https://solscan.io/account/{pubkey.read()}",
                        //     GlobeIcon { class: "h-5" }
                        // }
                        span {
                            class: "text-xs whitespace-nowrap text-elements-lowEmphasis",
                            "Explorer"
                        }
                    }
                    Col {
                        class: "items-center",
                        gap: 2,
                        Link {
                            class: "flex items-center justify-center w-12 h-12 rounded-full controls-secondary",
                            onclick: move |e: MouseEvent| {
                                e.stop_propagation();
                                on_close.call(e);
                            },
                            to: Route::TransferWithToken { token_ticker: "ORE".to_string() },
                            PaperAirplaneIcon { class: "h-5" }
                        }
                        span {
                            class: "text-xs whitespace-nowrap text-elements-lowEmphasis",
                            "Transfer"
                        }
                    }
                }
            }

            // Token List with overflow handling - the content area
            div {
                class: "flex-1 overflow-y-auto",
                style: "padding-bottom: 1rem;", // Add padding at the bottom for better visibility

                // Add a token list with click handling to close drawer like in native version
                {
                    let tokens = crate::hooks::use_tokens_with_values();
                    // let navigate = use_navigator();

                    rsx! {
                        Col {
                            class: "w-full",
                            {tokens.iter().map(|token| {
                                let token_clone = token.clone();
                                // let on_close_clone = on_close.clone();
                                // let navigate_clone = navigate.clone();

                                rsx! {
                                    div {
                                        key: "{token.token.mint}",
                                        class: "w-full justify-between items-center py-4 px-4 sm:rounded-md transition duration-300 ease-in-out hover:bg-controls-tertiary active:bg-controls-tertiaryHover hover:cursor-pointer",
                                        onclick: move |e| {
                                            // First close drawer (like in native version)
                                            e.stop_propagation();
                                            on_close.call(e.clone());

                                            // Then navigate
                                            navigator.push(Route::TransferWithToken {
                                                token_ticker: token_clone.token.ticker.clone()
                                            });
                                            // navigator.push(Route::Transfer {});
                                        },
                                        Row {
                                            class: "w-full justify-between items-center",
                                            Row {
                                                class: "items-center",
                                                gap: 4,
                                                img { class: "w-8 h-8 rounded-full shrink-0", src: "{token.token.image}" }
                                                Col {
                                                    span { class: "font-medium text-elements-highEmphasis", "{token.token.name}" }
                                                    span { class: "font-medium text-xs text-elements-lowEmphasis",
                                                        "{token.balance:.4} {token.token.ticker}"
                                                    }
                                                }
                                            }
                                            Col {
                                                class: "items-end",
                                                "${token.total_value:.2}"
                                            }
                                        }
                                    }
                                }
                            })}
                        }
                    }
                }
            }
        }
    }
}
