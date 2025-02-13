use dioxus::prelude::*;
use serde::Deserialize;
use solana_sdk::signature::Signature;

pub fn use_miner_events_provider() {
    use_context_provider(|| Signal::new(Vec::<MiningEvent>::new()));
}

pub fn use_miner_events() -> Signal<Vec<MiningEvent>> {
    use_context()
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct MiningEvent {   
    pub signature: Signature,
    pub timestamp: u64,
    pub difficulty: u64,
    pub net_reward: u64,
    pub net_base_reward: u64,
    pub net_miner_boost_reward: u64,
    pub member_difficulty: u64,
    pub member_reward: u64
}