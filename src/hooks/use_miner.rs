use dioxus::prelude::*;
use dioxus_std::utils::channel::{use_channel, UseChannel};
use solana_client_wasm::solana_sdk::blake3::Hash as Blake3Hash;

use super::{
    use_gateway, use_miner_toolbar_state, use_power_level, use_priority_fee, MinerStatus,
    MinerStatusMessage, ProofHandle,
};
use crate::miner::{submit_solution, Miner, WebWorkerResponse};

pub fn use_miner(cx: UseChannel<WebWorkerResponse>) -> Signal<Miner> {
    let mut toolbar_state = use_miner_toolbar_state();
    let power_level = use_power_level();
    let priority_fee = use_priority_fee();
    use_signal(|| Miner::new(cx, power_level, priority_fee))
}
