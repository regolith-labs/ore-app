use dioxus::prelude::*;
#[cfg(feature = "web")]
use solana_client_wasm::solana_sdk::{pubkey::Pubkey, signature::Keypair, signature::Signer};
#[cfg(feature = "desktop")]
use solana_sdk::{pubkey::Pubkey, signature::Keypair, signature::Signer};

use crate::hooks::use_persistent::{use_persistent, UsePersistent};

pub fn use_pubkey(cx: &ScopeState) -> Pubkey {
    let kp = use_keypair(cx);
    kp.pubkey()
}

pub fn use_keypair(cx: &ScopeState) -> Keypair {
    let kp_str = use_persistent(cx, "keypair", || Keypair::new().to_base58_string());
    Keypair::from_base58_string(&kp_str.get())
}

pub fn use_is_onboarded(cx: &ScopeState) -> &UsePersistent<bool> {
    use_persistent(cx, "is_onboarded", || false)
}
