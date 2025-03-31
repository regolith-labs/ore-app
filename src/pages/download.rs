use dioxus::prelude::*;

#[cfg(feature = "web")]
use crate::{
    components::{AppleIcon, Col, DownloadIcon, Heading, Row},
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

            AppleIcon {
                class: "w-16 h-16"
            }

            Row {
                class: "items-center justify-center",
                gap: 8,
                DownloadCard {
                    os: os,
                    arch: arch
                }
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

#[component]
fn DownloadButton(button_text: String) -> Element {
    rsx! {
        Row {
            class: " sm:flex-row mx-auto md:ml-0 h-min mt-8 px-4 justify-center items-center",
            gap: 4,
            Link {
                to: "/",
                class: "flex h-12 w-full sm:w-min px-8 rounded-full controls-primary",
                span {
                    class: "my-auto mx-auto text-nowrap font-semibold",
                    "{button_text}"
                }
                DownloadIcon {
                    class: "w-4 sm:w-8 h-4 sm:h-8 m-auto"
                }
                // WindowsIcon {
                //     class: "w-6 sm:w-8 h-6 sm:h-8 m-auto"
                // }

            }
        }
    }
}

// fn OsIcon(os: String) -> Element {
//     rsx!{
//         Link {
//             to: "https://discord.gg/4TQfshAAsT",
//             class: "flex h-10 sm:h-12 w-10 sm:w-12 transition-colors rounded-full transition-colors {button_color} hover:bg-controls-secondaryHover my-auto",
//             new_tab: true,
//             DiscordIcon {
//                 class: "w-6 sm:w-8 h-6 sm:h-8 m-auto"
//             }
//         }
//     }
// }

#[component]
fn DownloadCard(os: String, arch: String) -> Element {
    // let class = class.unwrap_or_default();
    let img_path = "https://pbs.twimg.com/profile_images/1510345561731330063/mRH8nY7D_400x400.jpg";
    rsx! {
        Col {
            class: "w-1/2 justify-center rounded-md p-5 grow",

            Col {
                class: "gap-3 items-center",
                img {
                    class: "w-16 h-16 rounded-full ",
                    src: "{img_path}"
                }
                Col {
                class: "gap-1",
                    // p {
                    //     class: "text-elements-midEmphasis text-center",
                    //     "Desktop"
                    // }
                    span {
                        class: "font-semibold text-elements-highEmphasis text-center text-3xl",
                        "{os}"
                    }
                    // span {
                    //     class: "text-sm text-gray-500",
                    //     "System: {arch}"
                    // }

                    // p {
                    //     class: "text-sm text-elements-midEmphasis text-center",
                    //     "Download the latest version of the ORE desktop app."
                    // }
                    // DownloadButton {
                    //     button_text: "Download".to_string()
                    // }
                }
            }
        }
    }
}
