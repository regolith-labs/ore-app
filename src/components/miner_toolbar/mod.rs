mod activating;
mod active;
mod error;
mod insufficient_sol;
mod layout;
mod not_started;
mod utils;

pub use activating::*;
pub use active::*;
pub use error::*;
pub use insufficient_sol::*;
pub use layout::*;
pub use not_started::*;
use ore_relayer_api::state::Escrow;
pub use utils::*;

use dioxus::prelude::*;

use crate::{
    hooks::{
        use_escrow, use_gateway, use_miner, use_miner_toolbar_state,
        use_wallet_adapter::{use_wallet_adapter, WalletAdapter},
        MinerStatus, ReadMinerToolbarState,
    },
    route::Route,
};

#[component]
pub fn MinerToolbar(hidden: bool) -> Element {
    let mut toolbar_state = use_miner_toolbar_state();
    let wallet_adapter = use_wallet_adapter();
    let miner = use_miner();
    let gateway = use_gateway();
    let mut escrow = use_escrow();
    let nav = use_navigator();

    let _ = use_resource(move || {
        let gateway = gateway.clone();
        async move {
            match *wallet_adapter.read() {
                WalletAdapter::Disconnected => {
                    escrow.set(Escrow::default());
                }
                WalletAdapter::Connected(pubkey) => {
                    if let Ok(e) = gateway.get_escrow(pubkey).await {
                        escrow.set(e);
                    } else {
                        escrow.set(Escrow::default());
                    }
                }
            }
            ()
        }
    });

    let class =
        "fixed transition-height transition-colors flex flex-row justify-between inset-x-0 bottom-0 drop-shadow-md";
    let height = "h-16 cursor-pointer";

    let bg = match toolbar_state.status() {
        MinerStatus::Active => "bg-green-500 text-white",
        MinerStatus::Error => "bg-red-500 text-white",
        MinerStatus::NotStarted => "bg-gray-100 dark:bg-gray-900",
        _ => "bg-gray-100 dark:bg-gray-900",
    };

    let display = if hidden { "hidden" } else { "" };

    if let WalletAdapter::Disconnected = *wallet_adapter.read() {
        return rsx! {};
    }

    rsx! {
        div {
            class: "{class} {height} {bg} {display}",
            onclick: move |e| {
                nav.push(Route::Mine {});
                e.stop_propagation();
            },
            div {
                class: "flex flex-row justify-between w-full max-w-[96rem] mx-auto h-full",
                match toolbar_state.status() {
                    MinerStatus::NotStarted => {
                        rsx! {
                            MinerToolbarNotStarted {}
                        }
                    }
                    MinerStatus::Activating => {
                        rsx! {
                            MinerToolbarActivating { miner }
                        }
                    }
                    MinerStatus::Active => {
                        rsx! {
                            MinerToolbarActive { miner }
                        }
                    }
                    MinerStatus::Error => {
                        rsx! {
                            MinerToolbarError {}
                        }
                    }
                }
            }
        }
    }
}
