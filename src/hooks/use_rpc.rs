use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{gateway::RPC_URL, hooks::use_persistent::use_persistent};

const KEY: &str = "rpc";

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct RpcUrl(pub String);

pub fn use_rpc_url() -> Signal<RpcUrl> {
    let rpc = use_context::<Signal<RpcUrl>>();
    let mut rpc_persistent = use_persistent(KEY, || RpcUrl(RPC_URL.to_string()));
    use_effect(move || rpc_persistent.set(rpc.read().clone()));
    rpc
}

pub fn use_rpc_url_provider() {
    let rpc = use_persistent(KEY, || RpcUrl(RPC_URL.to_string()));
    use_context_provider(|| Signal::new(rpc.get()));
}
