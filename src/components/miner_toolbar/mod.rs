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
pub use utils::*;

use dioxus::prelude::*;

use crate::{
    gateway::{mine, submit_solution, AsyncResult},
    hooks::{use_ore_supply, use_proof, use_treasury, use_webworker},
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
    let treasury = use_treasury(cx);
    let proof = use_proof(cx);
    let (ore_supply, refresh_ore_supply) = use_ore_supply(cx);
    let timer = use_state(cx, || 0u64);
    let (worker, message) = use_webworker(cx);
    let is_toolbar_open = use_shared_state::<IsToolbarOpen>(cx).unwrap();
    use_shared_state_provider(cx, || MinerStatus::NotStarted);
    let miner_status = use_shared_state::<MinerStatus>(cx).unwrap();

    let _m = use_future(cx, message, |_| {
        let message = message.read().clone();
        let status = miner_status.clone();
        let worker = worker.clone();
        async move {
            if let Some(solution) = message {
                if let Some(_sig) = submit_solution(&solution).await {
                    if let MinerStatus::Active = *status.read() {
                        mine(worker).await;
                    }
                }
            }
        }
    });

    let _n = use_future(cx, (), |_| {
        let timer = timer.clone();
        async move {
            loop {
                async_std::task::sleep(std::time::Duration::from_secs(1)).await;
                timer.set(*timer.current() + 1);
            }
        }
    });

    use_effect(cx, &proof, |_| {
        timer.set(0);
        async move {}
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
        MinerStatus::NotStarted => {
            if is_open {
                "bg-white"
            } else {
                "bg-gray-100"
            }
        }
        _ => "bg-gray-100",
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
                                timer: timer.clone(),
                                worker: worker.clone()
                            }
                        }
                    }
                    MinerStatus::Active => {
                        render! {
                            MinerToolbarActive {
                                treasury: treasury,
                                proof: proof,
                                timer: timer.clone(),
                                ore_supply: ore_supply,
                            }
                        }
                    }
                    MinerStatus::NetworkError => {
                        render! {
                            div {
                                class: "bg-red-500"
                            }
                        }
                    }
                }
            }
        }
    }
}
