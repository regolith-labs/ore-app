use dioxus::prelude::*;
use dioxus_std::utils::channel::{use_channel, UseChannel};
use solana_client_wasm::solana_sdk::blake3::Hash as Blake3Hash;

use super::{
    use_gateway, use_miner_toolbar_state, use_power_level, use_priority_fee, MinerStatus,
    MinerStatusMessage, ProofHandle,
};
use crate::miner::{submit_solution, Miner, WebWorkerResponse};

pub fn use_miner() -> Signal<Miner> {
    let mut toolbar_state = use_miner_toolbar_state();
    let gateway = use_gateway();
    let power_level = use_power_level();
    let proof_handle = use_context::<ProofHandle>();
    let priority_fee = use_priority_fee();

    let ch = use_channel::<WebWorkerResponse>(1);
    let mut rx = ch.clone().receiver();
    let miner = use_signal(|| Miner::new(ch, power_level, priority_fee));

    // Listen for results from miner.
    use_future(move || {
        let mut rx = rx.clone();
        async move {
            while let Ok(res) = rx.recv().await {
                println!("Received!!!");
                // display_hash.set(MinerDisplayHash(Blake3Hash::new_from_array(res.hash)));
                // status_message.set(MinerStatusMessage::Submitting);
                // let priority_fee = priority_fee.read().0;
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
