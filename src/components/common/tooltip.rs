use dioxus::prelude::*;

#[component]
pub fn Tooltip(text: String) -> Element {
    rsx! {
        span {
            class: "absolute top-1/2 -translate-y-1/2 left-full translate-x-2 opacity-0 invisible group-hover:opacity-100 group-hover:visible transition-all duration-200 bg-surface-floating border border-gray-800 rounded p-2 text-elements-midEmphasis text-sm w-max max-w-64 z-40",
            "{text}"
        }
    }
}
