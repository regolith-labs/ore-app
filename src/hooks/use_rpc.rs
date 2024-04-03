use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{gateway::RPC_URL, hooks::use_persistent::use_persistent};

const KEY: &str = "rpc";

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct RpcUrl(pub String);

const OLD_RPC_URL: &str = "https://amaleta-5y8tse-fast-mainnet.helius-rpc.com/";
const OLD_RPC_URL_2: &str = "https://rpc-proxy.hardhatchad.workers.dev";

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
    let rpc = use_persistent(cx, KEY, || RpcUrl(RPC_URL.to_string()));

    use_effect(cx, (), |_| {
        if rpc.get().0.eq(&OLD_RPC_URL) || rpc.get().0.eq(&OLD_RPC_URL_2) {
            rpc.set(RpcUrl(RPC_URL.to_string()));
        }
        async move {}
    });

    use_shared_state_provider(cx, || rpc.get());
}
