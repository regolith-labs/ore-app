use dioxus::prelude::*;

use crate::components::{IsToolbarOpen, MinerToolbarInsufficientFunds, PlayIcon, Tutorial};

#[component]
pub fn MinerToolbarNotStarted(cx: Scope) -> Element {
    let is_toolbar_open = use_shared_state::<IsToolbarOpen>(cx).unwrap();
    if is_toolbar_open.read().0 {
        render! {
            MinerToolbarInsufficientFunds {}
        }
    } else {
        render! {
            div {
                class: "flex flex-row w-full justify-between my-auto px-4 sm:px-8",
                p {
                    class: "font-semibold my-auto",
                    "Stopped"
                }
                div {
                    class: "flex flex-row gap-2 sm:gap-4",
                    StartButton {}
                }
                Tutorial {}
            }
        }
    }
}

#[component]
pub fn StartButton(cx: Scope) -> Element {
    let is_toolbar_open = use_shared_state::<IsToolbarOpen>(cx).unwrap();
    render! {
        button {
            class: "transition transition-colors flex w-10 h-10 justify-center rounded-full hover:bg-gray-200 active:bg-gray-300 dark:hover:bg-gray-800 dark:active:bg-gray-700",
            title: "Start mining",
            onclick: move |_e| {
                *is_toolbar_open.write() = IsToolbarOpen(true);
            },
            PlayIcon {
                class: "w-6 h-6 my-auto"
            }
        }
    }
}
