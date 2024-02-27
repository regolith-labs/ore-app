use dioxus::prelude::*;
use dioxus_std::utils::channel::UseChannel;

use crate::miner::{Miner, MiningResult};

pub fn use_miner<'a>(cx: &'a ScopeState, ch: &'a UseChannel<MiningResult>) -> &'a UseState<Miner> {
    use_state(cx, || Miner::new(ch))
}
