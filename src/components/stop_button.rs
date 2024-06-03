use dioxus::prelude::*;

use crate::{
    components::PauseIcon,
    hooks::{MinerStatus, MinerToolbarState, UpdateMinerToolbarState},
    miner::Miner,
};

#[component]
pub fn StopButton(miner: Signal<Miner>) -> Element {
    let mut toolbar_state = use_context::<Signal<MinerToolbarState>>();
    rsx! {
        button {
            class: "transition transition-colors flex-shrink-0 flex w-10 h-10 justify-center rounded-full hover:bg-green-600 active:bg-green-700",
            title: "Stop mining",
            onclick: move |_e| {
                miner.read().stop();
                toolbar_state.set_status(MinerStatus::NotStarted);
                toolbar_state.set_is_open(false);
            },
            PauseIcon {
                class: "w-6 h-6 my-auto"
            }
        }
    }
}
