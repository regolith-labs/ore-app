use dioxus::prelude::*;

use crate::components::{Col, PageTitle};

pub fn Download() -> Element {
    rsx! {
        Col {
            class: "w-full",
            gap: 8,
            PageTitle {
                title: "Download"
            }
        }
    }
}
