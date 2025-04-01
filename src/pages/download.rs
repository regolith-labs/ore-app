use dioxus::prelude::*;

#[cfg(feature = "web")]
use crate::{
    components::{AppleIcon, Col, DownloadIcon, Heading, Row, WindowsIcon},
    hooks::{parse_download_url, use_download_url},
};

#[cfg(not(feature = "web"))]
pub fn Download() -> Element {
    rsx! {}
}

#[cfg(feature = "web")]
pub fn Download() -> Element {
    let url = use_download_url();
    let (os, _arch) = parse_download_url(&url.cloned());

    let os_icon = match os.as_str() {
        "macOS" => rsx! { AppleIcon { class: "w-16 h-16" } },
        "Windows" => rsx! { WindowsIcon { class: "w-16 h-16" } },
        _ => rsx! { DownloadIcon { class: "w-16 h-16" } },
    };

    let download_text = match os.as_str() {
        "macOS" => "Download for Mac",
        "Windows" => "Download for Windows",
        _ => "Download",
    };

    rsx! {
        Col {
            class: "w-full h-full max-w-2xl mx-auto px-5 sm:px-8 pb-20 sm:pb-16",
            Heading {
                class: "w-full",
                title: "Download",
                subtitle: "Install the ORE desktop app."
            }
            DownloadSection {
                os: os.clone(),
                os_icon: os_icon.clone(),
                url: url.clone(),
                download_text: download_text
            }

        }
    }
}

#[cfg(feature = "web")]
#[component]
fn DownloadSection(os: String, os_icon: Element, url: String, download_text: String) -> Element {
    rsx! {
        Col {
            class: "items-center justify-center mt-8",
            gap: 8,
            span {
                {os_icon}
            }
            span {
                class: "font-semibold text-elements-highEmphasis text-center text-3xl",
                "{os}"
            }
            if os == "Linux" {
                span {
                    class: "text-elements-midEmphasis text-center",
                    "Linux support is coming soon. Please check back later."
                }
            } else {
                a {
                    class: "flex controls-primary w-full h-12 rounded-full items-center justify-center",
                    href: url,
                    DownloadIcon {
                        class: "w-5 h-5 mr-4"
                    }
                    span {
                        class: "my-auto",
                        "{download_text}"
                    }
                }
            }

        }
    }
}
