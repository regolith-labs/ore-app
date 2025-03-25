use crate::components::{Col, CopyIcon, GlobeIcon, PaperAirplaneIcon, Row};
use crate::hooks::{use_wallet, Wallet};
use crate::route::Route;
use dioxus::document::eval;
use dioxus::prelude::*;
use std::str::FromStr;
use {wasm_bindgen_futures, web_sys};

struct WebClipboard;

impl WebClipboard {
    fn new() -> Self {
        WebClipboard
    }

    fn set(&self, text: String) -> Result<(), String> {
        let window = web_sys::window().ok_or("No window available")?;
        let navigator = window.navigator();

        // Navigator.clipboard() returns the clipboard object directly
        let clipboard = navigator.clipboard();

        let promise = clipboard.write_text(&text);
        wasm_bindgen_futures::spawn_local(async {
            let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
        });

        Ok(())
    }
}

// use super::WalletTab;

enum Splice {
    Pubkey(String),
    Copied,
}

impl ToString for Splice {
    fn to_string(&self) -> String {
        match self {
            Self::Pubkey(pubkey) => pubkey.to_string(),
            Self::Copied => "Copied!".to_string(),
        }
    }
}

impl FromStr for Splice {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s.len();
        let first_four = &s[0..4];
        let last_four = &s[len - 4..len];
        let splice = format!("{}...{}", first_four, last_four);
        Ok(Splice::Pubkey(splice))
    }
}

#[component]
pub fn WalletDrawer(on_close: EventHandler<MouseEvent>, wallet_remount: Signal<bool>) -> Element {
    // Platform-specific clipboard
    #[cfg(feature = "web")]
    let clipboard = WebClipboard::new();

    // wallet
    let wallet = use_wallet();
    let mut pubkey = use_signal(|| "missing pubkey".to_string());
    let mut pubkey_splice = use_signal(|| Splice::Pubkey("0000...0000".to_string()));
    let mut pubkey_copied = use_signal(|| false);
    let navigator = use_navigator();

    // listen for wallet update
    use_effect(move || {
        if let Wallet::Connected(pk) = *wallet.read() {
            let pk = pk.to_string();
            // set pubkey
            pubkey.set(pk.clone());
            // set pubkey splice
            if let Ok(splice) = Splice::from_str(pk.as_str()) {
                pubkey_splice.set(splice);
            }
        }
    });

    // listen for pubkey clipboard reset
    use_effect(move || {
        if let Splice::Copied = *pubkey_splice.read() {
            spawn(async move {
                async_std::task::sleep(std::time::Duration::from_millis(1500)).await;
                let pk = pubkey.read();
                if let Ok(splice) = Splice::from_str(pk.as_str()) {
                    pubkey_splice.set(splice);
                }
            });
        }
    });

    // listen for pubkey copied reset
    use_effect(move || {
        if *pubkey_copied.read() {
            spawn(async move {
                async_std::task::sleep(std::time::Duration::from_millis(1500)).await;
                pubkey_copied.set(false);
            });
        }
    });

    rsx! {
        div {
            class: "flex flex-col h-full w-screen sm:w-96 elevated elevated-border text-white z-50 relative",
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

                // Clipboard button
                button {
                    class: "flex justify-center items-center rounded-full text-center py-4 px-6 w-full controls-secondary hover:cursor-pointer mb-4",
                    onclick: move |e| {
                        e.stop_propagation();
                        if let Err(err) = clipboard.set(pubkey.to_string()) {
                            log::error!("failed to set clipboard: {:?}", err);
                        }
                        pubkey_splice.set(Splice::Copied);
                    },
                    div { class: "flex items-center gap-2",
                        div { "{pubkey_splice.read().to_string()}" }
                        CopyIcon { class: "h-4 w-4", solid: false }
                    }
                }

                // Action links row
                Row {
                    class: "justify-center items-center mb-4 gap-8",
                    Col {
                        class: "items-center",
                        gap: 2,
                        a {
                            class: "flex items-center justify-center w-12 h-12 rounded-full controls-secondary",
                            href: "https://solscan.io/account/{pubkey.read()}",
                            GlobeIcon { class: "h-5" }
                        }
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

            // Disconnect button - with matching styling to native version
            Col {
                class: "mt-auto px-4 py-4 mb-4",
                DisconnectButton { wallet_remount }
            }
        }
    }
}

#[component]
fn DisconnectButton(wallet_remount: Signal<bool>) -> Element {
    rsx! {
        button {
            class: "w-full rounded-full text-center py-4 px-6 controls-secondary hover:cursor-pointer",
            onclick: move |_| {
                wallet_remount.set(true);
                let disconnect = eval(r#"window.OreWalletDisconnecter(); return"#);
                spawn(async move {
                    let _ = disconnect.await;
                });
            },
            span {
                class: "mx-auto",
                "Disconnect"
            }
        }
    }
}
