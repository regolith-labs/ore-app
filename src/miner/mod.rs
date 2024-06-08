mod web_worker;

pub use web_worker::*;

use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_std::utils::channel::UseChannel;
use drillx::{Hash, Solution};
use ore::{state::Treasury, BUS_COUNT, EPOCH_DURATION};
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use solana_client_wasm::solana_sdk::{
    compute_budget::ComputeBudgetInstruction, pubkey::Pubkey, signature::Signature, signer::Signer,
};
use web_sys::Worker;
use web_time::Duration;

use crate::{
    gateway::{
        signer, AsyncResult, Gateway, GatewayError, GatewayResult, CU_LIMIT_MINE, CU_LIMIT_RESET,
    },
    hooks::{PowerLevel, PriorityFee},
};

// TODO Create channel for webworkers to send solutions on
/// Miner encapsulates the logic needed to efficiently mine for valid hashes according to the application runtime and hardware.
// #[derive(PartialEq)]
pub struct Miner {
    power_level: Signal<PowerLevel>,
    priority_fee: Signal<PriorityFee>,
    // cx: Coroutine<WebWorkerResponse>,
    web_worker: Worker,
}

// TODO Aggregate results from web workers

// // TODO Create channel to receive results from webworker
// let ch = use_channel::<MiningResult>(cx, 1);

impl Miner {
    pub fn new(
        cx: UseChannel<WebWorkerResponse>,
        power_level: Signal<PowerLevel>,
        priority_fee: Signal<PriorityFee>,
    ) -> Self {
        Self {
            power_level: power_level.clone(),
            priority_fee: priority_fee.clone(),

            // TODO Create as many webworkers as there are cores
            web_worker: create_web_worker(cx),
        }
    }

    pub fn stop(&self) {
        // TODO interrupt current work (optimization)
    }

    // TODO
    pub async fn start_mining(&self, challenge: [u8; 32], cutoff_time: u64) {
        self.start_mining_web(challenge, cutoff_time).await;
    }

    // TODO Dispatch a difference nonce to each webworker (based on power level)
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

    // pub async fn wait_for_solution(&self) {
    //     let mut messages = vec![];
    //     for _ in 0..4 {
    //         if let Some(message) = self.ch.receiver().recv().await {
    //             messages.push(message);
    //         }
    //     }
    // }
}

// TODO Dispatch a difference nonce to each webworker (based on power level)
pub async fn start_mining_web(web_worker: Worker, challenge: [u8; 32], cutoff_time: u64) {
    web_worker
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
