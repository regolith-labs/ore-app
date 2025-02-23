use dioxus::prelude::*;

#[component]
pub fn Spinner(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        div {
            class: "{class} w-6 h-6 border-4 rounded-full border-t-elements-highEmphasis border-elements-highEmphasis/30 animate-spin"
        }
    }
}
