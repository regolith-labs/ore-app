use dioxus::prelude::*;
use dioxus_std::utils::channel::use_channel;

use super::{
    use_gateway, use_miner_toolbar_state, use_power_level, use_priority_fee, use_proof, use_pubkey,
};
use crate::miner::{Miner, WebWorkerResponse, WEB_WORKERS};

pub fn use_miner() -> Signal<Miner> {
    let mut cx = use_channel::<WebWorkerResponse>(WEB_WORKERS);
    let mut toolbar_state = use_miner_toolbar_state();
    let mut proof = use_proof();
    let power_level = use_power_level();
    let priority_fee = use_priority_fee();
    let pubkey = use_pubkey();
    let gateway = use_gateway();
    let miner = use_signal(|| Miner::new(cx.clone(), power_level, priority_fee));

    // Process web worker results
    use_future(move || {
        let mut rx = cx.receiver();
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
                            &mut proof,
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
