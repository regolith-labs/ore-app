use std::str::FromStr;

use dioxus::prelude::*;
use dioxus_sdk::clipboard::use_clipboard;

use crate::{
    components::Row,
    hooks::{on_transaction_done, use_wallet, Wallet},
};

#[component]
pub fn WalletDrawer(on_close: EventHandler<MouseEvent>) -> Element {
    // wallet
    let wallet = use_wallet();
    let mut pubkey = use_signal(|| "missing pubkey".to_string());
    let mut pubkey_splice = use_signal(|| Splice::Pubkey("0000...0000".to_string()));
    let mut is_copied = use_signal(|| false);
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
    // clipboard
    let mut clipboard = use_clipboard();
    // listen for clipboard
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
    rsx! {
        div { class: "flex flex-col gap-8 h-full sm:w-96 w-screen elevated elevated-border text-white py-8 z-50",
            button {
                class: "rounded-full text-center py-4 px-6 mx-4 controls-secondary hover:cursor-pointer flex justify-center items-center",
                onclick: move |e| {
                    e.stop_propagation();
                    clipboard.set(pubkey.to_string());
                    pubkey_splice.set(Splice::Copied);
                },
                div { "{pubkey_splice.read().to_string()}" }
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
