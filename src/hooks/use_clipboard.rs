use dioxus::prelude::*;
use web_sys::{window, Clipboard};

pub fn use_clipboard(_cx: &ScopeState) -> Option<Clipboard> {
    window()
        .expect("Failed to get window")
        .navigator()
        .clipboard()
}
