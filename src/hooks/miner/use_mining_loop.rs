use dioxus::prelude::*;

use super::{use_challenge_dispatch, use_miner, use_miner_is_active, use_next_challenge, use_solution_contribute};

pub fn use_mining_loop() {
    // Miner pub/sub channels
    let (from_miner, mut to_miner) = use_miner();
    let last_hash_at = use_signal(|| 0);

    // Miner loop (fetch challenge, dispatch to miner, post solutions)
    let challenge = use_next_challenge(last_hash_at);
    use_challenge_dispatch(challenge, to_miner);
    use_solution_contribute(last_hash_at, from_miner);

    // restart miner
    let is_active = use_miner_is_active();
    use_effect(move || {
        if *is_active.read() {
            to_miner.restart();
        }
    });
}