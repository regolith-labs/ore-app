use dioxus::prelude::*;

use anyhow::Result;
use drillx::Solution;
use futures::StreamExt;
use ore_miner_types::{InputMessage, OutputMessage};

/// two way channel between us and miner
pub fn use_miner_provider() {
    // from miner receiver
    let mut from_miner = use_context_provider(|| Signal::new(OutputMessage::Init));
    // to miner sender
    let _to_miner: Coroutine<InputMessage> = use_coroutine(
        move |mut rx: dioxus::prelude::UnboundedReceiver<InputMessage>| async move {
            // build continuous solutions channel
            let (sender, mut receiver) = tokio::sync::mpsc::unbounded_channel::<OutputMessage>();
            // poll for messages from controller
            while let Some(msg) = rx.next().await {
                log::info!("to worker: {:?}", msg);
                // spawn miner
                let sender = sender.clone();
                tokio::spawn(async move {
                    let cores = 10;
                    let device_id = 0;
                    let challenge = msg.challenge.challenge;
                    match nonce_indices(&msg.member, &msg.challenge, cores, device_id) {
                        Ok(nonce_indices) => {
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
                while let Some(msg) = receiver.recv().await {
                    from_miner.set(msg);
                    if let OutputMessage::Expired(_) = msg {
                        log::info!("expired");
                        break;
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
        log::info!("core: {:?}", core_id);
        let challenge = *challenge;
        std::thread::spawn({
            log::info!("spawning core: {:?}", core_id);
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
                    let hxs =
                        drillx::hashes_with_memory(&mut memory, &challenge, &nonce.to_le_bytes());
                    // look for best difficulty score in all hashes
                    for hx in hxs {
                        let difficulty = hx.difficulty();
                        if difficulty.gt(&best_difficulty) {
                            best_difficulty = difficulty;
                            // continously submit best solution to pool
                            if difficulty.ge(&min_difficulty) {
                                let diggest = hx.d;
                                let nonce = nonce.to_le_bytes();
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
                    if nonce % 10 == 0 {
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
                            log::info!(
                                "Mining... Time remaining: {}",
                                format_duration(
                                    cutoff_time.saturating_sub(timer.elapsed().as_secs()) as u32
                                ),
                            );
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
    // split nonce-device space for muliple cores
    let range_per_core = device_search_space_size.saturating_div(cores);
    let mut nonce_indices = Vec::with_capacity(cores as usize);
    for n in 0..(cores) {
        let index = left_bound + n * range_per_core;
        nonce_indices.push(index);
    }
    Ok(nonce_indices)
}

fn format_duration(seconds: u32) -> String {
    let minutes = seconds / 60;
    let remaining_seconds = seconds % 60;
    format!("{:02}:{:02}", minutes, remaining_seconds)
}
