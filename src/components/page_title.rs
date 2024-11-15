use dioxus::prelude::*;

use crate::components::Row;

#[component]
pub fn PageTitle(title: String) -> Element {
    rsx! {
        Row {
            class: "justify-between sm:hidden mx-5 sm:mx-8 h-10 font-wide text-2xl sm:text-3xl font-semibold",
            span {
                class: "my-auto",
                "{title}"
            }
        }
    }
}
