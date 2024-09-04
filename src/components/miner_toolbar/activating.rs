use dioxus::prelude::*;

use crate::{
    components::{try_start_mining, Spinner},
    hooks::{
        use_miner_toolbar_state, use_proof, MinerStatus, MinerStatusMessage,
        UpdateMinerToolbarState,
    },
    miner::Miner,
    route::Route,
};

#[component]
pub fn MinerToolbarActivating(miner: Signal<Miner>) -> Element {
    let mut toolbar_state = use_miner_toolbar_state();
    let proof = use_proof();
    let nav = use_navigator();

    // Start mining if the escrow account exists
    let _ = use_resource(move || async move {
        if let Some(Ok(proof)) = *proof.read() {
            match try_start_mining(miner, proof, &mut toolbar_state).await {
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
    });

    if proof.read().is_none() {
        nav.push(Route::Mine {});
    }

    rsx! {
        div {
            class: "flex flex-row w-full justify-end my-auto px-4 sm:px-8",
            div {
                class: "flex w-10 h-10",
                Spinner {
                    class: "m-auto text-white"
                }
            }
        }
    }
}
