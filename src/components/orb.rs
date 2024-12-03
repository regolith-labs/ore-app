use dioxus::prelude::*;

#[component]
pub fn Orb(#[props] is_gold: bool) -> Element {
    let animation = if is_gold { "animate-gold-orb" } else { "animate-black-orb" };
    rsx! {
        div {
            class: "relative w-96 h-96 bg-cover bg-no-repeat {animation}",
        }
    }
}
