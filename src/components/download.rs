#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::{hooks::use_is_onboarded, Route};

#[component]
pub fn Download(cx: Scope) -> Element {
    render! {
        div {
            class: "flex flex-col gap-16",
            div {
                class: "flex flex-col gap-4",
                h1 {
                    "Download"
                }
            }
        }
    }
}
