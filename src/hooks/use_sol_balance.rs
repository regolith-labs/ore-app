use dioxus::prelude::*;

use crate::gateway::{GatewayError, GatewayResult};

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
