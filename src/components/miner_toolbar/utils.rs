use std::rc::Rc;

use dioxus::{
    hooks::UseSharedState,
    prelude::{UseRef, UseState},
};
#[cfg(feature = "web")]
use solana_client_wasm::solana_sdk::native_token::LAMPORTS_PER_SOL;
#[cfg(feature = "desktop")]
use solana_sdk::native_token::LAMPORTS_PER_SOL;
#[cfg(feature = "web")]
use web_sys::Worker;

use crate::{
    gateway::{mine, Gateway, GatewayResult, WebworkerResponse},
    hooks::ResetWorker,
};

use super::{IsToolbarOpen, MinerStatus};

pub async fn try_start_mining(
    gateway: &Rc<Gateway>,
    balance: u64,
    worker: UseState<Worker>,
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
    mine(gateway, &worker).await?;

    Ok(true)
}

pub fn stop_mining(
    status: &UseSharedState<MinerStatus>,
    is_toolbar_open: &UseSharedState<IsToolbarOpen>,
    worker: &UseState<Worker>,
    message: &UseRef<Option<WebworkerResponse>>,
) {
    worker.reset(message);
    *status.write() = MinerStatus::NotStarted;
    *is_toolbar_open.write() = IsToolbarOpen(false);
}
