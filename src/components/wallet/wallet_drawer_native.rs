use std::str::FromStr;

use dioxus::prelude::*;
use dioxus_sdk::clipboard::use_clipboard;

use crate::components::{CopyIcon, GlobeIcon, PaperAirplaneIcon, PlayIcon, PlusIcon};
use crate::hooks::{use_wallet, use_wallet_native, Wallet};
use crate::route::Route;

#[component]
pub fn WalletDrawer(on_close: EventHandler<MouseEvent>) -> Element {
    // clipboard
    let mut clipboard = use_clipboard();
    // wallet
    let wallet = use_wallet();
    // pubkey
    let mut pubkey = use_signal(|| "missing pubkey".to_string());
    let mut pubkey_splice = use_signal(|| Splice::Pubkey("0000...0000".to_string()));
    let mut pubkey_copied = use_signal(|| false);
    // keypair
    let mut keypair = use_signal(|| "failed to read private key".to_string());
    let mut keypair_show_export = use_signal(|| false);
    let mut keypair_copied = use_signal(|| false);
    // listen for wallet update
    use_memo(move || {
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
    // listen for pubkey clipboard
    use_memo(move || {
        if let Splice::Copied = *pubkey_splice.read() {
            spawn(async move {
                tokio::time::sleep(tokio::time::Duration::from_millis(1_500)).await;
                let pk = pubkey.read();
                if let Ok(splice) = Splice::from_str(pk.as_str()) {
                    pubkey_splice.set(splice);
                }
            });
        }
    });
    // listen for pubkey copied
    use_memo(move || {
        if *pubkey_copied.read() {
            spawn(async move {
                tokio::time::sleep(tokio::time::Duration::from_millis(1_500)).await;
                pubkey_copied.set(false);
            });
        }
    });
    // listen for keypair copied
    use_memo(move || {
        if *keypair_copied.read() {
            spawn(async move {
                tokio::time::sleep(tokio::time::Duration::from_millis(1_500)).await;
                keypair_copied.set(false);
            });
        }
    });
    // listen for keypair export
    use_memo(move || {
        if *keypair_show_export.read() {
            if let Ok(kp) = use_wallet_native::get() {
                let kp = kp.creator.to_base58_string();
                keypair.set(kp.clone());
            }
        } else {
            // clear keypair so that it's not sitting in memory
            keypair.set("failed to read private key".to_string());
        }
    });
    rsx! {
        div {
            class: "flex flex-col gap-2 h-full sm:w-96 w-screen elevated elevated-border text-white py-8 z-50",
            onclick: move |e| {
                keypair_show_export.set(false);
            },
            button {
                class: "flex justify-center items-center rounded-full text-center py-4 px-6 mx-4 controls-secondary hover:cursor-pointer",
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
            div {
                class: "flex flex-row justify-center items-center gap-10 mx-4 p-4",
                div {
                    class: "flex flex-col items-center gap-2",
                    a {
                        class: "flex items-center justify-center w-12 h-12 rounded-full controls-secondary",
                            
                        PlusIcon { class: "h-5" }
                    }
                    span {
                        class: "text-xs whitespace-nowrap text-elements-lowEmphasis",
                        "Top Up"
                    }
                }
                div {
                    class: "flex flex-col items-center gap-2",
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
                div {
                    class: "flex flex-col items-center gap-2",
                    Link {
                        class: "flex items-center justify-center w-12 h-12 rounded-full controls-secondary",
                        to: Route::Transfer {},
                        onclick: move |e: MouseEvent| {
                            e.stop_propagation();
                            on_close.call(e);
                        },
                        PaperAirplaneIcon { class: "h-5" }
                    }
                    span {
                        class: "text-xs whitespace-nowrap text-elements-lowEmphasis",
                        "Transfer"
                    }
                }
            }
            if *keypair_show_export.read() {
                button {
                    class: "flex flex-col gap-2 mt-auto mx-4",
                    onclick: move |e| {
                        e.stop_propagation();
                        if let Err(err) = clipboard.set(keypair.to_string()) {
                            log::error!("failed to set clipboard: {:?}", err);
                        }
                        keypair_copied.set(true);
                    },
                    div { class: "p-2 controls-secondary text-center w-full hover:cursor-pointer flex justify-center items-center",
                        div {
                            if *keypair_copied.read() {
                                "Copied!"
                            } else {
                                div {
                                    class: "p-4 controls-secondary break-all text-sm w-full",
                                    style: "word-break: break-word; white-space: pre-wrap;",
                                    "{keypair.read().to_string()}"
                                }
                            }
                        }
                    }
                }
            } else {
                div { class: "flex flex-col gap-2 mt-auto",
                    button {
                        class: "flex justify-center items-center rounded-full text-center py-4 px-6 mx-4 controls-secondary hover:cursor-pointer",
                        onclick: move |e| {
                            e.stop_propagation();
                            keypair_show_export.set(true);
                        },
                        "Export Keypair"
                    }
                }
            }
        }
    }
}

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
