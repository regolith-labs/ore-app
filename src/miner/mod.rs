mod web_worker;

pub use web_worker::*;

use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_std::utils::channel::UseChannel;
use drillx::Solution;
use ore::{BUS_COUNT, EPOCH_DURATION};
use rand::Rng;
use serde_wasm_bindgen::to_value;
use solana_client_wasm::solana_sdk::{
    blake3::Hash as Blake3Hash, compute_budget::ComputeBudgetInstruction, pubkey::Pubkey,
    signature::Signature, signer::Signer,
};
use web_sys::Worker;

use crate::{
    gateway::{signer, Gateway, GatewayResult, CU_LIMIT_MINE},
    hooks::{
        MinerStatus, MinerStatusMessage, MinerToolbarState, PowerLevel, PriorityFee, ProofHandle,
        ReadMinerToolbarState, UpdateMinerToolbarState,
    },
};

pub const WEB_WORKERS: usize = 8;

/// Miner encapsulates the logic needed to efficiently mine for valid hashes according to the application runtime and hardware.
pub struct Miner {
    power_level: Signal<PowerLevel>,
    priority_fee: Signal<PriorityFee>,
    web_worker: [Worker; WEB_WORKERS],
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
            web_worker: std::array::from_fn(|_| create_web_worker(cx.clone())),
        }
    }

    pub async fn start_mining(&self, challenge: [u8; 32], offset: u64, cutoff_time: u64) {
        self.start_mining_web(challenge, offset, cutoff_time).await;
    }

    pub async fn start_mining_web(&self, challenge: [u8; 32], offset: u64, cutoff_time: u64) {
        let nonce = u64::MAX.saturating_div(self.web_worker.len() as u64);
        for (i, web_worker) in self.web_worker.iter().enumerate() {
            let nonce = nonce.saturating_mul(i as u64).saturating_add(offset);
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

    pub async fn process_web_worker_results(
        &self,
        messages: &Vec<WebWorkerResponse>,
        toolbar_state: &mut Signal<MinerToolbarState>,
        priority_fee: Signal<PriorityFee>,
        proof_handle: &mut ProofHandle,
        gateway: Rc<Gateway>,
        pubkey: Pubkey,
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
        if best_difficulty.lt(&ore::MIN_DIFFICULTY) {
            log::info!("New batch {:?}", offset);
            self.start_mining(challenge, offset, 0).await;
            return;
        }

        // Update toolbar state
        toolbar_state.set_display_hash(Blake3Hash::new_from_array(best_hash));
        toolbar_state.set_status_message(MinerStatusMessage::Submitting);
        let priority_fee = priority_fee.read().0;

        // Submit solution
        match submit_solution(&gateway, best_solution, priority_fee).await {
            // Start mining again
            Ok(sig) => {
                log::info!("Success: {}", sig);
                proof_handle.restart();
                if let MinerStatus::Active = toolbar_state.status() {
                    toolbar_state.set_status_message(MinerStatusMessage::Searching);
                    if let Ok(proof) = gateway.get_proof(pubkey).await {
                        if let Ok(clock) = gateway.get_clock().await {
                            let mut cutoff_time = proof
                                .last_hash_at
                                .saturating_add(60)
                                .saturating_sub(clock.unix_timestamp)
                                .max(0) as u64;
                            self.start_mining(proof.challenge.into(), 0, cutoff_time)
                                .await;
                        }
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
    gateway: &Rc<Gateway>,
    solution: Solution,
    priority_fee: u64,
) -> GatewayResult<Signature> {
    let signer = signer();
    // Build ixs
    let cu_limit_ix = ComputeBudgetInstruction::set_compute_unit_limit(CU_LIMIT_MINE);
    let cu_price_ix = ComputeBudgetInstruction::set_compute_unit_price(priority_fee);
    let mut ixs = vec![cu_limit_ix, cu_price_ix];

    // Reset if needed
    if needs_reset(gateway).await {
        ixs.push(ore::instruction::reset(signer.pubkey()));
    }

    // Build mine tx
    let bus_id = pick_bus();
    let ix = ore::instruction::mine(signer.pubkey(), ore::BUS_ADDRESSES[bus_id], solution);
    ixs.push(ix);

    // Send and configm
    gateway.send_and_confirm(&ixs, false, false).await
}

async fn needs_reset(gateway: &Rc<Gateway>) -> bool {
    if let Ok(clock) = gateway.get_clock().await {
        if let Ok(config) = gateway.get_config().await {
            return config
                .last_reset_at
                .saturating_add(EPOCH_DURATION)
                .saturating_sub(5) // Buffer
                .le(&clock.unix_timestamp);
        }
    }
    false
}

fn pick_bus() -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..BUS_COUNT)
}
