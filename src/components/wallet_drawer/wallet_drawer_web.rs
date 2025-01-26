use dioxus::document::eval;
use dioxus::prelude::*;

use super::{Col, LiquidityTable, TokenTable, WalletTab, WalletTabs};

#[component]
pub fn WalletDrawer(on_close: EventHandler<MouseEvent>, wallet_remount: Signal<bool>) -> Element {
    let tab = use_signal(|| WalletTab::Tokens);

    rsx! {
        div {
            class: "flex flex-col gap-8 h-full sm:w-96 w-screen elevated elevated-border text-white py-8 z-50",
            onclick: move |e| e.stop_propagation(),

            // "TODO: Wallet address + copy button"

            DisconnectButton { wallet_remount },
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

#[component]
fn DisconnectButton(wallet_remount: Signal<bool>) -> Element {
    rsx! {
        button {
            onclick: move |_| {
                wallet_remount.set(true);
                let disconnect = eval(r#"window.OreWalletDisconnecter(); return"#);
                spawn(async move {
                    let _ = disconnect.await;
                });
            },
            "Disconnect"
        }
    }
}
