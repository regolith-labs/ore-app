use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::hooks::use_persistent::use_persistent;

static KEY: &str = "is_onboarded";

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct IsOnboarded(pub bool);

pub fn use_is_onboarded() -> Signal<IsOnboarded> {
    let is_onboarded = use_context::<Signal<IsOnboarded>>();
    let mut is_onboarded_persistent = use_persistent(KEY, || IsOnboarded(false));
    use_effect(move || is_onboarded_persistent.set(*is_onboarded.read()));
    is_onboarded
}

pub fn use_is_onboarded_provider() {
    let is_onboarded = use_persistent(KEY, || IsOnboarded(false)).get();
    use_context_provider(|| Signal::new(is_onboarded));
}
