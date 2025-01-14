use dioxus::prelude::*;

/// two way channel between us and miner (web worker)
pub fn use_miner_provider() {
    // from miner receiver
    let _from_miner = use_context_provider(|| Signal::new(ore_miner_web::OutputMessage::Init));
    // to miner sender
    let _to_miner: Coroutine<ore_miner_web::InputMessage> =
        use_coroutine(move |mut _rx| async move {});
}
