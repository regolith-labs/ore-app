use dioxus::prelude::*;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::Signer;
use std::time::Duration;

use crate::components::*;
use crate::hooks::{use_wallet, use_wallet_native, Wallet};

pub fn WalletAdapter() -> Element {
    let wallet = use_wallet();
    match wallet.cloned() {
        Wallet::Connected(address) => {
            rsx! {
                ConnectedWalletAdapter { address }
            }
        }
        Wallet::Disconnected => {
            rsx! {
                div {
                    ConnectButtonNative { wallet, width: 100 }
                }
            }
        }
    }
}

#[component]
pub fn ConnectButtonNative(wallet: Signal<Wallet>, width: u64) -> Element {
    rsx! {
        button {
            class: "rounded-full transition my-auto h-12 text-black bg-white hover:cursor-pointer hover:scale-105 duration-300 ease-in-out bg-controls-primary flex items-center justify-center",
            style: "width: {width}px;",
            onclick: move |_| {
                if let Ok(keypair) = use_wallet_native::get_or_set() {
                    wallet.set(Wallet::Connected(keypair.creator.pubkey()));
                }
            },
            "Connect"
        }
    }
}

#[component]
fn ConnectedWalletAdapter(address: Pubkey) -> Element {
    let len = address.to_string().len();
    let first_four = &address.to_string()[0..4];
    let last_four = &address.to_string()[len - 4..len];

    let mut drawer_open = use_signal(|| false);
    let mut is_animating = use_signal(|| false);

    let close_drawer = move |_| {
        if !is_animating.cloned() {
            is_animating.set(true);
            drawer_open.set(false);
            spawn(async move {
                // Keep the animation state active during the transition
                tokio::time::sleep(Duration::from_millis(300)).await;
                is_animating.set(false);
            });
        }
    };

    rsx! {
        div { class: "relative",
            button {
                onclick: move |_| {
                    if !is_animating.cloned() {
                        is_animating.set(true);
                        drawer_open.set(true);
                        spawn(async move {
                            tokio::time::sleep(Duration::from_millis(300)).await;
                            is_animating.set(false);
                        });
                    }
                },
                Row {
                    class: "elevated-control elevated-border rounded-full text-sm font-semibold h-12 px-5 hover:cursor-pointer gap-3",
                    gap: 3,
                    span { class: "mx-auto my-auto", "{first_four}...{last_four}" }
                    DrawerIcon { class: "w-3 text-elements-lowEmphasis" }
                }
            }
            if *drawer_open.read() || *is_animating.read() {
                div {
                    class: "fixed inset-0 transition-all duration-300 ease-in-out bg-black/50",
                    class: if *drawer_open.read() { "wallet-drawer-fade opacity-100" } else { "wallet-drawer-fade-out opacity-0" },
                    onclick: close_drawer,
                    div {
                        class: "fixed top-0 right-0 h-full transition-transform duration-300 ease-in-out transform",
                        class: if *drawer_open.read() { "wallet-drawer-slide translate-x-0" } else { "wallet-drawer-slide-out translate-x-full" },
                        WalletDrawer { on_close: close_drawer }
                    }
                }
            }
        }
    }
}
