use std::rc::Rc;

use dioxus::prelude::UseSharedState;
#[cfg(feature = "web")]
use solana_client_wasm::solana_sdk::signer::Signer;
#[cfg(feature = "desktop")]
use solana_sdk::signer::Signer;

use crate::{
    gateway::{signer, Gateway, GatewayResult},
    metrics::{track, AppEvent},
    miner::Miner,
};

use super::MinerStatusMessage;

// TODO Move this somewhere

pub async fn try_start_mining(
    gateway: &Rc<Gateway>,
    miner: &Miner,
    status_message: &UseSharedState<MinerStatusMessage>,
) -> GatewayResult<()> {
    // Create proof account, if needed
    *status_message.write() = MinerStatusMessage::GeneratingChallenge;
    'register: loop {
        if gateway.register_ore().await.is_ok() {
            break 'register;
        }
    }

    // Start mining
    let signer = signer();
    let treasury = gateway.get_treasury().await.unwrap();
    let proof = gateway.get_proof(signer.pubkey()).await.unwrap();
    *status_message.write() = MinerStatusMessage::Searching;
    miner.start_mining(proof.challenge.into()).await;

    // Record event for data
    track(AppEvent::StartMiner, None);

    Ok(())
}
