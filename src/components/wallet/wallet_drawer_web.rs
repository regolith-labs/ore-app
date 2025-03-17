use dioxus::document::eval;
use dioxus::prelude::*;

use crate::hooks::{use_wallet, use_wss, GetPubkey, ToWssMsg};

// use super::WalletTab;

#[component]
pub fn WalletDrawer(on_close: EventHandler<MouseEvent>, wallet_remount: Signal<bool>) -> Element {
    // let tab = use_signal(|| WalletTab::Tokens);
    let wallet = use_wallet();
    let (from_wss, to_wss) = use_wss();

    use_effect(move || {
        let msg = from_wss.read();
        log::info!("from wss: {:?}", msg);
    });

    use_effect(move || {
        if let Ok(pubkey) = wallet.pubkey() {
            log::info!("pubkey: {:?}", pubkey);
            to_wss.send(ToWssMsg::Subscribe(pubkey));
        }
    });

    rsx! {
        div {
            class: "flex flex-col gap-8 h-full sm:w-96 w-screen elevated elevated-border text-white py-8 z-50",
            onclick: move |e| e.stop_propagation(),

            // "TODO: Wallet address + copy button"

            DisconnectButton { wallet_remount },
            // Col {
            //     WalletTabs { tab },
            //     match *tab.read() {
            //         WalletTab::Tokens => rsx! {
            //             TokenTable { on_close }
            //         },
            //         WalletTab::Liquidity => rsx! {
            //             LiquidityTable { on_close }
            //         }
            //     }
            // }
        }
    }
}

#[component]
fn DisconnectButton(wallet_remount: Signal<bool>) -> Element {
    rsx! {
        button {
            class: "rounded-full text-center py-4 px-6 mx-4 controls-secondary hover:cursor-pointer",
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
