use dioxus::prelude::*;

use crate::{components::Appearance, hooks::use_persistent::use_persistent};

use super::use_persistent::UsePersistent;

pub fn use_appearance(cx: &ScopeState) -> &UseSharedState<Appearance> {
    use_shared_state::<Appearance>(cx).unwrap()
}

pub fn use_appearance_persistant(cx: &ScopeState) -> &UsePersistent<Appearance> {
    use_persistent(cx, "appearance", || Appearance::Light)
}
