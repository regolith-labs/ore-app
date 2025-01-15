use dioxus::prelude::*;
use solana_sdk::pubkey::Pubkey;

use super::Wallet;

pub fn use_wallet_provider() {
    let mut signal = use_context_provider(|| Signal::new(Wallet::Disconnected));
    signal.set(Wallet::Connected(Pubkey::default()));
}
