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
use ore::TREASURY_ADDRESS;

pub use utils::*;

use dioxus::prelude::*;

use crate::{
    gateway::AsyncResult,
    hooks::{
        use_account_subscribe, use_gateway, use_miner, use_ore_supply, use_pubkey, use_treasury,
    },
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

// TODO A message bar
#[derive(Debug)]
pub struct MinerStatusMessage(pub String);

pub struct IsToolbarOpen(pub bool);

#[component]
pub fn MinerToolbar(cx: Scope<MinerToolbarProps>, hidden: bool) -> Element {
    use_shared_state_provider(cx, || MinerStatus::NotStarted);
    use_shared_state_provider(cx, || MinerStatusMessage(String::new()));
    let miner_status = use_shared_state::<MinerStatus>(cx).unwrap();
    let miner_status_message = use_shared_state::<MinerStatusMessage>(cx).unwrap();
    let is_toolbar_open = use_shared_state::<IsToolbarOpen>(cx).unwrap();
    let gateway = use_gateway(cx);
    let (treasury_rw, _) = use_treasury(cx);
    let treasury = *treasury_rw.read().unwrap();
    let proof_ = cx.consume_context::<ProofHandle>().unwrap();
    let (ore_supply, refresh_ore_supply) = use_ore_supply(cx);
    let ch = use_channel::<MiningResult>(cx, 1);
    let miner = use_miner(cx, ch);
    let pubkey = use_pubkey(cx);
    let _ = use_account_subscribe(cx, TREASURY_ADDRESS, treasury_rw);

    // Listen for results from miner.
    // Submit for validation and start mining next hash.
    let _ = use_future(cx, (), |_| {
        let mut rx = ch.clone().receiver();
        let status = miner_status.clone();
        let miner = miner.clone();
        let gateway = gateway.clone();
        let proof_ = proof_.clone();
        let miner_status_message = miner_status_message.clone();
        async move {
            while let Ok(res) = rx.recv().await {
                *miner_status_message.write() =
                    MinerStatusMessage("Submitting hash for validation".to_string());
                match submit_solution(&gateway, &res).await {
                    Ok(_sig) => {
                        *miner_status_message.write() =
                            MinerStatusMessage("Success! Hash validated".to_string());
                        proof_.restart();
                        if let MinerStatus::Active = *status.read() {
                            if let Ok(treasury) = gateway.get_treasury().await {
                                if let Ok(proof) = gateway.get_proof(pubkey).await {
                                    miner.start_mining(
                                        proof.hash.into(),
                                        treasury.difficulty.into(),
                                        pubkey,
                                    );
                                }
                            }
                        }
                    }
                    Err(err) => {
                        *miner_status_message.write() =
                            MinerStatusMessage("Error validating hash".to_string());
                        log::error!("Failed to submit hash: {:?}", err);
                    }
                }
            }
        }
    });

    // If epoch resets, refresh the total supply tracker.
    let t = match &treasury {
        AsyncResult::Ok(treasury) => treasury.epoch_start_at,
        _ => 0,
    };
    let _o = use_future(cx, &t, |_| {
        refresh_ore_supply.restart();
        async move {}
    });

    let is_open = is_toolbar_open.read().0;
    let class =
        "fixed transition-height transition-colors flex flex-row justify-between inset-x-0 bottom-0 drop-shadow-md";
    let height = if is_open {
        "h-[80vh] overflow-y-scroll"
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
                                treasury: treasury,
                                // proof: proof,
                                ore_supply: ore_supply,
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
