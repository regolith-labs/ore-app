use dioxus::prelude::*;
#[cfg(feature = "web")]
use solana_client_wasm::solana_sdk::{pubkey::Pubkey, signature::Keypair, signature::Signer};
#[cfg(feature = "desktop")]
use solana_sdk::{pubkey::Pubkey, signature::Keypair, signature::Signer};

use crate::hooks::use_persistent::use_persistent;

use super::use_persistent::UsePersistent;

const KEY: &str = "keypair";

pub fn use_pubkey(cx: &ScopeState) -> Pubkey {
    let kp = use_keypair(cx);
    kp.pubkey()
}

pub fn use_keypair(cx: &ScopeState) -> Keypair {
    let kp_str = use_persistent(cx, KEY, || Keypair::new().to_base58_string());
    Keypair::from_base58_string(&kp_str.get())
}

pub fn use_keypair_persistent(cx: &ScopeState) -> &UsePersistent<String> {
    use_persistent(cx, KEY, || Keypair::new().to_base58_string())
}
