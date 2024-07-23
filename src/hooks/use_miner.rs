use dioxus::prelude::*;
use dioxus_sdk::utils::channel::use_channel;

use super::{use_escrow, use_miner_toolbar_state, use_power_level, use_priority_fee};
use crate::miner::{Miner, WebWorkerResponse, WEB_WORKERS};

pub fn use_miner() -> Signal<Miner> {
    let mut cx = use_channel::<WebWorkerResponse>(*WEB_WORKERS);
    let mut toolbar_state = use_miner_toolbar_state();
    let power_level = use_power_level();
    let priority_fee = use_priority_fee();
    let miner = use_signal(|| Miner::new(cx.clone(), power_level, priority_fee));
    let mut escrow = use_escrow();

    // Process web worker results
    use_future(move || {
        let mut rx = cx.receiver();
        async move {
            let mut messages = vec![];
            while let Ok(msg) = rx.recv().await {
                log::info!("Got message: {:?}", msg);
                messages.push(msg);
                if messages.len().ge(&WEB_WORKERS) {
                    miner
                        .read()
                        .process_web_worker_results(&messages, &mut toolbar_state, &mut escrow)
                        .await;
                    messages.clear();
                }
            }
        }
    });

    miner
}
