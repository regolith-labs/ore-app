use std::rc::Rc;

use dioxus::hooks::UseSharedState;
#[cfg(feature = "web")]
use solana_client_wasm::solana_sdk::{native_token::LAMPORTS_PER_SOL, signer::Signer};
#[cfg(feature = "desktop")]
use solana_sdk::{native_token::LAMPORTS_PER_SOL, signer::Signer};

use crate::{
    gateway::{signer, Gateway, GatewayResult},
    miner::Miner,
};

use super::{IsToolbarOpen, MinerStatus};

pub async fn try_start_mining(
    gateway: &Rc<Gateway>,
    balance: u64,
    miner: &Miner,
) -> GatewayResult<bool> {
    if balance.eq(&0) {
        return Ok(false);
    }

    // Mark miner as inactive, if insufficient balance
    const MIN_BALANCE: u64 = LAMPORTS_PER_SOL.saturating_div(100);
    if balance.ge(&0) && balance.lt(&MIN_BALANCE) {
        return Ok(false);
    }

    // Create token account, if needed
    gateway.create_token_account_ore().await?;

    // Create proof account, if needed
    gateway.register_ore().await?;

    // Start mining
    let signer = signer();
    let treasury = gateway.get_treasury().await.unwrap();
    let proof = gateway.get_proof(signer.pubkey()).await.unwrap();
    miner.start_mining(
        proof.hash.into(),
        treasury.difficulty.into(),
        signer.pubkey(),
    );

    Ok(true)
}

pub fn stop_mining(
    status: &UseSharedState<MinerStatus>,
    is_toolbar_open: &UseSharedState<IsToolbarOpen>,
    // #[cfg(feature = "web")] worker: &UseState<Worker>,
    // #[cfg(feature = "web")] message: &UseRef<Option<WebworkerResponse>>,
) {
    // TODO Pause
    // #[cfg(feature = "web")]
    // worker.reset(message);
    *status.write() = MinerStatus::NotStarted;
    *is_toolbar_open.write() = IsToolbarOpen(false);
}
