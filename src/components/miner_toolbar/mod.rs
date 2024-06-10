mod activating;
mod active;
mod error;
mod insufficient_sol;
mod layout;
mod not_started;
mod utils;

use std::rc::Rc;

pub use activating::*;
pub use active::*;
use drillx::Solution;
pub use error::*;
pub use insufficient_sol::*;
pub use layout::*;
pub use not_started::*;
use ore::process_instruction;
pub use utils::*;

use dioxus::prelude::*;
use dioxus_std::utils::channel::use_channel;
use futures_util::stream::StreamExt;
use solana_client_wasm::solana_sdk::{blake3::Hash as Blake3Hash, pubkey::Pubkey};

use crate::{
    gateway::{self, Gateway},
    hooks::{
        use_miner, use_miner_toolbar_state, MinerStatus, MinerStatusMessage, MinerToolbarState,
        PriorityFee, ProofHandle, ReadMinerToolbarState, UpdateMinerToolbarState,
    },
    miner::{submit_solution, WebWorkerResponse, WEB_WORKERS},
};

#[component]
pub fn MinerToolbar(hidden: bool) -> Element {
    let mut toolbar_state = use_miner_toolbar_state();
    let miner = use_miner();

    let class =
        "fixed transition-height transition-colors flex flex-row justify-between inset-x-0 bottom-0 drop-shadow-md";
    let height = if toolbar_state.is_open() {
        "max-h-[80vh] shrink overflow-y-scroll"
    } else {
        "h-16 cursor-pointer"
    };

    let bg = match toolbar_state.status() {
        MinerStatus::Active => "bg-green-500 text-white",
        MinerStatus::Error => "bg-red-500 text-white",
        MinerStatus::NotStarted => {
            if toolbar_state.is_open() {
                "bg-white dark:bg-gray-900"
            } else {
                "bg-gray-100 dark:bg-gray-900"
            }
        }
        _ => "bg-gray-100 dark:bg-gray-900",
    };

    let display = if hidden { "hidden" } else { "" };

    rsx! {
        div {
            class: "{class} {height} {bg} {display}",
            onclick: move |e| {
                toolbar_state.set_is_open(true);
                e.cancel_bubble();
            },
            div {
                class: "flex flex-row justify-between w-full max-w-[96rem] mx-auto h-full",
                match toolbar_state.status() {
                    MinerStatus::NotStarted => {
                        rsx! {
                            MinerToolbarNotStarted {}
                        }
                    }
                    MinerStatus::Activating => {
                        rsx! {
                            MinerToolbarActivating { miner }
                        }
                    }
                    MinerStatus::Active => {
                        rsx! {
                            MinerToolbarActive { miner }
                        }
                    }
                    MinerStatus::Error => {
                        rsx! {
                            MinerToolbarError {}
                        }
                    }
                    _ => None
                }
            }
        }
    }
}
