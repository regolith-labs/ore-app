use std::rc::Rc;
#[cfg(feature = "desktop")]
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};

use dioxus::prelude::UseSharedState;
use dioxus_std::utils::{channel::UseChannel, rw::UseRw};
use ore::{state::Treasury, BUS_COUNT, EPOCH_DURATION};
use rand::Rng;
use serde::{Deserialize, Serialize};
#[cfg(feature = "web")]
use serde_wasm_bindgen::to_value;
#[cfg(feature = "web")]
use solana_client_wasm::solana_sdk::{
    compute_budget::ComputeBudgetInstruction,
    keccak::{hashv, Hash as KeccakHash},
    pubkey::Pubkey,
};
#[cfg(feature = "desktop")]
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction,
    keccak::{hashv, Hash as KeccakHash},
    pubkey::Pubkey,
    signer::Signer,
};
#[cfg(feature = "web")]
use web_sys::Worker;
#[cfg(feature = "web")]
use web_time::Duration;

#[cfg(feature = "web")]
use crate::worker::create_worker;
use crate::{
    gateway::{
        signer, AsyncResult, Gateway, GatewayError, GatewayResult, CU_LIMIT_MINE, CU_LIMIT_RESET,
    },
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

    // TODO
    pub async fn start_mining(&self, challenge: [u8; 32]) {
        // #[cfg(feature = "web")]
        // {
        //     self.worker
        //         .post_message(
        //             &to_value(
        //                 &(MineRequest {
        //                     hash,
        //                     difficulty,
        //                     pubkey: signer,
        //                 }),
        //             )
        //             .unwrap(),
        //         )
        //         .unwrap();
        // }

        // #[cfg(feature = "desktop")]
        // {
        //     let ch = self.ch.clone();
        //     let flag = Arc::new(AtomicBool::new(false));
        //     let result = Arc::new(Mutex::new(MiningResult::default()));
        //     let power_percent = ((self.power_level.read().0 + 1) as f64) / 8f64;
        //     let concurrency = num_cpus::get() as u64;
        //     let tuned_concurrency = ((concurrency as f64) * power_percent).round() as u64;
        //     let handles: Vec<_> = (0..tuned_concurrency)
        //         .map(|i| {
        //             std::thread::spawn({
        //                 let flag = flag.clone();
        //                 let result = result.clone();
        //                 move || {
        //                     let nonce =
        //                         u64::MAX.saturating_div(tuned_concurrency).saturating_mul(i);
        //                     if let Some(res) =
        //                         find_next_hash_par(hash, difficulty, signer, nonce, flag.clone())
        //                     {
        //                         flag.store(true, Ordering::Relaxed);
        //                         let mut w_result = result.lock().unwrap();
        //                         *w_result = res;
        //                     }
        //                 }
        //             })
        //         })
        //         .collect();
        //     async_std::task::spawn(async move {
        //         for h in handles {
        //             h.join().unwrap();
        //         }
        //         let res = {
        //             let r_result = result.lock().unwrap();
        //             r_result.clone()
        //         };
        //         ch.send(res).await.ok();
        //     });
        // }
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
        if nonce % 10_000 == 0 && flag.load(Ordering::Relaxed) {
            return None;
        }
        next_hash = hashv(&[
            hash.as_ref(),
            signer.as_ref(),
            nonce.to_le_bytes().as_slice(),
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

// TODO Update this to run for X seconds
#[cfg(feature = "web")]
pub fn find_next_hash(hash: KeccakHash, difficulty: KeccakHash, signer: Pubkey) -> MiningResult {
    let mut next_hash: KeccakHash;
    let mut nonce = 0u64;
    loop {
        next_hash = hashv(&[
            hash.as_ref(),
            signer.as_ref(),
            nonce.to_le_bytes().as_slice(),
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

fn find_open_bus() -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..BUS_COUNT)
}

pub async fn submit_solution(
    gateway: &Rc<Gateway>,
    res: &MiningResult,
    priority_fee: u64,
) -> GatewayResult<()> {
    // Submit mine tx.
    let next_hash = res.hash;
    let nonce = res.nonce;
    let signer = signer();

    // Read current treasury value
    // let treasury = match *treasury_rw.read().unwrap() {
    //     AsyncResult::Ok(treasury) => treasury,
    //     _ => return Err(GatewayError::Unknown), // TODO
    // };

    // Find a valid bus
    let mut rng = rand::thread_rng();
    loop {
        // Check if epoch needs to be reset
        // TODO Reset ix
        // if let Ok(clock) = gateway.get_clock().await {
        //     let epoch_end_at = treasury.last_reset_at.saturating_add(EPOCH_DURATION);

        //     // Submit restart epoch tx, if needed
        //     if clock.unix_timestamp.ge(&epoch_end_at) {
        //         // There are a lot of miners right now, randomize who tries the reset
        //         let selected_to_reset = rng.gen_range(0..10).eq(&0);
        //         if selected_to_reset {
        //             let cu_limit_ix =
        //                 ComputeBudgetInstruction::set_compute_unit_limit(CU_LIMIT_RESET);
        //             let cu_price_ix =
        //                 ComputeBudgetInstruction::set_compute_unit_price(priority_fee);
        //             let ix = ore::instruction::reset(signer.pubkey());
        //             gateway
        //                 .send_and_confirm(&[cu_limit_ix, cu_price_ix, ix], false, true)
        //                 .await
        //                 .ok();
        //         }
        //     }
        // }

        // Submit mine tx
        let bus_id = find_open_bus();
        log::info!("Using bus {}", bus_id);
        // TODO
        // let cu_limit_ix = ComputeBudgetInstruction::set_compute_unit_limit(CU_LIMIT_MINE);
        // let cu_price_ix = ComputeBudgetInstruction::set_compute_unit_price(priority_fee);
        // let ix = ore::instruction::mine(
        //     signer.pubkey(),
        //     ore::BUS_ADDRESSES[bus_id],
        //     next_hash.into(),
        //     nonce,
        // );
        // match gateway
        //     .send_and_confirm(&[cu_limit_ix, cu_price_ix, ix], false, false)
        //     .await
        // {
        //     Ok(_sig) => return Ok(()),
        //     Err(err) => {
        //         // TODO Retry
        //         // TODO It seems this can error can occur sometimes, even while tx was submitted
        //         log::error!("Error submitting: {:?}", err);
        //     }
        // }
    }
}
