use dioxus::prelude::*;
use dioxus_std::utils::channel::UseChannel;

use crate::miner::{Miner, MiningResult};

use super::use_power_level;

pub fn use_miner<'a>(cx: &'a ScopeState, ch: &'a UseChannel<MiningResult>) -> &'a UseState<Miner> {
    let power_level = use_power_level(cx);
    use_state(cx, || Miner::new(ch, power_level))
}
