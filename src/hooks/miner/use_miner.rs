use dioxus::prelude::*;

use ore_miner_types::{InputMessage, OutputMessage};

type FromMiner = Signal<OutputMessage>;
type ToMiner = Coroutine<InputMessage>;

pub fn use_miner() -> (FromMiner, ToMiner) {
    let from = use_context::<Signal<OutputMessage>>();
    let to = use_coroutine_handle::<InputMessage>();
    (from, to)
}

#[derive(Clone, Debug, PartialEq)]
pub enum MinerStatus {
    Stopped,
    Registering,
    FetchingChallenge,
    Hashing,
    SubmittingSolution,
}

pub fn use_miner_status_provider() {
    use_context_provider(|| Signal::new(MinerStatus::Stopped));
}

pub fn use_miner_status() -> Signal<MinerStatus> {
    use_context()
}

pub fn use_miner_is_active() -> Memo<bool> {
    let miner_status = use_miner_status();
    use_memo(move || {
        log::info!("Miner status: {:?}", miner_status.cloned());
        match miner_status.cloned() {
            MinerStatus::FetchingChallenge
            | MinerStatus::Hashing
            | MinerStatus::SubmittingSolution => true,
            _ => false,
        }
    })
}
