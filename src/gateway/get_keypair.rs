use gloo_storage::{LocalStorage, Storage};
use solana_client_wasm::solana_sdk::signature::Keypair;

pub fn get_keypair() -> Keypair {
    let key = "keypair";
    let value = LocalStorage::get(key).ok().unwrap_or_else(|| {
        let x = Keypair::new().to_base58_string();
        LocalStorage::set(key, &x).ok();
        x
    });
    Keypair::from_base58_string(&value)
}
