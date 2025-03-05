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

pub fn use_miner_cores_provider() {
    use_context_provider(|| Signal::new(1usize));
}

pub fn use_miner_cores() -> Signal<usize> {
    use_context()
}

pub fn use_miner_cpu_utilization() -> Signal<Vec<f32>> {
    let (from_miner, _to_miner) = use_miner();
    let mut signal = use_signal(|| vec![]);
    use_memo(move || {
        if let OutputMessage::TimeRemaining(_, vec) = &*from_miner.read() {
            signal.set(vec.clone());
        }
    });
    signal
}
