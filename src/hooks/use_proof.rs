use dioxus::prelude::*;
use ore_api::state::Proof;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;

use crate::gateway::GatewayResult;

use super::use_gateway;

// pub fn use_proof() -> Resource<GatewayResult<Proof>> {
//     let authority = use_pubkey();
//     use_user_proof(authority)
// }

pub fn use_user_proof(authority: Pubkey) -> Resource<GatewayResult<Proof>> {
    let gateway = use_gateway();
    use_resource(move || {
        let gateway = gateway.clone();
        async move { gateway.get_proof(authority).await }
    })
}
