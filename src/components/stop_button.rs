use dioxus::prelude::*;

use crate::{
    components::{IsToolbarOpen, MinerStatus, PauseIcon},
    metrics::{track, AppEvent},
    miner::Miner,
};

#[component]
pub fn StopButton(cx: Scope, miner: UseState<Miner>) -> Element {
    let status = use_shared_state::<MinerStatus>(cx).unwrap();
    let is_toolbar_open = use_shared_state::<IsToolbarOpen>(cx).unwrap();
    render! {
        button {
            class: "transition transition-colors flex w-10 h-10 justify-center rounded-full hover:bg-green-600 active:bg-green-700",
            title: "Stop mining",
            onclick: move |_e| {
                track(AppEvent::StopMiner, None);
                miner.get().stop();
                *status.write() = MinerStatus::NotStarted;
                *is_toolbar_open.write() = IsToolbarOpen(false);
            },
            PauseIcon {
                class: "w-6 h-6 my-auto"
            }
        }
    }
}
