use dioxus::prelude::*;

use crate::{
    components::{try_start_mining, IsToolbarOpen, MinerStatus},
    gateway::AsyncResult,
    hooks::{use_gateway, use_sol_balance},
    miner::Miner,
};

#[component]
pub fn MinerToolbarActivating(cx: Scope, miner: UseState<Miner>) -> Element {
    let gateway = use_gateway(cx);
    let sol_balance = use_sol_balance(cx);
    let is_toolbar_open = use_shared_state::<IsToolbarOpen>(cx).unwrap();
    let miner_status = use_shared_state::<MinerStatus>(cx).unwrap();

    use_future(cx, &sol_balance.clone(), |_| {
        let miner = miner.clone();
        let miner_status = miner_status.clone();
        let gateway = gateway.clone();
        async move {
            if let AsyncResult::Ok(sol_balance) = sol_balance {
                match try_start_mining(&gateway, sol_balance, miner.get()).await {
                    Ok(did_start) => {
                        if did_start {
                            *miner_status.write() = MinerStatus::Active;
                        } else {
                            // TODO Insufficient balance... Set appropriate error
                            log::error!("Insufficient balance to start mining");
                            *miner_status.write() = MinerStatus::NetworkError;
                        };
                    }
                    Err(err) => {
                        log::error!("Failed to start mining: {:?}", err);
                        *miner_status.write() = MinerStatus::NetworkError;
                        // TODO Present error to user
                    }
                }
            }
        }
    });

    if is_toolbar_open.read().0 {
        render! {
            div {
                class: "flex flex-col grow gap-8 justify-between p-8",
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
