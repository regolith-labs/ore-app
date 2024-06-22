use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{hooks::use_persistent::use_persistent, miner::WEB_WORKERS};

const KEY: &str = "power_level";

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct PowerLevel(pub u64);

pub fn use_power_level() -> Signal<PowerLevel> {
    let power_level = use_context::<Signal<PowerLevel>>();
    let mut power_level_persistent = use_persistent(KEY, || PowerLevel(*WEB_WORKERS as u64));
    use_effect(move || power_level_persistent.set(*power_level.read()));
    power_level
}

pub fn use_power_level_provider() {
    let power_level = use_persistent(KEY, || PowerLevel(*WEB_WORKERS as u64)).get();
    use_context_provider(|| Signal::new(power_level));
}
