// mod activating;
// mod active;
// mod error;
// mod insufficient_sol;
mod layout;
// mod not_started;
mod utils;

// pub use activating::*;
// pub use active::*;
// pub use error::*;
// pub use insufficient_sol::*;
pub use layout::*;
// pub use not_started::*;
pub use utils::*;

use dioxus::prelude::*;
use dioxus_std::utils::channel::use_channel;
use solana_client_wasm::solana_sdk::blake3::Hash as Blake3Hash;

use crate::hooks::{
    use_gateway, use_miner, use_miner_toolbar_state, use_miner_toolbar_state_provider,
    use_priority_fee, use_pubkey, use_treasury, MinerStatus, MinerStatusMessage, ProofHandle,
    ReadMinerToolbarState, UpdateMinerToolbarState,
};

#[component]
pub fn MinerToolbar(hidden: bool) -> Element {
    // use_context_provider(|| Signal::new(MinerStatus::NotStarted));
    // use_context_provider(|| Signal::new(MinerStatusMessage::Searching));
    // use_context_provider(|| Signal::new(MinerDisplayHash(Blake3Hash::new_unique())));
    // let mut miner_status = use_context::<Signal<MinerStatus>>();
    // let mut miner_status_message = use_context::<Signal<MinerStatusMessage>>();
    // let mut miner_display_hash = use_context::<Signal<MinerDisplayHash>>();
    // let mut is_toolbar_open = use_context::<Signal<IsToolbarOpen>>();
    let mut toolbar_state = use_miner_toolbar_state();
    let gateway = use_gateway();
    let pubkey = use_pubkey();
    let treasury = use_treasury();
    let miner = use_miner();

    // Animate the hash in the miner toolbar to visualize mining
    use_future(move || async move {
        loop {
            async_std::task::sleep(std::time::Duration::from_millis(75)).await;
            if let MinerStatusMessage::Searching = toolbar_state.status_message() {
                toolbar_state.set_display_hash(Blake3Hash::new_unique());
            } else {
                break;
            }
        }
    });

    let is_open = toolbar_state.read().is_open;
    let class =
        "fixed transition-height transition-colors flex flex-row justify-between inset-x-0 bottom-0 drop-shadow-md";
    let height = if is_open {
        "max-h-[80vh] shrink overflow-y-scroll"
    } else {
        "h-16 cursor-pointer"
    };

    let bg = match toolbar_state.read().status {
        MinerStatus::Active => "bg-green-500 text-white",
        MinerStatus::Error => "bg-red-500 text-white",
        MinerStatus::NotStarted => {
            if is_open {
                "bg-white dark:bg-gray-900"
            } else {
                "bg-gray-100 dark:bg-gray-900"
            }
        }
        _ => "bg-gray-100 dark:bg-gray-900",
    };

    let display = if hidden { "hidden" } else { "" };

    rsx! {
        div {
            class: "{class} {height} {bg} {display}",
            onclick: move |_e| {
                toolbar_state.set_is_open(true);
            },
            div {
                class: "flex flex-row justify-between w-full max-w-[96rem] mx-auto h-full",
                // match *miner_status.read() {
                    // MinerStatus::NotStarted => {
                    //     rsx! {
                    //         MinerToolbarNotStarted {}
                    //     }
                    // }
                    // MinerStatus::Activating => {
                    //     rsx! {
                    //         MinerToolbarActivating {
                    //             miner: miner.clone()
                    //         }
                    //     }
                    // }
                    // MinerStatus::Active => {
                    //     rsx! {
                    //         MinerToolbarActive {
                    //             miner: miner.clone()
                    //         }
                    //     }
                    // }
                    // MinerStatus::Error => {
                    //     rsx! {
                    //         MinerToolbarError {}
                    //     }
                    // }
                    // _ => None
                // }
            }
        }
    }
}
