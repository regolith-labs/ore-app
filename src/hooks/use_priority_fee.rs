use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::hooks::use_persistent::use_persistent;

const KEY: &str = "priority_fee";

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct PriorityFee(pub u64);

pub fn use_priority_fee() -> Signal<PriorityFee> {
    let priority_fee = use_context::<Signal<PriorityFee>>();
    let mut priority_fee_persistent = use_persistent(KEY, || PriorityFee(0));
    use_effect(move || priority_fee_persistent.set(*priority_fee.read()));
    priority_fee
}

pub fn use_priority_fee_provider() {
    let priority_fee = use_persistent(KEY, || PriorityFee(0)).get();
    use_context_provider(|| Signal::new(priority_fee));
}
