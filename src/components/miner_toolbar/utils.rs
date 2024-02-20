use dioxus::hooks::UseSharedState;
use solana_client_wasm::solana_sdk::native_token::LAMPORTS_PER_SOL;
use web_sys::Worker;

use crate::gateway::{create_ore_token_account, initialize_proof_account, mine};

use super::{IsToolbarOpen, MinerStatus};

pub async fn try_start_mining(balance: u64, worker: &Worker) -> bool {
    if balance.eq(&0) {
        return false;
    }

    // Mark miner as inactive, if insufficient balance
    const MIN_BALANCE: u64 = LAMPORTS_PER_SOL.saturating_div(100);
    if balance.ge(&0) && balance.lt(&MIN_BALANCE) {
        return false;
    }

    // Create token acc<'a>o<'a>unt, <'a, WorkerProps<'a>>if ne<'a, WorkerProps<'a>>eded
    create_ore_token_account().await;

    // Create proof account, if needed
    initialize_proof_account().await;

    // Start mining
    mine(worker.clone()).await;

    true
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
