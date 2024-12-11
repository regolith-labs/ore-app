use futures::StreamExt;
#[cfg(feature = "worker")]
use gloo_worker::Registrable;
use gloo_worker::{Worker, WorkerScope};
use serde::{Deserialize, Serialize};
#[cfg(feature = "worker")]
use wasm_bindgen::prelude::*;

mod error;

pub struct Miner;

#[derive(Debug, Serialize, Deserialize)]
pub struct InputMessage {
    pub member: ore_pool_types::Member,
    pub challenge: ore_pool_types::MemberChallengeV2,
    pub cutoff_time: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OutputMessage {
    Init,
    Solution(drillx::Solution),
    Expired(LastHashAt),
}
type LastHashAt = i64;

impl Worker for Miner {
    /// Update message type.
    type Message = String;
    /// Incoming message type.
    type Input = InputMessage;
    /// Outgoing message type.
    type Output = OutputMessage;

    /// Creates an instance of a worker.
    fn create(_: &WorkerScope<Self>) -> Self {
        Miner
    }

    /// Receives an update.
    ///
    /// This method is called when the worker send messages to itself via [`WorkerScope::send_message`].
    fn update(&mut self, _scope: &WorkerScope<Self>, _msg: Self::Message) {}

    /// Receives an input from a connected bridge.
    ///
    /// When a bridge sends an input via [`WorkerBridge::send`](crate::WorkerBridge::send), the worker will receive the
    /// input via this method.
    fn received(
        &mut self,
        scope: &WorkerScope<Self>,
        msg: Self::Input,
        id: gloo_worker::HandlerId,
    ) {
        log::info!("challenge received: {:?}", msg);
        // continuous submission channel
        let (tx, mut rx) = futures::channel::mpsc::unbounded::<OutputMessage>();
        // listen for solutions
        let scope = scope.clone();
        wasm_bindgen_futures::spawn_local(async move {
            while let Some(msg) = rx.next().await {
                log::info!("{:?}", msg);
                scope.respond(id, msg)
            }
        });
        // mine for solutions
        if let Err(err) = mine(msg.member, msg.challenge, msg.cutoff_time, tx) {
            log::error!("{:?}", err);
        }
    }
}

fn mine(
    member: ore_pool_types::Member,
    challenge: ore_pool_types::MemberChallengeV2,
    cutoff_time: u64,
    sender: futures::channel::mpsc::UnboundedSender<OutputMessage>,
) -> Result<(), error::Error> {
    // build nonce indices
    let nonce_index = member.id as u64;
    let num_total_members = challenge.num_total_members.max(1);
    let u64_unit = u64::MAX.saturating_div(num_total_members);
    // split member nonce space for multiple devices
    let nonce_unit = u64_unit.saturating_div(challenge.num_devices as u64);
    if challenge.device_id.gt(&challenge.num_devices) {
        return Err(error::Error::TooManyDevices);
    }
    let device_id = challenge.device_id.saturating_sub(1) as u64;
    let left_bound = u64_unit.saturating_mul(nonce_index) + device_id.saturating_mul(nonce_unit);
    // start hashing
    let timer = instant::Instant::now();
    let mut nonce = left_bound;
    let mut best_difficulty = 0;
    let mut memory = drillx::equix::SolverMemory::new();
    loop {
        // get hashes
        let hxs = drillx::hashes_with_memory(
            &mut memory,
            &challenge.challenge.challenge,
            &nonce.to_le_bytes(),
        );
        // look for best difficulty score in all hashes
        for hx in hxs {
            let difficulty = hx.difficulty() as u64;
            // push continuous update thru channel
            if difficulty.gt(&best_difficulty) {
                best_difficulty = difficulty;
                if difficulty.ge(&challenge.challenge.min_difficulty) {
                    let digest = hx.d;
                    let nonce = nonce.to_le_bytes();
                    let solution = drillx::Solution {
                        d: digest,
                        n: nonce,
                    };
                    if let Err(err) = sender.unbounded_send(OutputMessage::Solution(solution)) {
                        log::error!("{:?}", err);
                    }
                }
            }
        }
        // exit if time has elapsed
        if nonce % 100 == 0 {
            let time_expired = timer.elapsed().as_secs().ge(&cutoff_time);
            let sufficient = best_difficulty.ge(&challenge.challenge.min_difficulty);
            if time_expired && sufficient {
                if let Err(err) =
                    sender.unbounded_send(OutputMessage::Expired(challenge.challenge.lash_hash_at))
                {
                    log::error!("{:?}", err);
                }
                break;
            }
        }
        // increment nonce
        nonce += 1;
    }
    Ok(())
}

#[cfg(feature = "worker")]
#[wasm_bindgen(start)]
pub async fn register_miner() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("starting miner");
    Miner::registrar().register();
}
