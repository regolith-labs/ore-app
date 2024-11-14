use dioxus::prelude::*;

use crate::components::{Col, PageTitle};

pub fn Pay() -> Element {
    rsx! {
        Col {
            class: "w-full",
            gap: 8,
            PageTitle {
                title: "Pay"
            }
        }
    }
}
