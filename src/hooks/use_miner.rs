use dioxus::prelude::*;

use ore_miner_types::{InputMessage, OutputMessage};

#[cfg(not(feature = "web"))]
pub use super::use_miner_native::use_miner_provider;
#[cfg(feature = "web")]
pub use super::use_miner_web::use_miner_provider;

type FromMiner = Signal<OutputMessage>;
type ToMiner = Coroutine<InputMessage>;

pub fn use_miner() -> (FromMiner, ToMiner) {
    let from = use_context::<Signal<OutputMessage>>();
    let to = use_coroutine_handle::<InputMessage>();
    (from, to)
}

#[derive(Clone)]
pub struct IsActiveMiner(pub bool);
pub fn use_miner_is_active_provider() {
    use_context_provider(|| Signal::new(IsActiveMiner(false)));
}
pub fn use_miner_is_active() -> Signal<IsActiveMiner> {
    use_context::<Signal<IsActiveMiner>>()
}
