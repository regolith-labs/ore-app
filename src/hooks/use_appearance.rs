use dioxus::prelude::*;

use crate::{components::Appearance, hooks::use_persistent::use_persistent};

use super::use_persistent::UsePersistent;

pub fn use_appearance(cx: &ScopeState) -> &UseSharedState<Appearance> {
    use_shared_state::<Appearance>(cx).unwrap()
    // // let kp_str = use_persistent(cx, "keypair", || Keypair::new().to_base58_string());
    // // Keypair::from_base58_string(&kp_str.get())
    // // let appearance = use_shared_state(cx)
    // use_persistent(cx, "appearance", || Appearance::Light)
}

pub fn use_appearance_persistant(cx: &ScopeState) -> &UsePersistent<Appearance> {
    use_persistent(cx, "appearance", || Appearance::Light)
}
