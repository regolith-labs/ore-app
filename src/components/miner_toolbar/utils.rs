use std::rc::Rc;

use dioxus::hooks::UseSharedState;
use solana_client_wasm::solana_sdk::native_token::LAMPORTS_PER_SOL;
use web_sys::Worker;

use crate::gateway::{mine, Gateway, GatewayResult};

use super::{IsToolbarOpen, MinerStatus};

pub async fn try_start_mining(
    gateway: &Rc<Gateway>,
    balance: u64,
    worker: &Worker,
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
    mine(gateway, worker.clone()).await?;

    Ok(true)
}

// TODO Handle webworker shutdown and restart gracefully
pub fn stop_mining(
    status: &UseSharedState<MinerStatus>,
    is_toolbar_open: &UseSharedState<IsToolbarOpen>,
    // worker: Arc<Worker>,
) {
    // worker.terminate();
    *status.write() = MinerStatus::NotStarted;
    *is_toolbar_open.write() = IsToolbarOpen(false);
}
