use dioxus::prelude::*;
use ore_miner_types::OutputMessage;

use crate::{
    gateway::pool::PoolGateway, hooks::{use_gateway, use_member_record, use_pool_url, use_wallet, GetPubkey}
};

use super::{use_miner_is_active, use_miner_status, MinerStatus};

pub fn use_solution_contribute(
    mut last_hash_at: Signal<i64>,
    from_miner: Signal<OutputMessage>,
) -> Effect {
    let wallet = use_wallet();
    let pool_url = use_pool_url();
    let mut member_record = use_member_record();
    let mut miner_status = use_miner_status();
    let is_active = use_miner_is_active();
    use_effect(move || {
        // Check status
        let Ok(pubkey) = wallet.pubkey() else {
            return;
        };
        let Some(pool_url) = pool_url.cloned() else {
            return;
        };
        if !*is_active.read() {
            return;
        }

        // Process messsage from miner
        match *from_miner.read() {
            OutputMessage::Solution(solution) => {
                // Submit solution
                spawn(async move {
                    miner_status.set(MinerStatus::SubmittingSolution);
                    if let Err(err) = use_gateway().post_solution(pubkey, pool_url, &solution).await {
                        log::error!("Error posting solution: {:?}", err);
                    }
                    member_record.restart();
                });
            }
            OutputMessage::Expired(lha) => {
                // Update last hash at
                let peek = *last_hash_at.peek();
                if lha > peek { last_hash_at.set(lha); }
            }
            _ => {}
        }
    })
}