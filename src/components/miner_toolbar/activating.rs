use dioxus::prelude::*;
use ore_relayer_api::state::Escrow;
use solana_client_wasm::solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::native_token::sol_to_lamports;

use crate::{
    components::{try_start_mining, Spinner},
    hooks::{
        use_escrow, use_escrow_sol_balance, use_miner_toolbar_state, MinerStatus,
        MinerStatusMessage, ReadMinerToolbarState, UpdateMinerToolbarState,
    },
    miner::Miner,
    route::Route,
};

pub const RENT_MIN_BALANCE: u64 = 1392000;
pub const MIN_BALANCE: u64 = LAMPORTS_PER_SOL.saturating_div(1000) + RENT_MIN_BALANCE;

#[component]
pub fn MinerToolbarActivating(miner: Signal<Miner>) -> Element {
    let mut toolbar_state = use_miner_toolbar_state();
    let mut escrow_balance = use_escrow_sol_balance();
    let escrow = use_escrow();
    let nav = use_navigator();

    // Start mining if the escrow account exists
    let _ = use_resource(move || async move {
        if escrow.read().ne(&Escrow::default()) {
            // TODO This is currently comment out because users are manually paying to submit hashes
            // if let Some(Ok(balance)) = *escrow_balance.read() {
            //     if balance.ge(&MIN_BALANCE) {
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
            // }
            // }
        }
    });

    if escrow.read().eq(&Escrow::default()) {
        nav.push(Route::Mine {});
    }

    if let Some(Ok(balance)) = *escrow_balance.read() {
        if balance.lt(&MIN_BALANCE) {
            nav.push(Route::Mine {});
        }
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
