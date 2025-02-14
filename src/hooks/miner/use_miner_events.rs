use dioxus::prelude::*;
use serde::Deserialize;
use solana_sdk::signature::Signature;
use std::collections::VecDeque;

pub fn use_miner_events_provider() {
    use_context_provider(|| Signal::new(VecDeque::<MiningEvent>::new()));
}

pub fn use_miner_events() -> Signal<VecDeque<MiningEvent>> {
    use_context()
}

impl MiningEvent {
    pub fn add_to_signal(event: MiningEvent) {
        let mut events = use_miner_events();
        let mut new_events = events.read().clone();
        
        if new_events.contains(&event) {
            return;
        }

        new_events.push_front(event);
        if new_events.len() > 15 {
            new_events.pop_back();
        }
        *events.write() = new_events;
    }
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