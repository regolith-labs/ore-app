use dioxus::prelude::*;

#[component]
pub fn Row(class: Option<String>, children: Element, gap: Option<u8>) -> Element {
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
pub fn Col(class: Option<String>, children: Element, gap: Option<u8>) -> Element {
    let class = class.unwrap_or("".to_string());
    let gap = gap.map_or("".to_string(), |g| format!("gap-{}", g));
    rsx! {
        div {
            class: "flex flex-col {gap} {class}",
            {children}
        }
    }
}
