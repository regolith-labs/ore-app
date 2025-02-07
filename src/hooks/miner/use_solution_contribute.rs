use dioxus::prelude::*;
use ore_miner_types::OutputMessage;

use crate::{
    config::Pool,
    hooks::{use_gateway, use_wallet, GetPubkey},
    gateway::pool::PoolGateway,
};

pub fn use_solution_contribute(
    mut last_hash_at: Signal<i64>,
    from_miner: Signal<OutputMessage>,
    pool: Resource<Pool>,
) -> Effect {
    let wallet = use_wallet();
    use_effect(move || {
        let Ok(pubkey) = wallet.pubkey() else {
            return;
        };
        let Some(pool) = pool.cloned() else {
            return;
        };
        
        match *from_miner.read() {
            OutputMessage::Solution(solution) => {
                spawn(async move {
                    if let Err(err) = use_gateway().post_solution(pubkey, pool.url, &solution).await {
                        log::error!("Error posting solution: {:?}", err);
                    }
                });
            }
            OutputMessage::Expired(lha) => {
                let peek = *last_hash_at.peek();
                if lha > peek { last_hash_at.set(lha); }
            }
            _ => {}
        }
    })
}