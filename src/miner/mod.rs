mod web_worker;

use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_sdk::utils::channel::UseChannel;
use drillx::Solution;
use lazy_static::lazy_static;
use ore_relayer_api::consts::ESCROW;
use rand::Rng;
use serde_wasm_bindgen::to_value;
use solana_client_wasm::solana_sdk::{pubkey::Pubkey, signature::Signature};
use solana_sdk::blake3::Hash as Blake3Hash;
use web_sys::{window, Worker};
pub use web_worker::*;

use crate::{
    gateway::{Gateway, GatewayResult},
    hooks::{
        use_gateway, MinerStatus, MinerStatusMessage, MinerToolbarState, PowerLevel, PriorityFee,
        ReadMinerToolbarState, UpdateMinerToolbarState,
    },
};

// Number of physical cores on machine
lazy_static! {
    pub static ref WEB_WORKERS: usize = fetch_logical_processors();
}

fn fetch_logical_processors() -> usize {
    let window = window().expect("should have a window");
    let navigator = window.navigator();
    navigator.hardware_concurrency() as usize
}

/// Miner encapsulates the logic needed to efficiently mine for valid hashes according to the application runtime and hardware.
pub struct Miner {
    power_level: Signal<PowerLevel>,
    priority_fee: Signal<PriorityFee>,
    web_workers: Vec<Worker>,
}

impl Miner {
    pub fn new(
        cx: UseChannel<WebWorkerResponse>,
        power_level: Signal<PowerLevel>,
        priority_fee: Signal<PriorityFee>,
    ) -> Self {
        Self {
            power_level: power_level.clone(),
            priority_fee: priority_fee.clone(),
            web_workers: (0..*WEB_WORKERS)
                .map(|_| create_web_worker(cx.clone()))
                .collect(),
        }
    }

    pub async fn start_mining(&self, challenge: [u8; 32], offset: u64, cutoff_time: u64) {
        self.start_mining_web(challenge, offset, cutoff_time).await;
    }

    pub async fn start_mining_web(&self, challenge: [u8; 32], offset: u64, cutoff_time: u64) {
        let nonce = u64::MAX.saturating_div(self.web_workers.len() as u64);
        let power_level = self.power_level.read().0.saturating_sub(1) as usize;
        log::info!("Start mining web...");
        for (i, web_worker) in self.web_workers.iter().enumerate() {
            let nonce = nonce.saturating_mul(i as u64).saturating_add(offset);
            if i.le(&power_level) {
                log::info!("Posting message: {:?} {:?}", i, nonce);
                web_worker
                    .post_message(
                        &to_value(
                            &(WebWorkerRequest {
                                challenge,
                                nonce: nonce.to_le_bytes(),
                                offset,
                                cutoff_time,
                            }),
                        )
                        .unwrap(),
                    )
                    .unwrap();
            }
        }
    }

    pub async fn process_web_worker_results(
        &self,
        messages: &Vec<WebWorkerResponse>,
        toolbar_state: &mut Signal<MinerToolbarState>,
        gateway: Rc<Gateway>,
    ) {
        log::info!("Batch: {:?}", messages);

        // Get best solution
        let mut challenge = [0; 32];
        let mut offset = 0;
        let mut best_difficulty = 0;
        let mut best_solution = Solution::new([0; 16], [0; 8]);
        let mut best_hash = [0u8; 32];
        for msg in messages {
            if msg.difficulty.gt(&best_difficulty) {
                best_solution = drillx::Solution::new(msg.digest, msg.nonce);
                best_difficulty = msg.difficulty;
                best_hash = msg.hash;
                offset = msg.offset;
                challenge = msg.challenge;
            }
        }

        // Kickoff new batch
        if best_difficulty.lt(&ore_api::consts::MIN_DIFFICULTY) {
            self.start_mining(challenge, offset, 0).await;
            return;
        }

        // Update toolbar state
        toolbar_state.set_display_hash(Blake3Hash::new_from_array(best_hash));
        toolbar_state.set_status_message(MinerStatusMessage::Submitting);
        let priority_fee = self.priority_fee.read().0;

        // Submit solution
        let authority = toolbar_state.escrow().authority;
        let escrow_pubkey =
            Pubkey::find_program_address(&[ESCROW, authority.as_ref()], &ore_relayer_api::id()).0;
        match submit_solution(authority, best_solution, priority_fee).await {
            // Start mining again
            Ok(_sig) => {
                if let MinerStatus::Active = toolbar_state.status() {
                    if let Ok(proof) = gateway.get_proof(escrow_pubkey).await {
                        if let Ok(clock) = gateway.get_clock().await {
                            toolbar_state.set_status_message(MinerStatusMessage::Searching);
                            let cutoff_time = proof
                                .last_hash_at
                                .saturating_add(60)
                                .saturating_sub(clock.unix_timestamp)
                                .max(0) as u64;
                            self.start_mining(proof.challenge.into(), 0, cutoff_time)
                                .await;
                        } else {
                            log::error!("Failed to get clock");
                        }
                    } else {
                        log::error!("Failed to get proof");
                    }
                }
            }

            // Display error
            Err(err) => {
                toolbar_state.set_status_message(MinerStatusMessage::Error);
                log::error!("Failed to submit hash: {:?}", err);
            }
        }
    }
}

pub async fn submit_solution(
    pubkey: Pubkey,
    solution: Solution,
    priority_fee: u64,
) -> GatewayResult<Signature> {
    let gateway = use_gateway();
    gateway.send_via_relayer(pubkey, solution).await
}
