use dioxus::prelude::*;
use ore::{state::Proof, utils::AccountDeserialize};
use solana_client_wasm::solana_sdk::pubkey::Pubkey;

use crate::gateway::{proof_pubkey, AsyncResult, GatewayResult};

use super::{use_gateway, use_pubkey};

pub fn use_proof() -> Resource<GatewayResult<Proof>> {
    let pubkey = use_pubkey();
    let gateway = use_gateway();
    use_resource(move || {
        let gateway = gateway.clone();
        async move { gateway.get_proof(pubkey).await }
    })
}

pub fn use_user_proof(authority: Pubkey) -> Signal<AsyncResult<Proof>> {
    let mut proof = use_signal(|| AsyncResult::Loading);
    let gateway = use_gateway();
    use_future(move || {
        let gateway = gateway.clone();
        async move {
            let proof_pubkey = proof_pubkey(authority);
            if let Ok(data) = gateway.rpc.get_account_data(&proof_pubkey).await {
                if let Ok(p) = Proof::try_from_bytes(data.as_ref()) {
                    proof.set(AsyncResult::Ok(*p));
                }
            }
        }
    });
    proof
}
