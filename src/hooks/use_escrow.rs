use dioxus::prelude::*;
use ore_relayer_api::state::Escrow;

use crate::gateway::{GatewayError, GatewayResult};

use super::{
    use_gateway,
    use_wallet_adapter::{use_wallet_adapter, WalletAdapter},
};

pub fn use_escrow() -> Resource<GatewayResult<Escrow>> {
    let wallet_adapter = use_wallet_adapter();
    use_resource(move || async move {
        match *wallet_adapter.read() {
            WalletAdapter::Disconnected => Err(GatewayError::AccountNotFound.into()),
            WalletAdapter::Connected(pubkey) => {
                let gateway = use_gateway();
                gateway.get_escrow(pubkey).await
            }
        }
    })
}
