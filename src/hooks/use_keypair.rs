use solana_client_wasm::solana_sdk::{pubkey::Pubkey, signature::Keypair, signature::Signer};

use crate::hooks::use_persistent::use_persistent;

use super::use_persistent::UsePersistent;

const KEY: &str = "keypair";

pub fn use_pubkey() -> Pubkey {
    let kp = use_keypair();
    kp.pubkey()
}

pub fn use_keypair() -> Keypair {
    let kp_str = use_persistent(KEY, || Keypair::new().to_base58_string());
    Keypair::from_base58_string(&kp_str.get())
}

pub fn use_keypair_persistent() -> UsePersistent<String> {
    use_persistent(KEY, || Keypair::new().to_base58_string())
}
