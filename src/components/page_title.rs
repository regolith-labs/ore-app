use dioxus::prelude::*;

#[component]
pub fn PageTitle(title: String) -> Element {
    rsx! {
        span {
            class: "flex flex-row justify-between sm:hidden mx-5 sm:mx-8 h-10 font-wide text-2xl font-semibold",
            span {
                class: "my-auto",
                "{title}"
            }
        }
    }
}
