use dioxus::prelude::*;
use ore_pool_types::PoolMemberMiningEvent;

pub fn use_miner_events_provider() {
    use_context_provider(|| Signal::new(Vec::<PoolMemberMiningEvent>::new()));
}

pub fn use_miner_events() -> Signal<Vec<PoolMemberMiningEvent>> {
    use_context()
}