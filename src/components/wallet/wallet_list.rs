use dioxus::prelude::*;

use crate::{components::*, hooks::use_wallet_state, route::Route};

pub fn WalletList() -> Element {
    let wallet_state = use_wallet_state().cloned();
    rsx! {
        Col {
            class: "w-full",
            {wallet_state.wallet_pubkeys.iter().map(|wallet| {
                let wallet_pubkey = wallet.pubkey.to_string();
                let wallet_index = wallet.index;
                let wallet_name = wallet.name.clone();
                let wallet_pubkey_splice = format_pubkey(wallet_pubkey);
                rsx! {
                    // Link {
                        // to: Route::TransferWithToken { token_ticker: token.token.ticker.clone() },
                        Row {
                            key: "{wallet_index}",
                            class: "w-full justify-between items-center mx-4 mb-4 py-4 px-4 sm:rounded-md transition duration-300 ease-in-out bg-surface-floating hover:cursor-pointer",
                            Row {
                                class: "items-center",
                                gap: 4,
                                // img { class: "w-8 h-8 rounded-full shrink-0", src: "{token.token.image}" }
                                Col {
                                    span { class: "font-medium text-elements-highEmphasis", "{wallet_name}" }
                                    span { class: "font-medium text-sm text-elements-lowEmphasis",
                                        "{wallet_pubkey_splice}"
                                    }
                                }
                            }
                            Col {
                                class: "items-end",
                                // "${token.total_value:.2}"
                            }
                        }
                    // }
                }
            })}
        }
    }
}

fn format_pubkey(pubkey: String) -> String {
    let len = pubkey.len();
    let first_four = &pubkey[0..4];
    let last_four = &pubkey[len - 4..len];
    format!("{}...{}", first_four, last_four)
}
