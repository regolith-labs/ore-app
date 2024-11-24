use dioxus::prelude::*;

use crate::{components::*, hooks::ASSETS};

#[component]
pub fn Pair(pair: String) -> Element {
    rsx! {
        Col {
            class: "w-full px-5 sm:px-8",
            gap: 4,
            Breadcrumbs {}
            Heading {
                pair: pair.clone()
            }
        }
    }
}

#[component]
fn Heading(pair: String) -> Element {
    let ticker = pair.strip_suffix("-ORE").expect("Invalid pair format");
    let asset = ASSETS.get(ticker).expect("Asset not found");
    rsx! {
        Row {
            gap: 4,
            Row {
                class: "w-[4.5rem]",
                img {
                    class: "w-10 h-10 rounded-full",
                    src: "{asset.image}"
                }
                img {
                    class: "w-10 h-10 rounded-full -ml-2",
                    src: "icon.png"
                }
            }
            span {
                class: "font-wide text-2xl font-semibold my-auto translate-y-[1px]",
                "{pair}"
            }
        }
    }
}