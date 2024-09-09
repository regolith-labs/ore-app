mod web_worker;

use base64::Engine;
use dioxus::prelude::*;
use dioxus_sdk::utils::channel::UseChannel;
use drillx::Solution;
use lazy_static::lazy_static;
use ore_api::state::Proof;
use rand::Rng;
use serde_wasm_bindgen::to_value;
use solana_client_wasm::solana_sdk::{
    blake3::Hash as Blake3Hash, compute_budget::ComputeBudgetInstruction, pubkey::Pubkey,
    signature::Signature, transaction::Transaction,
};
use web_sys::{window, Worker};
use web_time::{Duration, Instant};
pub use web_worker::*;

use crate::{
    gateway::{self, proof_pubkey, GatewayError, GatewayResult},
    hooks::{
        use_gateway, use_wallet_adapter::WalletAdapter, MinerStatus, MinerStatusMessage,
        MinerToolbarState, PowerLevel, ReadMinerToolbarState, UpdateMinerToolbarState,
    },
    metrics::{self, AppEvent},
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
    web_workers: Vec<Worker>,
}

impl Miner {
    pub fn new(cx: UseChannel<WebWorkerResponse>, power_level: Signal<PowerLevel>) -> Self {
        Self {
            power_level: power_level.clone(),
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
                                power_level,
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
        proof: &mut Resource<GatewayResult<Proof>>,
        wallet_adapter: Signal<WalletAdapter>,
    ) {
        log::info!("Batch: {:?}", messages);
        // Exit early if not active
        match toolbar_state.status() {
            MinerStatus::Active => {}
            _ => return,
        }

        // Get the pubkey
        let WalletAdapter::Connected(authority) = *wallet_adapter.read() else {
            return;
        };

        // Get best solution
        let gateway = use_gateway();
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
        if let Ok(config) = gateway.get_config().await {
            if best_difficulty.lt(&(config.min_difficulty as u32)) {
                self.start_mining(challenge, offset, 0).await;
                return;
            }
        }

        // Update toolbar state
        toolbar_state.set_display_hash(Blake3Hash::new_from_array(best_hash));

        // Submit solution
        match submit_solution(authority, best_solution, toolbar_state).await {
            // Start mining again
            Ok(_sig) => {
                metrics::track(AppEvent::Mine);
                if let MinerStatus::Active = toolbar_state.status() {
                    async_std::task::sleep(Duration::from_millis(2000)).await;
                    proof.restart();
                    if let Ok(proof) = gateway.get_proof_update(authority, challenge).await {
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
                toolbar_state.set_status(MinerStatus::Error);
                match err {
                    GatewayError::SignatureDenied => {
                        toolbar_state.set_status_message(MinerStatusMessage::SignatureDenied);
                    }
                    _ => {}
                }
            }
        }
    }
}

pub async fn submit_solution(
    authority: Pubkey,
    solution: Solution,
    toolbar_state: &mut Signal<MinerToolbarState>,
) -> GatewayResult<Signature> {
    // Build tx
    toolbar_state.set_status_message(MinerStatusMessage::Submitting(0));
    let gateway = use_gateway();
    let price = gateway::get_recent_priority_fee_estimate(false).await;
    let cu_limit_ix = ComputeBudgetInstruction::set_compute_unit_limit(500_000);
    let cu_price_ix = ComputeBudgetInstruction::set_compute_unit_price(price);
    let mut ixs = vec![cu_limit_ix, cu_price_ix];
    ixs.push(ore_api::instruction::auth(proof_pubkey(authority)));
    ixs.push(ore_api::instruction::mine(
        authority,
        authority,
        find_bus(),
        solution,
    ));
    let mut tx = Transaction::new_with_payer(&ixs, Some(&authority));

    // Sign and submit the tx
    loop {
        // Set recent blockhash
        if let Ok(blockhash) = gateway.get_latest_blockhash().await {
            tx.message.recent_blockhash = blockhash;
        }
        log::info!("TX: {:?}", tx);

        // Get signature from user
        let mut eval = eval(
            r#"
        let msg = await dioxus.recv();
        let signed = await window.OreTxSigner({b64: msg});
        dioxus.send(signed);
        "#,
        );
        let bytes = bincode::serialize(&tx).unwrap();
        let b64 = base64::engine::general_purpose::STANDARD.encode(bytes);
        if let Ok(_res) = eval.send(serde_json::Value::String(b64)) {
            // Parse response
            let res = eval.recv().await;
            if let Ok(serde_json::Value::String(string)) = res {
                if let Some(tx) = base64::engine::general_purpose::STANDARD
                    .decode(string)
                    .ok()
                    .and_then(|buffer| bincode::deserialize(&buffer).ok())
                {
                    // Submit the tx
                    let mut i = 1;
                    let timer = Instant::now();
                    'submit: loop {
                        toolbar_state.set_status_message(MinerStatusMessage::Submitting(i));
                        match gateway.rpc.send_transaction(&tx).await {
                            Ok(sig) => {
                                // Confirm the signature
                                log::info!("Sig: {:?}", sig);
                                let confirmed = gateway.confirm_signature(sig).await;
                                if confirmed.is_ok() {
                                    return Ok(tx.signatures[0]);
                                }

                                // Break if 1 min has passed
                                if timer.elapsed().as_secs().gt(&60) {
                                    break 'submit;
                                }
                            }
                            Err(err) => {
                                // TODO
                                log::error!("Err: {:?}", err);
                                break 'submit;
                            }
                        }
                        i += 1;
                    }
                }
            } else {
                log::error!("Signature denied A");
                return Err(GatewayError::SignatureDenied);
            }
        } else {
            log::error!("Signature denied B");
            return Err(GatewayError::SignatureDenied);
        }
    }

    // Ok(tx.signatures[0])
}

fn find_bus() -> Pubkey {
    let i = rand::thread_rng().gen_range(0..ore_api::consts::BUS_COUNT);
    ore_api::consts::BUS_ADDRESSES[i]
}
