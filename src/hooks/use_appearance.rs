use dioxus::prelude::*;

use crate::{components::Appearance, hooks::use_persistent::use_persistent};

const KEY: &str = "appearance";

pub fn use_appearance(cx: &ScopeState) -> &UseSharedState<Appearance> {
    let appearance = use_shared_state::<Appearance>(cx).unwrap();
    let appearance_persistent = use_persistent(cx, KEY, || Appearance::Dark);
    use_effect(cx, appearance, |_| {
        appearance_persistent.set(*appearance.read());
        async move {}
    });
    appearance
}

pub fn use_appearance_provider(cx: &ScopeState) {
    let appearance = use_persistent(cx, KEY, || Appearance::Dark).get();
    use_shared_state_provider(cx, || appearance);
}
