use dioxus::prelude::*;
use futures::StreamExt;
use gloo_worker::Spawnable;
use ore_miner_web::Miner;

type FromMiner = Signal<String, SyncStorage>;
type ToMiner = Coroutine<String>;
/// two way channel between us and miner (web worker)
pub fn use_miner() -> (FromMiner, ToMiner) {
    // from miner receiver
    let from_miner = use_signal_sync(|| "init".to_string());
    // to miner sender
    let to_miner = use_coroutine(|mut rx| async move {
        to_owned![from_miner];
        // build new miner
        let mut spawner = Miner::spawner();
        let miner = spawner
            // callback for miner to send messages back to us
            .callback(move |msg| {
                log::info!("from worker: {:?}", msg);
                wasm_bindgen_futures::spawn_local(async move {
                    // send message back to us
                    from_miner.set(msg);
                })
            })
            // spawn new miner
            .spawn(Miner::path());
        // miner listen for messages from us
        while let Some(msg) = rx.next().await {
            // send message from us to miner
            log::info!("msg received: {:?}", msg);
            miner.send(msg);
        }
    });
    // two way channel
    (from_miner, to_miner)
}
