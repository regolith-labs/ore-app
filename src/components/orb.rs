use dioxus::prelude::*;

#[component]
pub fn Orb(
    class: Option<String>,
    is_gold: bool,
    is_large: Option<bool>,
) -> Element {
    let animation = if is_gold { "animate-gold-orb" } else { "animate-black-orb" };
    let class = class.unwrap_or_default();
    let size = if is_large.unwrap_or(false) { "w-56 h-56" } else { "w-32 h-32" };
    rsx! {
        div {
            class: "relative {size} bg-cover bg-no-repeat {animation} {class}",
        }
    }
}
