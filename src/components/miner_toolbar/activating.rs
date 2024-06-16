use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::native_token::LAMPORTS_PER_SOL;

use crate::{
    components::{try_start_mining, Spinner},
    hooks::{
        use_gateway, use_miner_toolbar_state, use_sol_balance, MinerStatus, MinerStatusMessage,
        ReadMinerToolbarState, UpdateMinerToolbarState,
    },
    miner::Miner,
};

use super::MinerToolbarInsufficientFunds;

const MIN_BALANCE: u64 = LAMPORTS_PER_SOL.saturating_div(100);

#[component]
pub fn MinerToolbarActivating(miner: Signal<Miner>) -> Element {
    let gateway = use_gateway();
    let sol_balance = use_sol_balance();
    let mut sufficient_balance = use_signal(|| true);
    let mut toolbar_state = use_miner_toolbar_state();

    use_effect(move || {
        if let Some(Ok(sol_balance)) = *sol_balance.read() {
            sufficient_balance.set(sol_balance.ge(&MIN_BALANCE));
        } else {
            sufficient_balance.set(false);
        }
    });

    use_future(move || {
        let gateway = gateway.clone();
        async move {
            if *sufficient_balance.read() {
                match try_start_mining(gateway, miner, &mut toolbar_state).await {
                    Ok(()) => {
                        toolbar_state.set_status(MinerStatus::Active);
                    }
                    Err(err) => {
                        log::error!("Failed to start mining: {:?}", err);
                        toolbar_state.set_status(MinerStatus::Error);
                        toolbar_state.set_status_message(MinerStatusMessage::Error);
                    }
                }
            }
        }
    });

    if sol_balance.read().is_some() && !*sufficient_balance.read() {
        return rsx! {
            MinerToolbarInsufficientFunds {}
        };
    }

    rsx! {
        if toolbar_state.is_open() {
            div {
                class: "flex flex-col grow gap-2 px-4 py-6 sm:px-8 sm:py-8",
                h2 {
                    class: "text-3xl md:text-4xl lg:text-5xl font-bold",
                    "Starting"
                }
                if let MinerStatusMessage::GeneratingChallenge = toolbar_state.status_message() {
                    div {
                        class: "flex flex-row gap-2",
                        p {
                            class: "text-lg",
                            "Generating challenge..."
                        }
                        Spinner {
                            class: "my-auto text-white"
                        }
                    }
                }
            }
        } else {
            div {
                class: "flex flex-row w-full justify-between my-auto px-4 sm:px-8",
                p {
                    class: "font-semibold my-auto",
                    "Starting"
                }
            }
        }
    }
}
