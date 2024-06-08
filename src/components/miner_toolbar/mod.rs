mod activating;
mod active;
mod error;
mod insufficient_sol;
mod layout;
mod not_started;
mod utils;

pub use activating::*;
pub use active::*;
pub use error::*;
pub use insufficient_sol::*;
pub use layout::*;
pub use not_started::*;
pub use utils::*;

use dioxus::prelude::*;
use dioxus_std::utils::channel::use_channel;
use futures_util::stream::StreamExt;
use solana_client_wasm::solana_sdk::blake3::Hash as Blake3Hash;

use crate::{
    gateway,
    hooks::{
        use_gateway, use_miner, use_miner_toolbar_state, use_priority_fee, use_pubkey, MinerStatus,
        MinerStatusMessage, ProofHandle, ReadMinerToolbarState, UpdateMinerToolbarState,
    },
    miner::{submit_solution, WebWorkerResponse},
};

#[component]
pub fn MinerToolbar(hidden: bool) -> Element {
    let mut toolbar_state = use_miner_toolbar_state();
    let priority_fee = use_priority_fee();
    let mut proof_handle = use_context::<ProofHandle>();
    let pubkey = use_pubkey();
    let gateway = use_gateway();
    let mut cx = use_channel::<WebWorkerResponse>(1);
    let mut rx = cx.receiver();
    let miner = use_miner(cx);

    use_future(move || {
        let mut rx = rx.clone();
        let mut proof_handle = proof_handle.clone();
        let gateway = gateway.clone();
        async move {
            while let Ok(res) = rx.recv().await {
                toolbar_state.set_display_hash(Blake3Hash::new_from_array(res.hash));
                toolbar_state.set_status_message(MinerStatusMessage::Submitting);
                let priority_fee = priority_fee.read().0;
                let solution = drillx::Solution::new(res.digest, res.nonce.to_le_bytes());
                match submit_solution(&gateway, solution, priority_fee).await {
                    Ok(sig) => {
                        log::info!("Success: {}", sig);
                        proof_handle.restart();
                        if let MinerStatus::Active = toolbar_state.status() {
                            toolbar_state.set_status_message(MinerStatusMessage::Searching);
                            if let Ok(proof) = gateway.get_proof(pubkey).await {
                                if let Ok(clock) = gateway.get_clock().await {
                                    let cutoff_time = proof
                                        .last_hash_at
                                        .saturating_add(60)
                                        .saturating_sub(clock.unix_timestamp)
                                        .max(0)
                                        as u64;
                                    miner
                                        .read()
                                        .start_mining(proof.challenge.into(), cutoff_time)
                                        .await;
                                }
                            }
                        }
                    }
                    Err(err) => {
                        toolbar_state.set_status_message(MinerStatusMessage::Error);
                        log::error!("Failed to submit hash: {:?}", err);
                    }
                }
            }
        }
    });

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
