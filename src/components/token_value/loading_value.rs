use dioxus::prelude::*;

#[allow(dead_code)]
#[derive(Clone, PartialEq, Eq)]
pub enum LoadingValueSize {
    Small,
    Medium,
    Large,
}

#[component]
pub fn LoadingValue(size: Option<LoadingValueSize>) -> Element {
    let size_class = match size.unwrap_or(LoadingValueSize::Small) {
        LoadingValueSize::Small => "w-10 h-4 rounded",
        LoadingValueSize::Medium => "w-16 h-6 rounded",
        LoadingValueSize::Large => "w-32 h-10 rounded",
    };
    rsx! {
        span {
            class: "{size_class} my-auto loading",
            ""
        }
    }
}
