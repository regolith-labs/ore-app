use dioxus::prelude::*;

#[component]
pub fn Heading(class: Option<String>, title: String, subtitle: String ) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        div {
            class: "flex flex-col gap-2 {class}",
            span {
                class: "font-wide text-base sm:text-lg font-semibold",
                "{title}"
            }
            span {
                class: "text-elements-lowEmphasis text-sm",
                "{subtitle}"
            }
        }
    }
} 