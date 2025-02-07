use dioxus::prelude::*;

use crate::hooks::{
    get_cutoff, get_updated_challenge, post_solution, use_gateway, use_member_db, use_miner,
    use_miner_is_active, use_wallet, GetPubkey,
};
use crate::config::FIRST_POOL;

pub fn MinerController() -> Element {
    let wallet = use_wallet();

    // on off button
    let is_active = use_miner_is_active();

    // register with first pool
    // TODO: round robin select
    let pool = FIRST_POOL.clone();
    let pool_url = pool.url.clone();
    let member = use_member_db(pool_url.clone());

    // build miner
    let (from_miner, mut to_miner) = use_miner();
    let mut last_hash_at = use_signal(|| 0);

    // restart miner
    use_effect(move || {
        if let true = is_active.read().0 {
            to_miner.restart();
        }
    });

    // next challenge
    let challenge = use_resource(move || {
        let gateway = use_gateway();
        let pool_url = pool_url.clone();
        let member = &*member.read();
        let member = member.clone();
        let last_hash_at = *last_hash_at.read();
        async move {
            if let Some(Ok(member)) = member {
                get_updated_challenge(
                    &gateway.http,
                    pool_url.as_str(),
                    member.authority.as_str(),
                    last_hash_at,
                )
                .await
            } else {
                Err(crate::gateway::GatewayError::AccountNotFound)
            }
        }
    });

    // challenge sender
    use_effect(move || {
        let is_active = is_active.read().0;
        let member = &*member.read();
        let member = member.clone();
        let challenge = *challenge.read();
        if let (Some(Ok(member)), Some(Ok(challenge)), true) = (member, challenge, is_active) {
            spawn(async move {
                let cutoff_time = get_cutoff(challenge.challenge.lash_hash_at, 5).await;
                match cutoff_time {
                    Ok(cutoff_time) => {
                        to_miner.send(ore_miner_types::InputMessage {
                            member,
                            challenge,
                            cutoff_time,
                        });
                    }
                    Err(err) => {
                        log::error!("{:?}", err);
                    }
                }
            });
        }
    });

    // solutions receiver
    use_effect(move || {
        let pubkey = wallet.pubkey();
        let pool_url = pool.url.clone();
        let from_miner_read = &*from_miner.read();
        if let ore_miner_types::OutputMessage::Solution(solution) = from_miner_read {
            let gateway = use_gateway();
            let solution = solution.clone();
            if let Ok(pubkey) = pubkey {
                spawn(async move {
                    if let Err(err) =
                        post_solution(&gateway.http, pool_url.as_str(), &pubkey, &solution).await
                    {
                        log::error!("{:?}", err);
                    }
                });
            }
        }
        if let ore_miner_types::OutputMessage::Expired(lha) = from_miner_read {
            // there may be many workers with the same lha observation
            // only update on the first expiration
            let peek = *last_hash_at.peek();
            if lha > &peek {
                log::info!("updating lha: {:?}:{:?}", peek, lha);
                last_hash_at.set(*lha);
            }
        }
    });

    // TODO: render animation on true, etc
    let mut is_mining = use_signal(|| false);
    use_effect(move || {
        if is_active.read().0 {
            is_mining.set(true);
        } else {
            is_mining.set(false);
        }
    });

    // TODO: pretty
    rsx! {
        // "{last_hash_at}"
        // "{is_mining}"
    }
}
