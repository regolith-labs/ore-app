use dioxus::prelude::*;
use ore_boost_api::state::{Boost, Stake};
use steel::Numeric;

use crate::gateway::GatewayResult;

pub fn use_claimable_yield(
    boost: Resource<GatewayResult<Boost>>,
    stake: Resource<GatewayResult<Stake>>,
) -> Memo<u64> {
    use_memo(move || {
        let mut rewards = 0;
        if let Some(Ok(boost)) = boost.cloned() {
            if let Some(Ok(stake)) = stake.cloned() {
                rewards += stake.rewards;
                if boost.rewards_factor > stake.last_rewards_factor {
                    let accumulated_rewards = boost.rewards_factor - stake.last_rewards_factor;
                    let personal_rewards = accumulated_rewards * Numeric::from_u64(stake.balance);
                    rewards += personal_rewards.to_u64();
                }
            }
        }
        rewards
    })
}
