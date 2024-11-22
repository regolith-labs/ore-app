use dioxus::prelude::*;

use crate::components::*;

#[component]
pub fn Subheading(class: Option<String>, title: String, subtitle: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        Col {
            gap: 2,
            class: "{class}",
            span {
                class: "font-wide text-base sm:text-lg font-semibold",
                "{title}"
            }
            if let Some(subtitle) = subtitle {
                span {
                    class: "text-elements-lowEmphasis text-sm",
                    "{subtitle}"
                }
            }
        }
    }
}