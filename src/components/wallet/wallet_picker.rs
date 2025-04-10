use crate::hooks::Wallet;
use crate::{
    components::*,
    hooks::{use_wallet, use_wallet_native, use_wallet_state},
};
use dioxus::prelude::*;
use solana_sdk::pubkey::Pubkey;

#[component]
pub fn WalletPicker(
    show: bool,
    on_close: EventHandler<()>,
    on_drawer_close: EventHandler<MouseEvent>,
) -> Element {
    let mut wallet_state = use_wallet_state();
    let mut current_wallet = use_wallet();
    let mut show_import_key = use_signal(|| false);
    let mut private_key = use_signal(|| String::new());
    let mut wallet_name = use_signal(|| String::new());
    let mut import_error = use_signal(|| None::<String>);

    if !show {
        return rsx! {};
    }

    // Function to select a wallet
    let mut select_wallet = move |pubkey_str: String, index: u8| {
        // Convert string to pubkey
        let pubkey = Pubkey::from_str_const(&pubkey_str);

        // Update global wallet
        current_wallet.set(Wallet::Connected(pubkey));

        // Update the current_wallet_index in wallet_state
        let mut state = wallet_state.write();
        state.current_wallet_index = index;

        // Save the config
        if let Err(err) = use_wallet_native::save_config(&state) {
            log::info!("Error saving config: {:?}", err);
        }

        drop(state);
    };

    // Don't close the drawer, let user continue interacting

    // Handle private key import
    let handle_import = move |e: MouseEvent| {
        e.stop_propagation();

        // Get private key and generate a default wallet name if not provided
        let key_value = private_key.read().to_string();
        let name_value = if wallet_name.read().is_empty() {
            format!("Wallet {}", wallet_state.read().num_wallets_used + 1)
        } else {
            wallet_name.read().to_string()
        };

        // Call the add_new_keypair function
        match use_wallet_native::add_new_keypair(Some(key_value), Some(name_value)) {
            Ok(_) => {
                // Clear input fields and errors
                private_key.set(String::new());
                wallet_name.set(String::new());
                import_error.set(None);

                // Return to wallet picker view
                show_import_key.set(false);
            }
            Err(err) => {
                // Display error message based on the error string representation
                let error_message = format!("{:?}", err);

                let user_friendly_message = if error_message.contains("InvalidPrivateKey") {
                    "Invalid private key format"
                } else if error_message.contains("UnableToDeriveKeypair") {
                    "Unable to derive keypair from private key"
                } else {
                    "Failed to import wallet"
                };

                import_error.set(Some(user_friendly_message.to_string()));
                log::error!("Error importing key: {:?}", err);
            }
        }
    };

    rsx! {
        // Overlay container
        div {
            class: "fixed inset-0 z-[1000] bg-black/50 hover:bg-black/60 transition-colors duration-200",
            onclick: move |_| on_close.call(()),

            if *show_import_key.read() {
                // Import Key Page
                div {
                    class: "fixed inset-0 elevated elevated-border flex flex-col px-4 pt-4 z-[1001]",
                    onclick: move |e| e.stop_propagation(),
                    // Back button and title
                    Row {
                        class: "items-center relative mb-8",
                        button {
                            class: "rounded-full text-center w-8 h-8 flex items-center justify-center bg-surface-floating hover:bg-surface-floating-hover cursor-pointer absolute left-0",
                            onclick: move |e| {
                                e.stop_propagation();
                                show_import_key.set(false);
                            },
                            ArrowLeftIcon { class: "h-5 w-5" }
                        }
                        h2 {
                            class: "text-xl font-semibold w-full text-center",
                            "Import private key"
                        }
                    }

                    // Key icon
                    div {
                        class: "flex justify-center mb-4",
                        div {
                            class: "bg-surface-floating rounded-full w-20 h-20 flex items-center justify-center",
                            KeyIcon { class: "h-8 w-8 text-gray-400" }
                        }
                    }

                    // Description
                    p {
                        class: "text-center text-elements-lowEmphasis mb-4",
                        "It will be encrypted and stored on your device."
                    }

                    // Wallet name input
                    div {
                        class: "mb-4",
                        input {
                            class: "w-full bg-surface-floating rounded-lg p-4 text-elements-highEmphasis focus:outline-none focus:ring-1 focus:ring-elements-midEmphasis",
                            placeholder: "Wallet name (optional)",
                            value: "{wallet_name}",
                            oninput: move |e| wallet_name.set(e.value().clone())
                        }
                    }

                    // Private key input
                    div {
                        class: "mb-auto",
                        textarea {
                            class: "w-full bg-surface-floating rounded-lg p-4 h-24 text-elements-highEmphasis resize-none focus:outline-none focus:ring-1 focus:ring-elements-midEmphasis",
                            style: "resize: none; overflow-y: auto;",
                            placeholder: "Private key",
                            value: "{private_key}",
                            oninput: move |e| private_key.set(e.value().clone())
                        }
                    }

                    // Error message (if any)
                    if let Some(error) = import_error.read().as_ref() {
                        span {
                            class: "text-red-500 text-md text-center mb-4",
                            "{error}"
                        }
                    }

                    // Import button
                    div {
                        class: "px-4 py-4 mb-4",
                        button {
                            class: "flex w-full rounded-full py-4 px-6 controls-secondary hover:cursor-pointer justify-center items-center",
                            onclick: handle_import,
                            "Import"
                        }
                    }
                }
            } else {
                // Wallet Picker
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
                        {wallet_state.read().wallet_pubkeys.iter().map(|wallet| {
                            let wallet_name = wallet.name.clone();
                            let is_selected = wallet.index == wallet_state.read().current_wallet_index;
                            let wallet_pubkey = wallet.pubkey.clone();
                            let wallet_pubkey_splice = format_pubkey(wallet_pubkey.clone());
                            let index_for_click = wallet.index;
                            rsx! {
                                button {
                                    key: "{wallet.index}",
                                    class: "w-full justify-between items-center mb-4 py-4 px-4 sm:rounded-md transition duration-300 ease-in-out bg-surface-floating hover:bg-controls-tertiary active:bg-controls-tertiaryHover hover:cursor-pointer",
                                    onclick: move |e| {
                                        e.stop_propagation();
                                        select_wallet(wallet_pubkey.clone(), index_for_click);
                                        on_drawer_close.call(e);
                                    },
                                    Row {
                                        class: "items-center",
                                        gap: 4,
                                        div {
                                            class: "flex flex-col items-start",
                                            span { class: "font-medium text-elements-highEmphasis", "{wallet_name}" }
                                            span { class: "font-medium text-xs text-elements-lowEmphasis", "{wallet_pubkey_splice}" }
                                        }
                                        if is_selected {
                                            CheckIcon { class: "h-5 w-5 ml-auto text-elements-highEmphasis" }
                                        }
                                    }
                                }
                            }
                        })}
                    }

                    if wallet_state.read().num_wallets_used < use_wallet_native::MAX_WALLETS_ALLOWED {
                        Col {
                            class: "px-4 py-4 mb-4",
                            button {
                                class: "flex w-full rounded-full py-4 px-6 controls-secondary hover:cursor-pointer justify-center items-center gap-2",
                                onclick: move |e| {
                                    e.stop_propagation();
                                    show_import_key.set(true);
                                    import_error.set(None); // Clear any previous errors
                                },
                                PlusIcon { class: "h-4 w-4" }
                                "Add new Solana wallet"
                            }
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
}

fn format_pubkey(pubkey: String) -> String {
    let len = pubkey.len();
    let first_four = &pubkey[0..4];
    let last_four = &pubkey[len - 4..len];
    format!("{}...{}", first_four, last_four)
}
