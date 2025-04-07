use dioxus::prelude::*;
use ore_api::state::Proof;
use ore_boost_api::state::{Boost, Config as BoostConfig, Stake};
use steel::Numeric;

use crate::gateway::GatewayResult;

use super::{use_boost_config_wss, use_boost_proof_wss};

pub fn use_claimable_yield(
    boost: Signal<GatewayResult<Boost>>,
    stake: Signal<GatewayResult<Stake>>,
) -> Memo<u64> {
    let boost_proof = use_boost_proof_wss();
    let boost_config = use_boost_config_wss();
    use_memo(move || {
        let mut rewards = 0;
        if let Ok(boost) = boost.cloned() {
            if let Ok(stake) = stake.cloned() {
                if let Ok(boost_proof) = boost_proof.cloned() {
                    if let Ok(boost_config) = boost_config.cloned() {
                        rewards +=
                            calculate_claimable_yield(boost, boost_proof, stake, boost_config);
                    }
                }
            }
        }
        rewards
    })
}

pub fn calculate_claimable_yield(
    boost: Boost,
    boost_proof: Proof,
    stake: Stake,
    boost_config: BoostConfig,
) -> u64 {
    let mut rewards = stake.rewards;
    let mut config_rewards_factor = boost_config.rewards_factor;
    let mut boost_rewards_factor = boost.rewards_factor;

    if boost_proof.balance > 0 {
        config_rewards_factor +=
            Numeric::from_fraction(boost_proof.balance, boost_config.total_weight);
    }

    if config_rewards_factor > boost.last_rewards_factor {
        let accumulated_rewards = config_rewards_factor - boost.last_rewards_factor;
        let boost_rewards = accumulated_rewards * Numeric::from_u64(boost.weight);
        boost_rewards_factor += boost_rewards / Numeric::from_u64(boost.total_deposits);
    }

    if boost_rewards_factor > stake.last_rewards_factor {
        let accumulated_rewards = boost_rewards_factor - stake.last_rewards_factor;
        let personal_rewards = accumulated_rewards * Numeric::from_u64(stake.balance);
        rewards += personal_rewards.to_u64();
    }

    rewards
}
