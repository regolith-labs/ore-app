use dioxus::prelude::*;

use crate::{
    components::{Col, Heading},
    hooks::use_download_url,
};

pub fn Download() -> Element {
    let url = use_download_url();

    rsx! {
        Col {
            class: "w-full h-full max-w-2xl mx-auto px-5 sm:px-8 pb-20 sm:pb-16",
            gap: 8,
            Heading {
                class: "w-full",
                title: "Download",
                subtitle: "Install the ORE desktop app."
            }

            a {
                class: "flex controls-primary w-full h-12 rounded-full",
                href: url,
                span {
                    class: "mx-auto my-auto",
                    "Download"
                }
            }
        }
    }
}
