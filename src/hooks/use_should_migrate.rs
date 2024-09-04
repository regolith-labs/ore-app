use dioxus::prelude::*;
use ore_relayer_api::state::Escrow;

use crate::gateway::escrow_pubkey;

use super::{use_gateway, use_wallet_adapter::use_wallet_adapter};

pub fn use_should_migrate() -> Signal<Escrow> {
    let wallet_adapter = use_wallet_adapter();

    use_resource(move || async move {
        match *wallet_adapter.read() {
            WalletAdapter::Disconnected => false,
            WalletAdapter::Connected(pubkey) => {
                let gateway = use_gateway();
                let address = escrow_pubkey(pubkey);
                match gateway.get_escrow(address).await {
                    Ok(_) => true,
                    Err(_) => false,
                }
            }
        }
    })
}
