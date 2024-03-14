use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::hooks::use_persistent::use_persistent;

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct PowerLevel(pub u8);

const KEY: &str = "power_level";

pub fn use_power_level(cx: &ScopeState) -> &UseSharedState<PowerLevel> {
    let power_level = use_shared_state::<PowerLevel>(cx).unwrap();
    let power_level_persistent = use_persistent(cx, KEY, || PowerLevel(10));
    use_effect(cx, power_level, |_| {
        power_level_persistent.set(*power_level.read());
        async move {}
    });
    power_level
}

pub fn use_power_level_provider(cx: &ScopeState) {
    let power_level = use_persistent(cx, KEY, || PowerLevel(10)).get();
    use_shared_state_provider(cx, || power_level);
}
