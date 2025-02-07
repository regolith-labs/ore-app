use dioxus::prelude::*;

use crate::hooks::{use_challenge_dispatch, use_member_record, use_miner, use_miner_is_active, use_next_challenge, use_pool, use_solution_contribute};

pub fn MinerController() -> Element {
    // register with first pool
    // TODO: round robin select
    let pool = use_pool();
    let member_record = use_member_record(pool);

    // Miner state
    let (from_miner, mut to_miner) = use_miner();
    let last_hash_at = use_signal(|| 0);

    // Miner loop (fetch challenge, dispatch to miner, post solutions)
    let challenge = use_next_challenge(last_hash_at, member_record, pool);
    use_challenge_dispatch(challenge, member_record, to_miner);
    use_solution_contribute(last_hash_at, from_miner, pool);

    // restart miner
    let is_active = use_miner_is_active();
    use_effect(move || {
        if let true = is_active.read().0 {
            to_miner.restart();
        }
    });

    // TODO: pretty
    rsx! {
        // "{last_hash_at}"
        // "{is_mining}"
    }
}
