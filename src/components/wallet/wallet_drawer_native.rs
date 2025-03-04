use dioxus::prelude::*;

use crate::{
    components::Row,
    hooks::{on_transaction_done, use_wallet, Wallet},
};

#[component]
pub fn WalletDrawer(on_close: EventHandler<MouseEvent>) -> Element {
    let wallet = use_wallet();
    let mut pubkey = use_signal(|| "missing pubkey".to_string());

    use_effect(move || {
        if let Wallet::Connected(pk) = *wallet.read() {
            pubkey.set(pk.to_string());
        }
    });

    rsx! {
        div {
            class: "flex flex-col gap-8 h-full sm:w-96 w-screen elevated elevated-border text-white py-8 z-50",
            style: "padding-left: 20px; padding-right: 20px;",
            onclick: move |e| e.stop_propagation(),
            // "TODO: Wallet address + copy button"
            div {
                "{pubkey}"
            }
        }
    }
}
