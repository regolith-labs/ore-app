use dioxus::prelude::*;

use crate::components::PageTitle;

pub fn Pay() -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-8 w-full",
            PageTitle {
                title: "Pay"
            }
        }
    }
}
