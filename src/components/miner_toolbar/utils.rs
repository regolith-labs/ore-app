use std::rc::Rc;

use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::signer::Signer;

use crate::{
    gateway::{signer, Gateway, GatewayResult},
    hooks::{MinerStatusMessage, MinerToolbarState, UpdateMinerToolbarState},
    miner::Miner,
};

// TODO Move this somewhere

pub async fn try_start_mining(
    gateway: Rc<Gateway>,
    miner: Signal<Miner>,
    toolbar_state: &mut Signal<MinerToolbarState>,
) -> GatewayResult<()> {
    // Create proof account, if needed

    toolbar_state.set_status_message(MinerStatusMessage::GeneratingChallenge);
    loop {
        if gateway.register_ore().await.is_ok() {
            break;
        }
    }

    // Start mining
    let signer = signer();
    let proof = gateway.get_proof(signer.pubkey()).await.unwrap();
    let clock = gateway.get_clock().await.unwrap();
    let mut cutoff_time = proof
        .last_hash_at
        .saturating_add(60)
        .saturating_sub(clock.unix_timestamp)
        .max(0) as u64;
    if cutoff_time.eq(&0) {
        cutoff_time = 60;
    }

    toolbar_state.set_status_message(MinerStatusMessage::Searching);
    miner
        .read()
        .start_mining(proof.challenge.into(), cutoff_time)
        .await;

    Ok(())
}
