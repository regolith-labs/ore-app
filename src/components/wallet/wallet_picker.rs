use crate::{components::*, hooks::use_wallet_state};
use dioxus::prelude::*;

#[component]
pub fn WalletPicker(show: bool, on_close: EventHandler<()>) -> Element {
    let wallet_state = use_wallet_state().cloned();

    if !show {
        return rsx! {};
    }

    rsx! {
        // Overlay container
        div {
            class: "fixed inset-0 z-[1000] bg-black/50 hover:bg-black/60 transition-colors duration-200",
            onclick: move |_| on_close.call(()),
            // Dropdown content
            div {
                class: "fixed inset-0 elevated elevated-border flex flex-col px-4 pt-4 z-[1001]",
                onclick: move |e| e.stop_propagation(),
                // Header with title and close button
                Row {
                    class: "items-center relative mb-4",
                    button {
                        class: "rounded-full text-center w-8 h-8 flex items-center justify-center bg-surface-floating hover:bg-surface-floating-hover cursor-pointer absolute left-0",
                        onclick: move |e| {
                            e.stop_propagation();
                            on_close.call(());
                        },
                        span {
                            class: "text-xl font-semibold",
                            "×"
                        }
                    }
                    h2 {
                        class: "text-xl font-semibold w-full text-center",
                        "Wallets"
                    }
                }
                // // Network selector (Solana)
                // button {
                //     class: "w-full flex items-center gap-2 p-4 bg-surface-elevated hover:bg-surface-elevated/80 transition-colors",
                //     onclick: move |e| e.stop_propagation(),
                //     img { class: "w-6 h-6", src: "/path/to/solana-logo.png", alt: "Solana" }
                //     span { "Solana" }
                //     ChevronDownIcon { class: "h-4 w-4 ml-auto" }
                // }
                // Wallet list
                div {
                    class: "flex-1 overflow-y-auto",
                    onclick: move |e| e.stop_propagation(),
                    {wallet_state.wallet_pubkeys.iter().map(|wallet| {
                        let wallet_pubkey = wallet.pubkey.to_string();
                        let wallet_name = wallet.name.clone();
                        let is_selected = wallet.index == wallet_state.current_wallet_index;
                        let wallet_pubkey = wallet.pubkey.to_string();
                        let wallet_pubkey_splice = format_pubkey(wallet_pubkey);
                        rsx! {
                            button {
                                key: "{wallet.index}",
                                class: "w-full justify-between items-center py-4 px-4 sm:rounded-md transition duration-300 ease-in-out hover:bg-controls-tertiary active:bg-controls-tertiaryHover hover:cursor-pointer",
                                onclick: move |e| e.stop_propagation(),
                                Row {
                                    class: "items-center",
                                    gap: 4,
                                    div {
                                        class: "flex flex-col items-start",
                                        span { class: "font-medium text-elements-highEmphasis", "{wallet_name}" }
                                        span { class: "font-medium text-xs text-elements-lowEmphasis", "{wallet_pubkey_splice}" }
                                    }
                                    if is_selected {
                                        CheckIcon { class: "h-4 w-4 ml-auto text-elements-highEmphasis" }
                                    }
                                }
                            }
                        }
                    })}
                }
                Col {
                    class: "px-4 py-4 mb-4",
                    button {
                        class: "flex w-full rounded-full py-4 px-6 controls-secondary hover:cursor-pointer justify-center items-center gap-2",
                        onclick: move |e| {
                            e.stop_propagation();
                            // keypair_show_export.set(true);
                        },
                        PlusIcon { class: "h-5 w-5" }
                        "Add new Solana wallet"
                    }
                }
                // Add new wallet button
                // button {
                //     class: "w-full flex items-center gap-2 p-4 text-[#4C9EE8] hover:bg-surface-elevated transition-colors border-t border-elements-lowEmphasis hover:cursor-pointer",
                //     onclick: move |e| e.stop_propagation(),
                //     PlusIcon { class: "h-5 w-5" }
                //     span { "Add new Solana wallet" }
                // }
            }
        }
    }
}

fn format_pubkey(pubkey: String) -> String {
    let len = pubkey.len();
    let first_four = &pubkey[0..4];
    let last_four = &pubkey[len - 4..len];
    format!("{}...{}", first_four, last_four)
}
