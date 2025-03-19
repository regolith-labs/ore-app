use dioxus::prelude::*;
use ore_api::state::Proof;
use ore_boost_api::state::{Boost, Stake};
use steel::Numeric;

use crate::gateway::GatewayResult;

pub fn use_claimable_yield(
    boost: Resource<GatewayResult<Boost>>,
    boost_proof: Resource<GatewayResult<Proof>>,
    stake: Resource<GatewayResult<Stake>>,
) -> Memo<u64> {
    use_memo(move || {
        let mut rewards = 0;
        if let Some(Ok(boost)) = boost.cloned() {
            if let Some(Ok(stake)) = stake.cloned() {
                if let Some(Ok(boost_proof)) = boost_proof.cloned() {
                    rewards += calculate_claimable_yield(boost, boost_proof, stake);
                }
            }
        }
        rewards
    })
}

pub fn calculate_claimable_yield(boost: Boost, boost_proof: Proof, stake: Stake) -> u64 {
    let mut rewards = 0;
    let mut boost_rewards_factor = boost.rewards_factor;
    if boost_proof.balance > 0 {
        boost_rewards_factor += Numeric::from_fraction(boost_proof.balance, boost.total_deposits);
    }

    if boost_rewards_factor > stake.last_rewards_factor {
        let accumulated_rewards = boost_rewards_factor - stake.last_rewards_factor;
        let personal_rewards = accumulated_rewards * Numeric::from_u64(stake.balance);
        rewards += personal_rewards.to_u64();
    }

    rewards
}
