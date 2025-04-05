use dioxus::document::eval;
use dioxus::prelude::*;
use solana_sdk::pubkey::Pubkey;

use crate::components::*;
use crate::hooks::{use_wallet, use_wallet_drawer_state, Wallet};
use crate::time::Duration;

pub fn WalletAdapter() -> Element {
    let wallet = use_wallet();

    let mut wallet_mount = use_future(move || async move {
        if Wallet::Disconnected == wallet.cloned() {
            async_std::task::sleep(Duration::from_millis(500)).await;
            let eval = eval(
                r#"
                    window.MountWalletAdapter();
                    return
                "#,
            );
            let _ = eval.await;
        }
    });

    let mut wallet_remount = use_signal(|| false);

    match wallet.cloned() {
        Wallet::Connected(address) => {
            rsx! {
                ConnectedWalletAdapter {
                    address: address,
                    wallet_remount
                }
            }
        }
        Wallet::Disconnected => {
            if *wallet_remount.peek() {
                wallet_mount.restart();
                wallet_remount.set(false);
            }
            rsx! {
                div {
                    class: "rounded-full transition my-auto h-12 text-black bg-white hover:cursor-pointer hover:scale-105 duration-300 ease-in-out bg-controls-primary",
                    nav {
                        id: "ore-wallet-adapter"
                    }
                }
            }
        }
    }
}

#[component]
fn ConnectedWalletAdapter(address: Pubkey, wallet_remount: Signal<bool>) -> Element {
    let len = address.to_string().len();
    let first_four = &address.to_string()[0..4];
    let last_four = &address.to_string()[len - 4..len];

    let mut drawer_state = use_wallet_drawer_state();
    let _is_open = *drawer_state.read();
    let mut is_animating = use_signal(|| false);

    // Close function that handles animation
    let close_drawer = move |_e: MouseEvent| {
        if !*is_animating.read() {
            is_animating.set(true);
            drawer_state.set(false);
            spawn(async move {
                // Keep the animation state active during the transition
                async_std::task::sleep(crate::time::Duration::from_millis(300)).await;
                is_animating.set(false);
            });
        }
    };

    rsx! {
        div {
            class: "relative",
            button {
                onclick: move |_| {
                    if !is_animating.cloned() {
                        is_animating.set(true);
                        drawer_state.set(true);
                        spawn(async move {
                            async_std::task::sleep(crate::time::Duration::from_millis(300)).await;
                            is_animating.set(false);
                        });
                    }
                },
                Row {
                    class: "elevated-control elevated-border rounded-full text-sm font-semibold h-12 px-5 hover:cursor-pointer gap-3",
                    gap: 3,
                    span {
                        class: "mx-auto my-auto",
                        "{first_four}...{last_four}"
                    }
                    DrawerIcon {
                        class: "w-3 text-elements-lowEmphasis"
                    }
                }
            }

            if *drawer_state.read() || *is_animating.read() {
                WalletDrawerOverlay {
                    is_open: *drawer_state.read(),
                    on_close: close_drawer,
                    wallet_remount: wallet_remount
                }
            }
        }
    }
}

#[component]
fn WalletDrawerOverlay(
    is_open: bool,
    on_close: EventHandler<MouseEvent>,
    wallet_remount: Signal<bool>,
) -> Element {
    // Render drawer always, but with proper animation classes
    rsx! {
        Fragment {
            // Background overlay
            div {
                class: "fixed inset-0 transition-all duration-300 ease-in-out bg-black/50 z-[1000]",
                class: if is_open { "wallet-drawer-fade opacity-100" } else { "wallet-drawer-fade-out opacity-0" },
                style: "height: 100vh; width: 100vw;",
                onclick: move |e| on_close.call(e)
            }

            // Drawer content with slide animation
            div {
                class: "fixed top-0 right-0 h-full w-screen sm:w-96 transition-transform duration-300 ease-in-out transform z-[1001]",
                class: if is_open { "wallet-drawer-slide translate-x-0" } else { "wallet-drawer-slide-out translate-x-full" },
                style: "height: 100vh;",
                WalletDrawer {
                    on_close: on_close.clone(),
                    wallet_remount: wallet_remount
                }
            }
        }
    }
}
