use dioxus::prelude::*;
use ore::state::Proof;

use crate::gateway::{proof_pubkey, AsyncResult};

use super::{use_account, use_pubkey};

pub fn use_proof(cx: &ScopeState) -> AsyncResult<Proof> {
    let pubkey = use_pubkey(cx);
    let proof_pubkey = proof_pubkey(pubkey);
    use_account(cx, proof_pubkey)
}
