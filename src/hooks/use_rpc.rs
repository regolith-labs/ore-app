use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{gateway::RPC_URL, hooks::use_persistent::use_persistent};

const KEY: &str = "rpc";

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct RpcUrl(pub String);

pub fn use_rpc_url(cx: &ScopeState) -> &UseSharedState<RpcUrl> {
    let rpc = use_shared_state::<RpcUrl>(cx).unwrap();
    let rpc_persistent = use_persistent(cx, KEY, || RpcUrl(RPC_URL.to_string()));
    use_effect(cx, rpc, |_| {
        rpc_persistent.set(rpc.read().clone());
        async move {}
    });
    rpc
}

pub fn use_rpc_url_provider(cx: &ScopeState) {
    let rpc = use_persistent(cx, KEY, || RpcUrl(RPC_URL.to_string())).get();
    use_shared_state_provider(cx, || rpc);
}
