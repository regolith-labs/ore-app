use dioxus::prelude::*;

use crate::hooks::{use_wallet, Wallet};

use crate::components::*;

#[component]
pub fn WalletDrawer(on_close: EventHandler<MouseEvent>) -> Element {
    let wallet = use_wallet();
    let tab = use_signal(|| WalletTab::Tokens);
    let mut pubkey = use_signal(|| "no pubkey".to_string());
    use_effect(move || {
        if let Wallet::Connected(pk) = *wallet.read() {
            pubkey.set(pk.to_string());
        }
    });
    rsx! {
        div {
            class: "flex flex-col gap-8 h-full sm:w-96 w-screen elevated elevated-border text-white py-8 z-50",
            // "TODO: Wallet address + copy button"
            onclick: move |e| e.stop_propagation(),
            div { "{pubkey}" }
            Col {
                WalletTabs { tab },
                match *tab.read() {
                    WalletTab::Tokens => rsx! {
                        TokenTable { on_close }
                    },
                    WalletTab::Liquidity => rsx! {
                        LiquidityTable { on_close }
                    }
                }
            }
        }
    }
}
