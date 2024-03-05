mod miner_charts;
mod miner_toolbar_activating;
mod miner_toolbar_active;
mod miner_toolbar_insufficient_sol;
mod miner_toolbar_not_started;
mod utils;

use dioxus_std::utils::channel::use_channel;
pub use miner_charts::*;
pub use miner_toolbar_activating::*;
pub use miner_toolbar_active::*;
pub use miner_toolbar_insufficient_sol::*;
pub use miner_toolbar_not_started::*;
use ore::TREASURY_ADDRESS;
#[cfg(feature = "web")]
use solana_client_wasm::solana_sdk::keccak::Hash as KeccakHash;
#[cfg(feature = "desktop")]
use solana_sdk::keccak::Hash as KeccakHash;

pub use utils::*;

use dioxus::prelude::*;

use crate::{
    gateway::AsyncResult,
    hooks::{
        use_account_subscribe, use_gateway, use_miner, use_ore_supply, use_pubkey, use_treasury,
    },
    miner::{submit_solution, MiningResult},
    ProofHandle,
};

#[derive(Debug)]
pub enum MinerStatus {
    NotStarted,
    Activating,
    Active,

    // TODO Add error field
    NetworkError,
}

// TODO A message bar
#[derive(Debug)]
pub struct MinerStatusMessage(pub String);

#[derive(Debug)]
pub struct MinerDisplayHash(pub KeccakHash);

pub struct MinerDisplayHashIsGrinding(pub bool);

pub struct IsToolbarOpen(pub bool);

#[component]
pub fn MinerToolbar(cx: Scope<MinerToolbarProps>, hidden: bool) -> Element {
    use_shared_state_provider(cx, || MinerStatus::NotStarted);
    use_shared_state_provider(cx, || MinerStatusMessage(String::new()));
    use_shared_state_provider(cx, || MinerDisplayHash(KeccakHash::new_unique()));
    use_shared_state_provider(cx, || MinerDisplayHashIsGrinding(false));
    let miner_status = use_shared_state::<MinerStatus>(cx).unwrap();
    let miner_status_message = use_shared_state::<MinerStatusMessage>(cx).unwrap();
    let miner_display_hash = use_shared_state::<MinerDisplayHash>(cx).unwrap();
    let miner_display_hash_is_grinding =
        use_shared_state::<MinerDisplayHashIsGrinding>(cx).unwrap();
    let is_toolbar_open = use_shared_state::<IsToolbarOpen>(cx).unwrap();
    let gateway = use_gateway(cx);
    let (treasury_rw, _) = use_treasury(cx);
    let treasury = *treasury_rw.read().unwrap();
    let proof_ = cx.consume_context::<ProofHandle>().unwrap();
    let (ore_supply, refresh_ore_supply) = use_ore_supply(cx);
    let ch = use_channel::<MiningResult>(cx, 1);
    let miner = use_miner(cx, ch);
    let pubkey = use_pubkey(cx);
    let _ = use_account_subscribe(cx, TREASURY_ADDRESS, treasury_rw);

    let _ = use_future(cx, miner_display_hash_is_grinding, |_| {
        let display_hash = miner_display_hash.clone();
        let is_grinding = miner_display_hash_is_grinding.clone();
        async move {
            loop {
                async_std::task::sleep(std::time::Duration::from_millis(75)).await;
                if is_grinding.read().0 {
                    *display_hash.write() = MinerDisplayHash(KeccakHash::new_unique());
                } else {
                    break;
                }
            }
        }
    });

    // Listen for results from miner.
    // Submit for validation and start mining next hash.
    let _ = use_future(cx, (), |_| {
        let mut rx = ch.clone().receiver();
        let status = miner_status.clone();
        let miner = miner.clone();
        let gateway = gateway.clone();
        let proof_ = proof_.clone();
        let miner_status_message = miner_status_message.clone();
        let miner_display_hash = miner_display_hash.clone();
        let miner_display_hash_is_grinding = miner_display_hash_is_grinding.clone();
        async move {
            while let Ok(res) = rx.recv().await {
                *miner_display_hash.write() = MinerDisplayHash(res.hash);
                *miner_display_hash_is_grinding.write() = MinerDisplayHashIsGrinding(false);
                *miner_status_message.write() =
                    MinerStatusMessage("Submitting hash for validation...".to_string());
                match submit_solution(&gateway, &res).await {
                    Ok(_sig) => {
                        proof_.restart();
                        if let MinerStatus::Active = *status.read() {
                            if let Ok(treasury) = gateway.get_treasury().await {
                                if let Ok(proof) = gateway.get_proof(pubkey).await {
                                    miner.start_mining(
                                        proof.hash.into(),
                                        treasury.difficulty.into(),
                                        pubkey,
                                    );
                                    *miner_display_hash_is_grinding.write() =
                                        MinerDisplayHashIsGrinding(true);
                                    *miner_status_message.write() = MinerStatusMessage(
                                        "Searching for a valid hash...".to_string(),
                                    );
                                }
                            }
                        }
                    }
                    Err(err) => {
                        let msg = format!("Error validating hash: {:?}", err).to_string();
                        *miner_status_message.write() = MinerStatusMessage(msg);
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
        "fixed transition-height transition-colors flex flex-row justify-between inset-x-0 bottom-0 drop-shadow-md";
    let height = if is_open {
        "max-h-[80vh] h-min overflow-y-scroll"
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

    let display = if *hidden { "hidden" } else { "" };

    render! {
        div {
            class: "{class} {height} {bg} {display}",
            onclick: move |_e| {
                *is_toolbar_open.write() = IsToolbarOpen(true);
            },
            div {
                class: "flex flex-row justify-between w-full max-w-[96rem] mx-auto",
                match *miner_status.read() {
                    MinerStatus::NotStarted => {
                        render! {
                            MinerToolbarNotStarted {}
                        }
                    }
                    MinerStatus::Activating => {
                        render! {
                            MinerToolbarActivating {
                                miner: miner.clone()
                            }
                        }
                    }
                    MinerStatus::Active => {
                        render! {
                            MinerToolbarActive {
                                treasury: treasury,
                                ore_supply: ore_supply,
                                miner: miner.clone()
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
