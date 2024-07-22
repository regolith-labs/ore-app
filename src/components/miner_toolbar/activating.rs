use dioxus::prelude::*;
use ore_relayer_api::state::Escrow;
use solana_client_wasm::solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::native_token::sol_to_lamports;

use crate::{
    components::{try_start_mining, MinerToolbarTopUp, Spinner},
    hooks::{
        use_escrow, use_escrow_sol_balance, use_miner_toolbar_state, MinerStatus,
        MinerStatusMessage, ReadMinerToolbarState, UpdateMinerToolbarState,
    },
    miner::Miner,
};

use super::MinerToolbarCreateAccount;

const RENT_MIN_BALANCE: u64 = 1392000;
const MIN_BALANCE: u64 = LAMPORTS_PER_SOL.saturating_div(1000) + RENT_MIN_BALANCE;

#[component]
pub fn MinerToolbarActivating(miner: Signal<Miner>) -> Element {
    let mut toolbar_state = use_miner_toolbar_state();
    let escrow = use_escrow();
    let mut sol_balance = use_escrow_sol_balance();

    // Start mining if the escrow account exists
    let _ = use_resource(move || async move {
        if escrow.read().ne(&Escrow::default()) {
            if let Some(Ok(balance)) = *sol_balance.read() {
                if balance.ge(&MIN_BALANCE) {
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
            }
        }
    });

    if escrow.read().eq(&Escrow::default()) {
        return rsx! {
            MinerToolbarCreateAccount {
                miner: miner.clone()
            }
        };
    }

    if let Some(Ok(balance)) = *sol_balance.read() {
        if balance.lt(&MIN_BALANCE) {
            return rsx! {
                MinerToolbarTopUp {
                    sol_balance: sol_balance.clone()
                }
            };
        }
    }

    rsx! {
        div {
            class: "flex flex-row w-full justify-between my-auto px-4 sm:px-8",
            p {
                class: "font-semibold my-auto",
                "Starting..."
            }
            div {
                class: "flex w-10 h-10",
                Spinner {
                    class: "m-auto text-white"
                }
            }
        }
    }
}
