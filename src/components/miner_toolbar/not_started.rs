use dioxus::prelude::*;

use crate::{
    components::PlayIcon,
    hooks::{use_miner_toolbar_state, ReadMinerToolbarState, UpdateMinerToolbarState},
};

pub fn MinerToolbarNotStarted() -> Element {
    rsx! {
        div {
            class: "flex flex-row w-full justify-end my-auto px-4 sm:px-8",
            div {
                class: "flex flex-row gap-2 sm:gap-4",
                StartButton {}
            }
        }
    }
}

pub fn StartButton() -> Element {
    let mut toolbar_state = use_miner_toolbar_state();
    let bg_color = match toolbar_state.status() {
        crate::hooks::MinerStatus::Error => "bg-red-500 hover:bg-red-600 active:bg-red-800",
        _ => "bg-green-500 hover:bg-green-600 active:bg-green-700",
    };
    rsx! {
        button {
            class: "transition transition-colors flex flex-row gap-2 w-full h-10 px-4 justify-center rounded-full text-white text-nowrap disabled:opacity-50 {bg_color}",
            onclick: move |e| {
                toolbar_state.start();
                e.stop_propagation();
            },
            PlayIcon {
                class: "w-6 h-6 my-auto"
            }
            p {
                class: "my-auto font-semibold",
                "Mine"
            }
        }
    }
}
