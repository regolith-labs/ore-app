use dioxus::prelude::*;

#[cfg(feature = "web")]
use crate::{
    components::{Col, Heading},
    hooks::{parse_download_url, use_download_url},
};

#[cfg(not(feature = "web"))]
pub fn Download() -> Element {
    rsx! {}
}

#[cfg(feature = "web")]
pub fn Download() -> Element {
    let url = use_download_url();
    let (os, arch) = parse_download_url(&url.cloned());

    rsx! {
        Col {
            class: "w-full h-full max-w-2xl mx-auto px-5 sm:px-8 pb-20 sm:pb-16",
            gap: 8,
            Heading {
                class: "w-full",
                title: "Download",
                subtitle: "Install the ORE desktop app."
            }

            span {
                class: "text-sm text-gray-500",
                "System: {os} {arch}"
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
