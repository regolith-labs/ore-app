use std::rc::Rc;
#[cfg(feature = "desktop")]
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

use dioxus::prelude::UseSharedState;
use dioxus_std::utils::channel::UseChannel;
use ore::EPOCH_DURATION;
use serde::{Deserialize, Serialize};
#[cfg(feature = "web")]
use serde_wasm_bindgen::to_value;
#[cfg(feature = "web")]
use solana_client_wasm::solana_sdk::{
    keccak::{hashv, Hash as KeccakHash},
    pubkey::Pubkey,
    {signature::Signature, signer::Signer},
};
#[cfg(feature = "desktop")]
use solana_sdk::{
    keccak::{hashv, Hash as KeccakHash},
    pubkey::Pubkey,
    {signature::Signature, signer::Signer},
};
#[cfg(feature = "web")]
use web_sys::Worker;

#[cfg(feature = "web")]
use crate::worker::create_worker;
use crate::{
    gateway::{signer, Gateway, GatewayResult},
    hooks::PowerLevel,
};

/// Mining request for web workers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MineRequest {
    pub hash: KeccakHash,
    pub difficulty: KeccakHash,
    pub pubkey: Pubkey,
}

/// Mining response from web workers
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MiningResult {
    pub hash: KeccakHash,
    pub nonce: u64,
}

/// Miner encapsulates the logic needed to efficiently mine for valid hashes according to the application runtime and hardware.
#[derive(PartialEq)]
pub struct Miner {
    #[cfg(feature = "web")]
    worker: Worker,
    #[cfg(feature = "desktop")]
    ch: UseChannel<MiningResult>,
    power_level: UseSharedState<PowerLevel>,
}

impl Miner {
    pub fn new(ch: &UseChannel<MiningResult>, power_level: &UseSharedState<PowerLevel>) -> Self {
        Self {
            #[cfg(feature = "web")]
            worker: create_worker(ch),
            #[cfg(feature = "desktop")]
            ch: ch.clone(),
            power_level: power_level.clone(),
        }
    }

    pub fn stop(&self) {
        // TODO interrupt current work (optimization)
    }

    pub fn start_mining(&self, hash: KeccakHash, difficulty: KeccakHash, signer: Pubkey) {
        #[cfg(feature = "web")]
        {
            self.worker
                .post_message(
                    &to_value(
                        &(MineRequest {
                            hash,
                            difficulty,
                            pubkey: signer,
                        }),
                    )
                    .unwrap(),
                )
                .unwrap();
        }

        // TODO Configurable power level
        #[cfg(feature = "desktop")]
        {
            let ch = self.ch.clone();
            let flag = Arc::new(AtomicBool::new(false));
            let result = Arc::new(Mutex::new(MiningResult::default()));
            let power_percent = ((self.power_level.read().0 + 1) as f64) / 8f64;
            let concurrency = num_cpus::get() as u64;
            let tuned_concurrency = ((concurrency as f64) * power_percent).round() as u64;
            let handles: Vec<_> = (0..tuned_concurrency)
                .map(|i| {
                    std::thread::spawn({
                        let flag = flag.clone();
                        let result = result.clone();
                        move || {
                            let nonce =
                                u64::MAX.saturating_div(tuned_concurrency).saturating_mul(i);
                            if let Some(res) =
                                find_next_hash_par(hash, difficulty, signer, nonce, flag.clone())
                            {
                                flag.store(true, Ordering::Relaxed);
                                let mut w_result = result.lock().unwrap();
                                *w_result = res;
                            }
                        }
                    })
                })
                .collect();
            for h in handles {
                h.join().unwrap();
            }
            let r_result = result.lock().unwrap();
            let res = r_result.clone();
            async_std::task::spawn(async move {
                ch.send(res).await.ok();
            });
        }
    }
}

#[cfg(feature = "desktop")]
fn find_next_hash_par(
    hash: KeccakHash,
    difficulty: KeccakHash,
    signer: Pubkey,
    nonce: u64,
    flag: Arc<AtomicBool>,
) -> Option<MiningResult> {
    let mut next_hash: KeccakHash;
    let mut nonce = nonce;
    loop {
        if nonce % 10_000 == 0 {
            if flag.load(Ordering::Relaxed) {
                return None;
            }
            log::info!("Nonce: {:?}", nonce);
        }
        next_hash = hashv(&[
            hash.to_bytes().as_slice(),
            signer.to_bytes().as_slice(),
            nonce.to_be_bytes().as_slice(),
        ]);
        if next_hash.le(&difficulty) {
            break;
        }
        nonce += 1;
    }
    Some(MiningResult {
        hash: next_hash,
        nonce,
    })
}

#[cfg(feature = "web")]
pub fn find_next_hash(hash: KeccakHash, difficulty: KeccakHash, signer: Pubkey) -> MiningResult {
    let mut next_hash: KeccakHash;
    let mut nonce = 0u64;
    loop {
        if nonce % 10_000 == 0 {
            log::info!("Nonce: {:?}", nonce);
        }
        next_hash = hashv(&[
            hash.to_bytes().as_slice(),
            signer.to_bytes().as_slice(),
            nonce.to_be_bytes().as_slice(),
        ]);
        if next_hash.le(&difficulty) {
            break;
        }
        nonce += 1;
    }
    MiningResult {
        hash: next_hash,
        nonce,
    }
}

pub async fn submit_solution(
    gateway: &Rc<Gateway>,
    res: &MiningResult,
) -> GatewayResult<Signature> {
    // Submit mine tx.
    let mut bus_id = 0;
    let next_hash = res.hash;
    let nonce = res.nonce;
    let signer = signer();
    loop {
        // Check if epoch needs to be reset
        let treasury = gateway.get_treasury().await?;
        let clock = gateway.get_clock().await?;
        let epoch_end_at = treasury.epoch_start_at.saturating_add(EPOCH_DURATION);

        // Submit restart epoch tx, if needed.
        if clock.unix_timestamp.ge(&epoch_end_at) {
            let ix = ore::instruction::reset(signer.pubkey());
            gateway.send_and_confirm(&[ix]).await?;
        }

        // Submit mine tx
        let ix = ore::instruction::mine(
            signer.pubkey(),
            ore::BUS_ADDRESSES[bus_id],
            next_hash.into(),
            nonce,
        );
        match gateway.send_and_confirm(&[ix]).await {
            Ok(sig) => return Ok(sig),
            Err(_err) => {
                // Retry on different bus.
                bus_id += 1;
                if bus_id.ge(&ore::BUS_COUNT) {
                    bus_id = 0;
                }
            }
        }
    }
}
