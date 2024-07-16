use dioxus::prelude::*;
use ore_relayer_api::{consts::ESCROW, state::Escrow};
use solana_client_wasm::solana_sdk::{native_token::LAMPORTS_PER_SOL, pubkey::Pubkey};

use crate::{
    components::{try_start_mining, wallet_adapter, Spinner},
    gateway,
    hooks::{
        use_gateway, use_miner_toolbar_state, use_sol_balance,
        use_wallet_adapter::{use_wallet_adapter, WalletAdapter},
        MinerStatus, MinerStatusMessage, ReadMinerToolbarState, UpdateMinerToolbarState,
    },
    miner::Miner,
};

use super::MinerToolbarInsufficientFunds;

const MIN_BALANCE: u64 = LAMPORTS_PER_SOL.saturating_div(100);

#[component]
pub fn MinerToolbarActivating(miner: Signal<Miner>) -> Element {
    let gateway = use_gateway();
    let mut toolbar_state = use_miner_toolbar_state();

    // TODO Start mining if the escrow account exists
    use_resource(move || async move {
        if toolbar_state.escrow().ne(&Escrow::default()) {
            match try_start_mining(miner, &mut toolbar_state).await {
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

    if toolbar_state.escrow().eq(&Escrow::default()) {
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
