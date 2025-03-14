use dioxus::prelude::*;

#[component]
pub fn InfoText(class: Option<String>, text: String, hidden: Signal<bool>) -> Element {
    let class = class.unwrap_or("".to_string());
    let max_height = if *hidden.read() {
        "max-h-0"
    } else {
        "max-h-96"
    };
    let opacity = if *hidden.read() {
        "opacity-0"
    } else {
        "opacity-100"
    };
    rsx! {
        div {
            class: "overflow-hidden transition-all duration-300 ease-in-out h-min {max_height} {class}",
            span {
                class: "block w-full transition-opacity duration-300 ease-in-out pt-2 text-wrap {opacity} text-elements-midEmphasis",
                "{text}"
            }
        }
    }
}
