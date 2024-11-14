use dioxus::prelude::*;

#[component]
pub fn Row(gap: Option<u8>, class: Option<String>, children: Element) -> Element {
    let class = class.unwrap_or("".to_string());
    let gap = gap.map_or("".to_string(), |g| format!("gap-{}", g));
    rsx! {
        div {
            class: "flex flex-row {gap} {class}",
            {children}
        }
    }
}

#[component]
pub fn Col(gap: Option<u8>, class: Option<String>, children: Element) -> Element {
    let class = class.unwrap_or("".to_string());
    let gap = gap.map_or("".to_string(), |g| format!("gap-{}", g));
    rsx! {
        div {
            class: "flex flex-col {gap} {class}",
            {children}
        }
    }
}
