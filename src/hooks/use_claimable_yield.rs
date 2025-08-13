use dioxus::prelude::*;
use ore_boost_api::state::{Boost, Config as BoostConfig, Stake};
use steel::Numeric;

use crate::{
    gateway::{GatewayResult, UiTokenAmount},
    hooks::use_reserve_balance_wss,
};

use super::use_boost_config_wss;

pub fn use_claimable_yield(
    boost: Signal<GatewayResult<Boost>>,
    stake: Signal<GatewayResult<Stake>>,
) -> Memo<u64> {
    let reserve_balance = use_reserve_balance_wss();
    let boost_config = use_boost_config_wss();
    use_memo(move || {
        let mut rewards = 0;
        if let Ok(boost) = boost.cloned() {
            if let Ok(stake) = stake.cloned() {
                if let Ok(reserve_balance) = reserve_balance.cloned() {
                    if let Ok(boost_config) = boost_config.cloned() {
                        rewards +=
                            calculate_claimable_yield(boost, reserve_balance, stake, boost_config);
                    }
                }
            }
        }
        rewards
    })
}

pub fn calculate_claimable_yield(
    boost: Boost,
    reserve_balance: UiTokenAmount,
    stake: Stake,
    boost_config: BoostConfig,
) -> u64 {
    let mut rewards = stake.rewards;
    let mut config_rewards_factor = boost_config.rewards_factor;
    let mut boost_rewards_factor = boost.rewards_factor;

    let reserve_balance_amount = reserve_balance.amount.parse::<u64>().unwrap();
    if reserve_balance_amount > 0 {
        config_rewards_factor +=
            Numeric::from_fraction(reserve_balance_amount, boost_config.total_weight);
    }

    if config_rewards_factor > boost.last_rewards_factor && boost.total_deposits > 0 {
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
