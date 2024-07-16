use dioxus::prelude::*;
use ore_relayer_api::state::Escrow;
use solana_client_wasm::solana_sdk::native_token::LAMPORTS_PER_SOL;

use crate::{
    components::{try_start_mining, Spinner},
    hooks::{
        use_escrow, use_miner_toolbar_state, MinerStatus, MinerStatusMessage,
        ReadMinerToolbarState, UpdateMinerToolbarState,
    },
    miner::Miner,
};

use super::MinerToolbarInsufficientFunds;

const MIN_BALANCE: u64 = LAMPORTS_PER_SOL.saturating_div(100);

#[component]
pub fn MinerToolbarActivating(miner: Signal<Miner>) -> Element {
    let mut toolbar_state = use_miner_toolbar_state();
    let escrow = use_escrow();

    // Start mining if the escrow account exists
    let _ = use_resource(move || async move {
        if escrow.read().ne(&Escrow::default()) {
            match try_start_mining(miner, escrow, &mut toolbar_state).await {
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

    if escrow.read().eq(&Escrow::default()) {
        return rsx! {
            MinerToolbarInsufficientFunds {
                miner: miner.clone()
            }
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
                div {
                    class: "flex flex-row gap-2",
                    p {
                        class: "text-lg",
                        "Preparing to mine..."
                    }
                    Spinner {
                        class: "my-auto text-white"
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
