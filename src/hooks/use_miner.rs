use dioxus::prelude::*;
use dioxus_std::utils::channel::{use_channel, UseChannel};
use solana_client_wasm::solana_sdk::blake3::Hash as Blake3Hash;

use super::{
    use_gateway, use_miner_toolbar_state, use_power_level, use_priority_fee, use_pubkey,
    MinerStatus, MinerStatusMessage, ProofHandle,
};
use crate::miner::{submit_solution, Miner, WebWorkerResponse, WEB_WORKERS};

pub fn use_miner() -> Signal<Miner> {
    let mut cx = use_channel::<WebWorkerResponse>(WEB_WORKERS);
    let mut rx = cx.receiver();
    let mut toolbar_state = use_miner_toolbar_state();
    let power_level = use_power_level();
    let priority_fee = use_priority_fee();
    let mut proof_handle = use_context::<ProofHandle>();
    let pubkey = use_pubkey();
    let gateway = use_gateway();
    let miner = use_signal(|| Miner::new(cx, power_level, priority_fee));

    // Process web worker results
    use_future(move || {
        let mut rx = rx.clone();
        let mut proof_handle = proof_handle.clone();
        let gateway = gateway.clone();
        async move {
            let mut messages = vec![];
            while let Ok(msg) = rx.recv().await {
                messages.push(msg);
                if messages.len().ge(&WEB_WORKERS) {
                    miner
                        .read()
                        .process_web_worker_results(
                            &messages,
                            &mut toolbar_state,
                            priority_fee,
                            &mut proof_handle,
                            gateway.clone(),
                            pubkey,
                        )
                        .await;
                    messages.clear();
                }
            }
        }
    });

    miner
}
