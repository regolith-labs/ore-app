mod miner_charts;
mod miner_toolbar_activating;
mod miner_toolbar_active;
mod miner_toolbar_insufficient_sol;
mod miner_toolbar_not_started;
mod utils;

pub use miner_charts::*;
pub use miner_toolbar_activating::*;
pub use miner_toolbar_active::*;
pub use miner_toolbar_insufficient_sol::*;
pub use miner_toolbar_not_started::*;
use ore::TREASURY_ADDRESS;

pub use utils::*;

use dioxus::prelude::*;

use crate::{
    gateway::{mine, submit_solution, AsyncResult},
    hooks::{
        use_account_subscribe, use_gateway, use_ore_supply, use_proof, use_treasury, use_webworker,
    },
};

#[derive(Debug)]
pub enum MinerStatus {
    NotStarted,
    Activating,
    Active,

    // TODO Add error field
    NetworkError,
}

pub struct IsToolbarOpen(pub bool);

#[derive(Props, PartialEq)]
pub struct MinerToolbarProps {
    pub hidden: bool,
}

#[component]
pub fn MinerToolbar(cx: Scope<MinerToolbarProps>) -> Element {
    let gateway = use_gateway(cx);
    let (treasury_rw, _) = use_treasury(cx);
    let treasury = *treasury_rw.read().unwrap();
    let (proof_rw, proof_fut) = use_proof(cx);
    let proof = *proof_rw.read().unwrap();
    let (ore_supply, refresh_ore_supply) = use_ore_supply(cx);
    let (worker, message) = use_webworker(cx);
    let is_toolbar_open = use_shared_state::<IsToolbarOpen>(cx).unwrap();

    let _ = use_account_subscribe(cx, TREASURY_ADDRESS, treasury_rw);

    use_shared_state_provider(cx, || MinerStatus::NotStarted);
    let miner_status = use_shared_state::<MinerStatus>(cx).unwrap();

    let _m = use_future(cx, message, |_| {
        let message = message.read().clone();
        let status = miner_status.clone();
        let worker = worker.clone();
        let gateway = gateway.clone();
        let proof_fut = proof_fut.clone();
        async move {
            if let Some(solution) = message {
                match submit_solution(&gateway, &solution).await {
                    Ok(_sig) => {
                        proof_fut.restart();
                        if let MinerStatus::Active = *status.read() {
                            mine(&gateway, worker).await.ok();
                        }
                    }
                    Err(err) => {
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
        "absolute transition-height transition-colors flex flex-row justify-between inset-x-0 bottom-0 drop-shadow-md";
    let height = if is_open {
        "h-2/3 overflow-y-scroll"
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

    let display = if cx.props.hidden { "hidden" } else { "" };

    render! {
        div {
            class: "{class} {height} {bg} {display}",
            onclick: move |_e| {
                *is_toolbar_open.write() = IsToolbarOpen(true);
            },
            {
                match *miner_status.read() {
                    MinerStatus::NotStarted => {
                        render! {
                            MinerToolbarNotStarted {}
                        }
                    }
                    MinerStatus::Activating => {
                        render! {
                            MinerToolbarActivating {
                                worker: worker.clone()
                            }
                        }
                    }
                    MinerStatus::Active => {
                        render! {
                            MinerToolbarActive {
                                treasury: treasury,
                                proof: proof,
                                ore_supply: ore_supply,
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
