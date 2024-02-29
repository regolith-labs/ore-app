#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::{hooks::use_is_onboarded, Route};

#[component]
pub fn Landing(cx: Scope) -> Element {
    let is_onboarded = use_is_onboarded(cx);
    let nav = use_navigator(cx);

    // If the user is already onboarded, redirect to home.
    if is_onboarded.get() {
        nav.replace(Route::Home {});
    }

    render! {
        img {
            class: "fixed top-0 left-0 w-screen -z-50",
            src: "/smoke.png"
        }
        div {
            class: "flex flex-col gap-y-8 my-auto pb-24",
            p {
                class: "text-center text-6xl font-bold",
                "It's time to "
                span {
                    class: "inline-block text-white leading-relaxed bg-green-500 px-2",
                    "mine."
                }
            }
            Link {
                class: "mx-auto text-2xl font-semibold hover:underline",
                to: Route::Home {},
                "Start →"
            }
        }
    }
}
