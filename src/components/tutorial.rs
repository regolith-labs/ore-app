use dioxus::prelude::*;

pub fn Tutorial() -> Element {
    rsx! {
        div {
            class: "absolute right-4 sm:right-8 bottom-20 px-3 py-2 animate-bounce bg-green-500 text-white rounded shadow-sm",
            p {
                class: "font-medium",
                "Click here to start mining."
            }
        }
    }
}
