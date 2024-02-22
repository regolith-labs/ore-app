use dioxus::prelude::*;
use dioxus_std::utils::rw::UseRw;
use ore::state::Proof;

use crate::gateway::{proof_pubkey, AsyncResult};

use super::{use_account, use_pubkey};

pub fn use_proof(cx: &ScopeState) -> (&mut UseRw<AsyncResult<Proof>>, &UseFuture<()>) {
    let pubkey = use_pubkey(cx);
    let proof_pubkey = proof_pubkey(pubkey);
    use_account(cx, proof_pubkey)
}
