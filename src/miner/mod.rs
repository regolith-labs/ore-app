#[cfg(feature = "desktop")]
mod desktop;
#[cfg(feature = "web")]
mod web_worker;

#[cfg(feature = "desktop")]
pub use desktop::*;
#[cfg(feature = "web")]
pub use web_worker::*;

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
use drillx::{Hash, Solution};
use ore::{state::Treasury, BUS_COUNT, EPOCH_DURATION};
use rand::Rng;
use serde::{Deserialize, Serialize};
#[cfg(feature = "web")]
use serde_wasm_bindgen::to_value;
#[cfg(feature = "web")]
use solana_client_wasm::solana_sdk::{compute_budget::ComputeBudgetInstruction, pubkey::Pubkey};
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

use crate::{
    gateway::{
        signer, AsyncResult, Gateway, GatewayError, GatewayResult, CU_LIMIT_MINE, CU_LIMIT_RESET,
    },
    hooks::{PowerLevel, PriorityFee},
};

// TODO Create channel for webworkers to send solutions on
/// Miner encapsulates the logic needed to efficiently mine for valid hashes according to the application runtime and hardware.
#[derive(PartialEq)]
pub struct Miner {
    power_level: UseSharedState<PowerLevel>,
    priority_fee: UseSharedState<PriorityFee>,

    #[cfg(feature = "web")]
    ch: UseChannel<WebWorkerResponse>,
    #[cfg(feature = "web")]
    web_worker: Worker,
}

// TODO Aggregate results from web workers

// // TODO Create channel to receive results from webworker
// let ch = use_channel::<MiningResult>(cx, 1);

impl Miner {
    #[cfg(feature = "web")]
    pub fn new(
        ch: &UseChannel<WebWorkerResponse>,
        power_level: &UseSharedState<PowerLevel>,
        priority_fee: &UseSharedState<PriorityFee>,
    ) -> Self {
        Self {
            power_level: power_level.clone(),
            priority_fee: priority_fee.clone(),

            // TODO Create as many webworkers as there are cores
            ch: ch.clone(),
            web_worker: create_web_worker(ch),
        }
    }

    #[cfg(feature = "desktop")]
    pub fn new(
        power_level: &UseSharedState<PowerLevel>,
        priority_fee: &UseSharedState<PriorityFee>,
    ) -> Self {
        Self {
            power_level: power_level.clone(),
            priority_fee: priority_fee.clone(),
        }
    }

    pub fn stop(&self) {
        // TODO interrupt current work (optimization)
    }

    // TODO
    pub async fn start_mining(&self, challenge: [u8; 32], cutoff_time: u64) {
        #[cfg(feature = "web")]
        self.start_mining_web(challenge, cutoff_time).await;

        #[cfg(feature = "desktop")]
        self.start_mining_desktop(challenge).await;
    }

    // TODO Dispatch a difference nonce to each webworker (based on power level)
    #[cfg(feature = "web")]
    pub async fn start_mining_web(&self, challenge: [u8; 32], cutoff_time: u64) {
        self.web_worker
            .post_message(
                &to_value(
                    &(WebWorkerRequest {
                        challenge,
                        nonce: 0,
                        cutoff_time,
                    }),
                )
                .unwrap(),
            )
            .unwrap();
    }

    #[cfg(feature = "desktop")]
    pub async fn start_mining_desktop(&self, challenge: [u8; 32]) {
        // TODO
    }

    // pub async fn wait_for_solution(&self) {
    //     let mut messages = vec![];
    //     for _ in 0..4 {
    //         if let Some(message) = self.ch.receiver().recv().await {
    //             messages.push(message);
    //         }
    //     }
    // }
}

pub async fn submit_solution(
    gateway: &Rc<Gateway>,
    solution: &Solution,
    priority_fee: u64,
) -> GatewayResult<()> {
    // Submit mine tx.
    let signer = signer();

    // TODO Get config
    // Read current treasury value
    // let treasury = match *treasury_rw.read().unwrap() {
    //     AsyncResult::Ok(treasury) => treasury,
    //     _ => return Err(GatewayError::Unknown), // TODO
    // };

    // Find a valid bus
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
        let bus_id = pick_bus();
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

fn pick_bus() -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..BUS_COUNT)
}
