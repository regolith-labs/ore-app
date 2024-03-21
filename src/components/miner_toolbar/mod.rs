mod miner_charts;
mod miner_toolbar_activating;
mod miner_toolbar_active;
mod miner_toolbar_insufficient_sol;
mod miner_toolbar_not_started;
mod utils;

use dioxus_std::utils::channel::use_channel;
pub use miner_charts::*;
pub use miner_toolbar_activating::*;
pub use miner_toolbar_active::*;
pub use miner_toolbar_insufficient_sol::*;
pub use miner_toolbar_not_started::*;
#[cfg(feature = "web")]
use solana_client_wasm::solana_sdk::keccak::Hash as KeccakHash;
#[cfg(feature = "desktop")]
use solana_sdk::keccak::Hash as KeccakHash;

pub use utils::*;

use dioxus::prelude::*;

use crate::{
    hooks::{use_gateway, use_miner, use_priority_fee, use_pubkey},
    miner::{submit_solution, MiningResult},
    ProofHandle,
};

#[derive(Debug)]
pub enum MinerStatus {
    NotStarted,
    Activating,
    Active,

    // TODO Add error field
    NetworkError,
}

#[derive(Copy, Clone, Debug)]
pub enum MinerStatusMessage {
    CreatingTokenAccount,
    GeneratingChallenge,
    Searching,
    Submitting,
    Error,
}

#[derive(Debug)]
pub struct MinerDisplayHash(pub KeccakHash);

#[derive(Debug)]
pub struct IsToolbarOpen(pub bool);

#[component]
pub fn MinerToolbar(cx: Scope<MinerToolbarProps>, hidden: bool) -> Element {
    use_shared_state_provider(cx, || MinerStatus::NotStarted);
    use_shared_state_provider(cx, || MinerStatusMessage::Searching);
    use_shared_state_provider(cx, || MinerDisplayHash(KeccakHash::new_unique()));
    let miner_status = use_shared_state::<MinerStatus>(cx).unwrap();
    let miner_status_message = use_shared_state::<MinerStatusMessage>(cx).unwrap();
    let miner_display_hash = use_shared_state::<MinerDisplayHash>(cx).unwrap();
    let is_toolbar_open = use_shared_state::<IsToolbarOpen>(cx).unwrap();
    let priority_fee = use_priority_fee(cx);
    let gateway = use_gateway(cx);
    let proof_ = cx.consume_context::<ProofHandle>().unwrap();
    let ch = use_channel::<MiningResult>(cx, 1);
    let miner = use_miner(cx, ch);
    let pubkey = use_pubkey(cx);

    let _ = use_future(cx, miner_status_message, |_| {
        let display_hash = miner_display_hash.clone();
        let msg = miner_status_message.clone();
        async move {
            loop {
                async_std::task::sleep(std::time::Duration::from_millis(75)).await;
                if let MinerStatusMessage::Searching = *msg.read() {
                    *display_hash.write() = MinerDisplayHash(KeccakHash::new_unique());
                } else {
                    break;
                }
            }
        }
    });

    // Listen for results from miner.
    // Submit for validation and start mining next hash.
    let _ = use_future(cx, (), |_| {
        let mut rx = ch.clone().receiver();
        let status = miner_status.clone();
        let miner = miner.clone();
        let gateway = gateway.clone();
        let proof_ = proof_.clone();
        let miner_status_message = miner_status_message.clone();
        let miner_display_hash = miner_display_hash.clone();
        let priority_fee = priority_fee.clone();
        async move {
            while let Ok(res) = rx.recv().await {
                'submit: loop {
                    *miner_display_hash.write() = MinerDisplayHash(res.hash);
                    *miner_status_message.write() = MinerStatusMessage::Submitting;
                    let priority_fee = priority_fee.read().0;
                    match submit_solution(&gateway, &res, priority_fee).await {
                        Ok(_sig) => {
                            proof_.restart();
                            if let MinerStatus::Active = *status.read() {
                                if let Ok(treasury) = gateway.get_treasury().await {
                                    if let Ok(proof) = gateway.get_proof(pubkey).await {
                                        miner.start_mining(
                                            proof.hash.into(),
                                            treasury.difficulty.into(),
                                            pubkey,
                                        );
                                        *miner_status_message.write() =
                                            MinerStatusMessage::Searching;
                                        break 'submit;
                                    }
                                }
                            }
                        }
                        Err(err) => {
                            *miner_status_message.write() = MinerStatusMessage::Error;
                            log::error!("Failed to submit hash: {:?}", err);
                        }
                    }
                }
            }
        }
    });

    let is_open = is_toolbar_open.read().0;
    let class =
        "fixed transition-height transition-colors flex flex-row justify-between inset-x-0 bottom-0 drop-shadow-md";
    let height = if is_open {
        "max-h-[80vh] h-min overflow-y-scroll"
    } else {
        "h-16 cursor-pointer"
    };

    let bg = match *miner_status.read() {
        MinerStatus::Active => "bg-green-500 text-white",
        MinerStatus::NetworkError => "bg-red-500 text-white",
        MinerStatus::NotStarted => {
            if is_open {
                "bg-white dark:bg-black"
            } else {
                "bg-gray-100 dark:bg-gray-900"
            }
        }
        _ => "bg-gray-100 dark:bg-gray-900",
    };

    let display = if *hidden { "hidden" } else { "" };

    render! {
        div {
            class: "{class} {height} {bg} {display}",
            onclick: move |_e| {
                *is_toolbar_open.write() = IsToolbarOpen(true);
            },
            div {
                class: "flex flex-row justify-between w-full max-w-[96rem] mx-auto",
                match *miner_status.read() {
                    MinerStatus::NotStarted => {
                        render! {
                            MinerToolbarNotStarted {}
                        }
                    }
                    MinerStatus::Activating => {
                        render! {
                            MinerToolbarActivating {
                                miner: miner.clone()
                            }
                        }
                    }
                    MinerStatus::Active => {
                        render! {
                            MinerToolbarActive {
                                miner: miner.clone()
                            }
                        }
                    }
                    MinerStatus::NetworkError => {
                        // TODO
                        None
                    }
                }
            }
        }
    }
}
