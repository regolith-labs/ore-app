use dioxus::prelude::*;

#[component]
pub fn BackButton(onclick: EventHandler) -> Element {
    rsx! {
        button {
            class: "transition-colors text-2xl -ml-2 w-10 h-10 bg-transparent hover-100 active-200 rounded-full mr-auto",
            onclick: move |_| onclick.call(()),
            "←"
        }
    }
}
