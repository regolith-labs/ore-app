use dioxus::prelude::*;

use crate::components::*;

#[component]
pub fn Heading(class: Option<String>, title: String, subtitle: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        Col {
            gap: 2,
            class: "{class}",
            span {
                class: "font-wide text-3xl sm:text-4xl font-bold",
                "{title}"
            }
            if let Some(subtitle) = subtitle {
                span {
                    class: "text-elements-lowEmphasis font-medium",
                    "{subtitle}"
                }
            }
        }
    }
}

#[component]
pub fn Subheading(class: Option<String>, title: String) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        span {
            class: "text-elements-highEmphasis font-semibold text-2xl {class}",
            "{title}"
        }
    }
}
