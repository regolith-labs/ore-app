use dioxus::prelude::*;
use dioxus_std::utils::channel::{use_channel, UseChannel};
use solana_client_wasm::solana_sdk::blake3::Hash as Blake3Hash;

use crate::{
    components::{MinerDisplayHash, MinerStatus, MinerStatusMessage},
    miner::{submit_solution, Miner, WebWorkerResponse},
};

use super::{use_gateway, use_power_level, use_priority_fee, ProofHandle};

pub fn use_miner<'a>(
    cx: &'a ScopeState,
    display_hash: &UseSharedState<MinerDisplayHash>,
    status: &UseSharedState<MinerStatus>,
    status_message: &UseSharedState<MinerStatusMessage>,
) -> &'a UseState<Miner> {
    let gateway = use_gateway(cx);
    let power_level = use_power_level(cx);
    let proof_handle = cx.consume_context::<ProofHandle>().unwrap();
    let priority_fee = use_priority_fee(cx);

    #[cfg(feature = "web")]
    let ch = use_channel::<WebWorkerResponse>(cx, 1);
    #[cfg(feature = "web")]
    let miner = use_state(cx, || Miner::new(ch, power_level, priority_fee));

    #[cfg(feature = "desktop")]
    let miner = use_state(cx, || Miner::new(power_level, priority_fee));

    // Listen for results from miner.
    let _ = use_future(cx, (), |_| {
        let mut rx = ch.clone().receiver();
        let miner = miner.clone();
        let gateway = gateway.clone();
        let proof_handle = proof_handle.clone();
        let status = status.clone();
        let status_message = status_message.clone();
        let display_hash = display_hash.clone();
        let priority_fee = priority_fee.clone();
        async move {
            while let Ok(res) = rx.recv().await {
                println!("Received!!!");
                *display_hash.write() = MinerDisplayHash(Blake3Hash::new_from_array(res.hash));
                *status_message.write() = MinerStatusMessage::Submitting;
                let priority_fee = priority_fee.read().0;
                // match submit_solution(&gateway, &res, priority_fee).await {
                //     Ok(_sig) => {
                //         proof_handle.restart();
                //         if let MinerStatus::Active = *status.read() {
                //             *status_message.write() = MinerStatusMessage::Searching;
                //             // if let Ok(proof) = gateway.get_proof(pubkey).await {
                //             //     miner.start_mining(proof.challenge.into()).await;
                //             // }
                //         }
                //     }
                //     Err(err) => {
                //         *status_message.write() = MinerStatusMessage::Error;
                //         log::error!("Failed to submit hash: {:?}", err);
                //     }
                // }
            }
        }
    });

    miner
}
