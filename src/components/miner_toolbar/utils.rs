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
) -> GatewayResult<bool> {
    // Create token account, if needed
    let signer = signer();
    *status_message.write() = MinerStatusMessage::CreatingTokenAccount;
    gateway.create_token_account_ore(signer.pubkey()).await?;

    // Create proof account, if needed
    *status_message.write() = MinerStatusMessage::GeneratingChallenge;
    gateway.register_ore().await?;

    // Start mining
    let treasury = gateway.get_treasury().await.unwrap();
    let proof = gateway.get_proof(signer.pubkey()).await.unwrap();
    *status_message.write() = MinerStatusMessage::Searching;
    miner
        .start_mining(
            proof.hash.into(),
            treasury.difficulty.into(),
            signer.pubkey(),
        )
        .await;

    // Record event for data
    track(AppEvent::StartMiner, None);

    Ok(true)
}
