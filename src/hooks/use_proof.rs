use dioxus::prelude::*;
use ore::{state::Proof, utils::AccountDeserialize};
use solana_client_wasm::solana_sdk::pubkey::Pubkey;

use crate::gateway::{proof_pubkey, AsyncResult};

use super::{use_gateway, use_pubkey};

#[derive(Clone)]
pub struct ProofHandle(UseFuture<()>);

impl ProofHandle {
    pub fn restart(&self) {
        self.0.restart();
    }
}

pub fn use_proof(cx: &ScopeState) -> &UseSharedState<AsyncResult<Proof>> {
    use_shared_state::<AsyncResult<Proof>>(cx).unwrap()
}

pub fn use_proof_provider(cx: &ScopeState) {
    use_shared_state_provider::<AsyncResult<Proof>>(cx, || AsyncResult::Loading);
    let proof = use_shared_state::<AsyncResult<Proof>>(cx).unwrap();
    let pubkey = use_pubkey(cx);
    let proof_pubkey = proof_pubkey(pubkey);
    let gateway = use_gateway(cx);

    let f = use_future(cx, (), |_| {
        let proof = proof.clone();
        let gateway = gateway.clone();
        async move {
            if let Ok(data) = gateway.rpc.get_account_data(&proof_pubkey).await {
                if let Ok(p) = Proof::try_from_bytes(data.as_ref()) {
                    *proof.write() = AsyncResult::Ok(*p);
                }
            }
        }
    });

    cx.provide_context(ProofHandle(f.clone()));
}

pub fn use_user_proof(cx: &ScopeState, authority: Pubkey) -> AsyncResult<Proof> {
    let proof = use_state(cx, || AsyncResult::Loading);
    let gateway = use_gateway(cx);
    use_future(cx, (), |_| {
        let proof = proof.clone();
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
    *proof.get()
}
