use dioxus::prelude::*;

use crate::components::{Col, Heading};

pub fn Download() -> Element {
    rsx! {
        Col {
            class: "w-full",
            gap: 8,
            Heading {
                class: "w-full",
                title: "Download",
                subtitle: "Coming soon."
            }
        }
    }
}
