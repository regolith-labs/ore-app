use dioxus::prelude::*;

use crate::{
    components::{PlayIcon, Tutorial},
    hooks::{use_miner_toolbar_state, ReadMinerToolbarState, UpdateMinerToolbarState},
};

pub fn MinerToolbarNotStarted() -> Element {
    let mut toolbar_state = use_miner_toolbar_state();

    use_effect(move || {
        if toolbar_state.is_open() {
            toolbar_state.start();
        }
    });

    rsx! {
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

pub fn StartButton() -> Element {
    let mut toolbar_state = use_miner_toolbar_state();
    rsx! {
        button {
            class: "transition transition-colors flex w-10 h-10 justify-center rounded-full hover:bg-gray-200 active:bg-gray-300 dark:hover:bg-gray-800 dark:active:bg-gray-700",
            title: "Start mining",
            onclick: move |_e| toolbar_state.start(),
            PlayIcon {
                class: "w-6 h-6 my-auto"
            }
        }
    }
}
