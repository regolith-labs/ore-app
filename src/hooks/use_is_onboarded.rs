use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::hooks::use_persistent::use_persistent;

static KEY: &str = "is_onboarded";

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct IsOnboarded(pub bool);

pub fn use_is_onboarded(cx: &ScopeState) -> &UseSharedState<IsOnboarded> {
    let is_onboarded = use_shared_state::<IsOnboarded>(cx).unwrap();
    let is_onboarded_persistent = use_persistent(cx, KEY, || IsOnboarded(false));
    use_effect(cx, is_onboarded, |_| {
        is_onboarded_persistent.set(*is_onboarded.read());
        async move {}
    });
    is_onboarded
}

pub fn use_is_onboarded_provider(cx: &ScopeState) {
    let is_onboarded = use_persistent(cx, KEY, || IsOnboarded(false)).get();
    use_shared_state_provider(cx, || is_onboarded);
}
