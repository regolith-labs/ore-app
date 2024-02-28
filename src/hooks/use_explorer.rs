use std::ops::Deref;

use dioxus::prelude::*;

use crate::{components::Explorer, hooks::use_persistent::use_persistent};

const KEY: &str = "explorer";

pub fn use_explorer(cx: &ScopeState) -> &UseSharedState<Explorer> {
    let explorer = use_shared_state::<Explorer>(cx).unwrap();
    let explorer_persistent = use_persistent(cx, KEY, || Explorer::Solana);
    use_effect(cx, explorer, |_| {
        explorer_persistent.set(*explorer.read());
        async move {}
    });
    explorer
}

pub fn use_explorer_provider(cx: &ScopeState) {
    let explorer = use_persistent(cx, KEY, || Explorer::Solana).get();
    use_shared_state_provider(cx, || explorer);
}

pub fn use_explorer_account_url(cx: &ScopeState, address: &String) -> String {
    let explorer = use_explorer(cx);
    match explorer.read().deref() {
        Explorer::Solana => format!("https://explorer.solana.com/address/{}", address),
        Explorer::SolanaFm => format!("https://solana.fm/address/{}", address),
        Explorer::Solscan => format!("https://solscan.io/account/{}", address),
        Explorer::Xray => format!("https://xray.helius.xyz/account/{}", address),
    }
}

pub fn use_explorer_transaction_url(cx: &ScopeState, signature: &String) -> String {
    let explorer = use_explorer(cx);
    match explorer.read().deref() {
        Explorer::Solana => format!("https://explorer.solana.com/tx/{}", signature),
        Explorer::SolanaFm => format!("https://solana.fm/tx/{}", signature),
        Explorer::Solscan => format!("https://solscan.io/tx/{}", signature),
        Explorer::Xray => format!("https://xray.helius.xyz/tx/{}", signature),
    }
}
