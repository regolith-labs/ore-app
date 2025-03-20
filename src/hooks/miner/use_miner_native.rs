use std::sync::Arc;

use dioxus::prelude::*;

use anyhow::Result;
use drillx::{equix, Solution};
use futures::StreamExt;
use ore_miner_types::{InputMessage, OutputMessage};
use tokio::sync::Mutex;

/// two way channel between us and miner
pub fn use_miner_provider() {
    // system cores monitor
    let sys = sysinfo::System::new();
    let sys = Arc::new(Mutex::new(sys));
    // from miner receiver
    let mut from_miner = use_context_provider(|| Signal::new(OutputMessage::Init));
    // to miner sender
    let _to_miner: Coroutine<InputMessage> = use_coroutine(
        move |mut rx: dioxus::prelude::UnboundedReceiver<InputMessage>| {
            let sys = Arc::clone(&sys);
            async move {
                // build continuous solutions channel
                let (sender, mut receiver) =
                    tokio::sync::mpsc::unbounded_channel::<OutputMessage>();
                // poll for messages from controller
                while let Some(msg) = rx.next().await {
                    log::info!("to worker: {:?}", msg);
                    // spawn miner
                    let sender = sender.clone();
                    tokio::spawn(async move {
                        let device_id = 0;
                        let cores = msg.cores as u8;
                        let challenge = msg.challenge.challenge;
                        // build nonce space
                        match nonce_indices(&msg.member, &msg.challenge, cores, device_id) {
                            Ok(nonce_indices) => {
                                // spawn miner threads
                                if let Err(err) = find_hash_par(
                                    &challenge.challenge,
                                    challenge.lash_hash_at,
                                    nonce_indices.as_slice(),
                                    msg.cutoff_time as u64,
                                    challenge.min_difficulty as u32,
                                    cores,
                                    &sender,
                                )
                                .await
                                {
                                    log::error!("{:?}", err);
                                }
                            }
                            Err(err) => {
                                log::error!("{:?}", err);
                            }
                        }
                    });
                    // listen for solutions from miner
                    let mut best_difficulty = 0;
                    while let Some(msg) = receiver.recv().await {
                        // submit best solutions
                        if let OutputMessage::Solution(solution) = msg {
                            let difficulty = solution.to_hash().difficulty();
                            // submit
                            if difficulty.gt(&best_difficulty) {
                                from_miner.set(msg.clone());
                                best_difficulty = difficulty;
                                log::info!("found new best difficulty: {}", best_difficulty);
                            }
                        }
                        // exit if expired
                        if let OutputMessage::Expired(_) = msg {
                            from_miner.set(msg.clone());
                            break;
                        }
                        // time remaining
                        if let OutputMessage::TimeRemaining(seconds, _) = msg {
                            // sleep to allow solution submissions to process
                            tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
                            // check cpu utilization
                            let cpus = {
                                let mut sys = sys.lock().await;
                                sys.refresh_cpu_usage();
                                sys.cpus()
                                    .into_iter()
                                    .map(|cpu| cpu.cpu_usage())
                                    .collect::<Vec<_>>()
                            };
                            // send cpu utilization
                            let msg = OutputMessage::TimeRemaining(seconds, cpus);
                            from_miner.set(msg);
                        }
                    }
                }
            }
        },
    );
}

async fn find_hash_par(
    challenge: &[u8; 32],
    last_hash_at: i64,
    nonce_indices: &[u64],
    cutoff_time: u64,
    min_difficulty: u32,
    cores: u8,
    solutions_channel: &tokio::sync::mpsc::UnboundedSender<OutputMessage>,
) -> Result<()> {
    // get cores
    let core_ids = core_affinity::get_core_ids().ok_or(anyhow::anyhow!("failed to query cores"))?;
    let core_ids = core_ids.into_iter().filter(|id| id.id < (cores as usize));
    // distribute
    for core_id in core_ids {
        let challenge = *challenge;
        std::thread::spawn({
            // init drillx solver
            let mut memory = drillx::equix::SolverMemory::new();
            let solutions_channel = solutions_channel.clone();
            let nonce = nonce_indices[core_id.id];
            move || {
                // pin to core
                let _ = core_affinity::set_for_current(core_id);
                // start hashing
                let timer = std::time::Instant::now();
                let mut nonce = nonce;
                let mut best_difficulty = 0;
                loop {
                    // get hashes
                    let hxs = solve(&mut memory, &challenge, &nonce.to_le_bytes());
                    // look for best difficulty score in all hashes
                    for hx in hxs {
                        let difficulty = hx.difficulty();
                        if difficulty.gt(&best_difficulty) {
                            best_difficulty = difficulty;
                            // continously submit best solution to pool
                            if difficulty.ge(&min_difficulty) {
                                let diggest = hx.d;
                                let nonce = nonce.to_le_bytes();
                                log::info!("/////////////////////////////////////");
                                log::info!("difficulty: {}", difficulty);
                                let solution = Solution {
                                    d: diggest,
                                    n: nonce,
                                };
                                let solution = OutputMessage::Solution(solution);
                                if let Err(err) = solutions_channel.send(solution) {
                                    log::error!("{:?}", err);
                                }
                            }
                        }
                    }
                    // exit if time has elapsed
                    if nonce % 100 == 0 {
                        if timer.elapsed().as_secs().ge(&cutoff_time) {
                            // send expiration message
                            if core_id.id == 0 {
                                let expired = OutputMessage::Expired(last_hash_at);
                                if let Err(err) = solutions_channel.send(expired) {
                                    log::error!("{:?}", err);
                                }
                            }
                            break;
                        } else if core_id.id == 0 {
                            let remaining = cutoff_time.saturating_sub(timer.elapsed().as_secs());
                            if let Err(err) = solutions_channel
                                .send(OutputMessage::TimeRemaining(remaining as i64, vec![]))
                            {
                                log::error!("{:?}", err);
                            }
                        }
                    }
                    // increment nonce
                    nonce += 1;
                }
            }
        });
    }
    Ok(())
}

fn nonce_indices(
    member: &ore_pool_types::Member,
    challenge: &ore_pool_types::MemberChallenge,
    cores: u8,
    device_id: u8,
) -> Result<Vec<u64>> {
    // build nonce indices
    let cores = cores as u64;
    let num_total_members = challenge.num_total_members.max(1);
    let member_search_space_size = u64::MAX.saturating_div(num_total_members);
    let device_search_space_size =
        member_search_space_size.saturating_div(challenge.num_devices as u64);
    // check device id doesn't go beyond pool limit
    if device_id > challenge.num_devices {
        return Err(anyhow::anyhow!("too many devices"));
    }
    // calculate bounds on nonce space
    let left_bound = member_search_space_size.saturating_mul(member.id as u64)
        + (device_id as u64).saturating_mul(device_search_space_size);
    log::info!("left bound: {}", left_bound);
    // split nonce-device space for muliple cores
    let range_per_core = device_search_space_size.saturating_div(cores);
    let mut nonce_indices = Vec::with_capacity(cores as usize);
    for n in 0..(cores) {
        let index = left_bound.saturating_add(n.saturating_mul(range_per_core));
        nonce_indices.push(index);
    }
    Ok(nonce_indices)
}

#[inline(always)]
fn solve(
    mem: &mut equix::SolverMemory,
    challenge: &[u8; 32],
    nonce: &[u8; 8],
) -> Vec<drillx::Hash> {
    drillx::hashes_with_memory(mem, challenge, nonce)
}
