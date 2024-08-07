use dioxus::prelude::*;
use dioxus_sdk::utils::channel::use_channel;

use super::{use_escrow, use_miner_toolbar_state, use_power_level, use_priority_fee, use_proof};
use crate::miner::{Miner, WebWorkerResponse, WEB_WORKERS};

pub fn use_miner() -> Signal<Miner> {
    let mut cx = use_channel::<WebWorkerResponse>(*WEB_WORKERS);
    let mut toolbar_state = use_miner_toolbar_state();
    let power_level = use_power_level();
    let priority_fee = use_priority_fee();
    let miner = use_signal(|| Miner::new(cx.clone(), power_level, priority_fee));
    let mut escrow = use_escrow();
    let proof = use_proof();

    // Process web worker results
    use_future(move || {
        let mut rx = cx.receiver();
        async move {
            let mut power_level = usize::MAX;
            let mut messages = vec![];
            while let Ok(msg) = rx.recv().await {
                if msg.power_level.lt(&power_level) {
                    power_level = msg.power_level;
                }
                messages.push(msg);
                if messages.len().gt(&power_level) {
                    miner
                        .read()
                        .process_web_worker_results(
                            &messages,
                            &mut toolbar_state,
                            &mut escrow,
                            proof,
                        )
                        .await;
                    messages.clear();
                }
            }
        }
    });

    miner
}
