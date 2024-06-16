use dioxus::prelude::*;
use ore::{state::Proof, utils::AccountDeserialize};
use solana_client_wasm::solana_sdk::pubkey::Pubkey;

use crate::gateway::{proof_pubkey, AsyncResult};

use super::{use_gateway, use_pubkey};

#[derive(Clone)]
pub struct ProofHandle(UseFuture);

impl ProofHandle {
    pub fn restart(&mut self) {
        self.0.restart();
    }
}

pub fn use_proof() -> Signal<AsyncResult<Proof>> {
    use_context::<Signal<AsyncResult<Proof>>>()
}

pub fn use_proof_provider() {
    use_context_provider::<Signal<AsyncResult<Proof>>>(|| Signal::new(AsyncResult::Loading));
    let mut proof = use_context::<Signal<AsyncResult<Proof>>>();
    let pubkey = use_pubkey();
    let proof_pubkey = proof_pubkey(pubkey);
    let gateway = use_gateway().clone();

    let f = use_future(move || {
        let gateway = gateway.clone();
        async move {
            if let Ok(data) = gateway.rpc.get_account_data(&proof_pubkey).await {
                if let Ok(p) = Proof::try_from_bytes(data.as_ref()) {
                    proof.set(AsyncResult::Ok(*p));
                }
            }
        }
    });

    use_context_provider::<ProofHandle>(|| ProofHandle(f.clone()));
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
