use dioxus::prelude::*;
use web_sys::Worker;

use crate::{
    components::{try_start_mining, IsToolbarOpen, MinerStatus},
    gateway::AsyncResult,
    hooks::use_sol_balance,
};

#[derive(Props, PartialEq)]
pub struct MinerToolbarActivatingProps {
    pub timer: UseState<u64>,
    pub worker: Worker,
}

#[component]
pub fn MinerToolbarActivating(cx: Scope<MinerToolbarActivatingProps>) -> Element {
    let worker = &cx.props.worker;
    let sol_balance = use_sol_balance(cx);
    let is_toolbar_open = use_shared_state::<IsToolbarOpen>(cx).unwrap();
    let miner_status = use_shared_state::<MinerStatus>(cx).unwrap();

    use_future(cx, &sol_balance.clone(), |_| {
        let timer = cx.props.timer.clone();
        let worker = worker.clone();
        let miner_status = miner_status.clone();
        async move {
            if let AsyncResult::Ok(sol_balance) = sol_balance {
                if try_start_mining(sol_balance, &worker).await {
                    *miner_status.write() = MinerStatus::Active;
                    timer.set(0);
                } else {
                    *miner_status.write() = MinerStatus::NetworkError;
                };
            }
        }
    });

    if is_toolbar_open.read().0 {
        render! {
            div {
                class: "flex flex-col grow gap-8 justify-between p-8 bg-white",
                div {
                    class: "flex flex-col gap-3",
                    h2 {
                        class: "text-2xl font-bold",
                        "Starting"
                    }
                }
            }
        }
    } else {
        render! {
            div {
                p {
                    class: "font-medium my-auto",
                    "Starting"
                }
            }
        }
    }
}
