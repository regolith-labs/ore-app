use dioxus::prelude::*;

use crate::gateway::{GatewayError, GatewayResult};

use super::{use_gateway, use_pubkey};

pub fn use_sol_balance() -> Resource<GatewayResult<u64>> {
    let address = use_pubkey();
    let gateway = use_gateway();
    use_resource(move || {
        let gateway = gateway.clone();
        async move {
            gateway
                .rpc
                .get_balance(&address)
                .await
                .map_err(GatewayError::from)
        }
    })
}
