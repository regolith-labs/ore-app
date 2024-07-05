use dioxus::prelude::*;
use solana_extra_wasm::account_decoder::parse_token::UiTokenAmount;

use crate::gateway::{GatewayError, GatewayResult};

use super::use_gateway;

pub fn use_ore_supply() -> Resource<GatewayResult<UiTokenAmount>> {
    let gateway = use_gateway();
    use_resource(move || {
        let gateway = gateway.clone();
        async move {
            gateway
                .rpc
                .get_token_supply(&ore_api::consts::MINT_ADDRESS)
                .await
                .map_err(GatewayError::from)
        }
    })
}
