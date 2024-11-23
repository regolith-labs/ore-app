use dioxus::prelude::*;

pub fn BackButton() -> Element {
    let navigator = use_navigator();
    rsx! {
        button {
            class: "w-10 h-10 -ml-2.5 rounded-full text-bold text-elements-midEmphasis hover:bg-controls-handle",
            onclick: move |_| {
                navigator.go_back();
            },
            "←"
        }
    }
}
