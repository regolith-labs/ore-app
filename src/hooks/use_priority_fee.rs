use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::hooks::use_persistent::use_persistent;

const KEY: &str = "priority_fee";

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct PriorityFee(pub u64);

pub fn use_priority_fee(cx: &ScopeState) -> &UseSharedState<PriorityFee> {
    let priority_fee = use_shared_state::<PriorityFee>(cx).unwrap();
    let priority_fee_persistent = use_persistent(cx, KEY, || PriorityFee(0));
    use_effect(cx, priority_fee, |_| {
        priority_fee_persistent.set(*priority_fee.read());
        async move {}
    });
    priority_fee
}

pub fn use_priority_fee_provider(cx: &ScopeState) {
    let priority_fee = use_persistent(cx, KEY, || PriorityFee(0)).get();
    use_shared_state_provider(cx, || priority_fee);
}
