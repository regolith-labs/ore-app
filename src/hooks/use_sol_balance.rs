use dioxus::prelude::*;

use crate::gateway::{escrow_pubkey, GatewayError, GatewayResult};

use super::{
    use_gateway,
    use_wallet_adapter::{use_wallet_adapter, WalletAdapter},
};

pub fn use_sol_balance() -> Resource<GatewayResult<u64>> {
    let wallet_adapter = use_wallet_adapter();
    use_resource(move || async move {
        match *wallet_adapter.read() {
            WalletAdapter::Disconnected => Err(GatewayError::AccountNotFound.into()),
            WalletAdapter::Connected(pubkey) => use_gateway()
                .rpc
                .get_balance(&pubkey)
                .await
                .map_err(GatewayError::from),
        }
    })
}

pub fn use_escrow_sol_balance() -> Resource<GatewayResult<u64>> {
    let wallet_adapter = use_wallet_adapter();
    use_resource(move || async move {
        match *wallet_adapter.read() {
            WalletAdapter::Disconnected => Err(GatewayError::AccountNotFound.into()),
            WalletAdapter::Connected(pubkey) => use_gateway()
                .rpc
                .get_balance(&escrow_pubkey(pubkey))
                .await
                .map_err(GatewayError::from),
        }
    })
}
