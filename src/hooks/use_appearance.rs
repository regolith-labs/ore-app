use dioxus::prelude::*;

use crate::{components::Appearance, hooks::use_persistent::use_persistent};

const KEY: &str = "appearance";

pub fn use_appearance() -> Signal<Appearance> {
    let appearance = use_context::<Signal<Appearance>>();
    let mut appearance_persistent = use_persistent(KEY, || Appearance::Dark);
    use_effect(move || appearance_persistent.set(*appearance.read()));
    appearance
}

pub fn use_appearance_provider() {
    let appearance = use_persistent(KEY, || Appearance::Dark).get();
    use_context_provider(|| Signal::new(appearance));
}
