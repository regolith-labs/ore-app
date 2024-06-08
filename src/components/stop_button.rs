use dioxus::prelude::*;

use crate::{
    components::PauseIcon,
    hooks::{
        use_miner, use_miner_toolbar_state, MinerStatus, MinerToolbarState, UpdateMinerToolbarState,
    },
    miner::Miner,
};

pub fn StopButton() -> Element {
    let mut toolbar_state = use_miner_toolbar_state();
    rsx! {
        button {
            class: "transition transition-colors flex-shrink-0 flex w-10 h-10 justify-center rounded-full hover:bg-green-600 active:bg-green-700",
            title: "Stop mining",
            onclick: move |e| {
                // miner.read().stop();
                toolbar_state.pause();
                e.cancel_bubble();
            },
            PauseIcon {
                class: "w-6 h-6 my-auto"
            }
        }
    }
}
