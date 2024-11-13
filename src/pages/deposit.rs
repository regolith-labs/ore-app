use dioxus::prelude::*;

use crate::components::PageTitle;

pub fn Deposit() -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-8 w-full",
            PageTitle {
                title: "Deposit"
            }
        }
    }
}
