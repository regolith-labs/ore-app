use dioxus::prelude::*;

use crate::components::{Col, Heading};

pub fn Vault() -> Element {
    rsx! {
        Col {
            class: "w-full h-full pb-20 sm:pb-16",
            gap: 8,
            Heading {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                title: "Vault",
                subtitle: "Stake unpaired ORE to earn the idle yield rate."
            }
            "Coming soon"
        }
    }
}
