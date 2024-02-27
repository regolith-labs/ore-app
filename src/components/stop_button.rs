use dioxus::prelude::*;
#[cfg(feature = "web")]
use solana_extra_wasm::account_decoder::parse_token::UiTokenAmount;
#[cfg(feature = "web")]
use web_sys::Worker;

use crate::{
    components::{IsToolbarOpen, MinerStatus, PauseIcon},
    hooks::MiningResult,
};

static BUTTON_CLASS: &str = "transition transition-colors flex w-10 h-10 justify-center rounded-full hover:bg-green-600 active:bg-green-700";
static ICON_CLASS: &str = "w-6 h-6 my-auto";

#[cfg(feature = "web")]
#[component]
pub fn StopButton(
    cx: Scope,
    worker: UseState<Worker>,
    message: UseRef<Option<MiningResult>>,
) -> Element {
    let status = use_shared_state::<MinerStatus>(cx).unwrap();
    let is_toolbar_open = use_shared_state::<IsToolbarOpen>(cx).unwrap();
    render! {
        button {
            class: "{BUTTON_CLASS}",
            onclick: move |_e| {
                // let worker = worker.clone();
                // let message = message.clone();
                // let status = status.clone();
                // let is_toolbar_open = is_toolbar_open.clone();
                async move {
                    // stop_mining(&status, &is_toolbar_open, &worker, &message);
                }
            },
            PauseIcon {
                class: "{ICON_CLASS}"
            }
        }
    }
}

#[cfg(feature = "desktop")]
#[component]
pub fn StopButton(cx: Scope) -> Element {
    let status = use_shared_state::<MinerStatus>(cx).unwrap();
    let is_toolbar_open = use_shared_state::<IsToolbarOpen>(cx).unwrap();
    render! {
        button {
            class: "{BUTTON_CLASS}",
            onclick: move |_e| {
                let status = status.clone();
                let is_toolbar_open = is_toolbar_open.clone();
                async move {
                    // stop_mining(&status, &is_toolbar_open);
                }
            },
            PauseIcon {
                class: "{ICON_CLASS}"
            }
        }
    }
}
