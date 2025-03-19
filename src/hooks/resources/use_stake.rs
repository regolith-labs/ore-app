use std::collections::HashMap;

use dioxus::prelude::*;
use ore_api::consts::MINT_ADDRESS;
use ore_boost_api::state::{boost_pda, stake_pda, Stake};
use steel::Pubkey;

use crate::{
    config::LISTED_BOOSTS,
    gateway::{ore::OreGateway, GatewayError, GatewayResult, UiTokenAmount},
    hooks::{use_gateway, use_wallet, Wallet},
    solana::spl_token::ui_amount_to_amount,
    utils::LiquidityPair,
};

pub(crate) fn use_stakes_provider() {
    // Hashmap to cache resources
    let mut stakes = HashMap::new();

    // Idle ORE boost
    stakes.insert(MINT_ADDRESS, use_stake_resource(MINT_ADDRESS));

    // Listed boosts
    for boost_meta in LISTED_BOOSTS.iter() {
        stakes.insert(boost_meta.lp_mint, use_stake_resource(boost_meta.lp_mint));
    }

    // Setup context provider
    use_context_provider(|| stakes);
}

fn use_stake_resource(mint_address: Pubkey) -> Resource<GatewayResult<Stake>> {
    let wallet = use_wallet();
    use_resource(move || async move {
        match *wallet.read() {
            Wallet::Disconnected => Err(GatewayError::WalletDisconnected),
            Wallet::Connected(authority) => {
                let boost_address = boost_pda(mint_address).0;
                let stake_address = stake_pda(authority, boost_address).0;
                use_gateway()
                    .get_stake(stake_address)
                    .await
                    .map_err(GatewayError::from)
            }
        }
    })
}

pub fn use_stake(mint_address: Pubkey) -> Resource<GatewayResult<Stake>> {
    let stakes: HashMap<Pubkey, Resource<GatewayResult<Stake>>> = use_context();
    if let Some(stake) = stakes.get(&mint_address) {
        *stake
    } else {
        use_stake_resource(mint_address)
    }
}

pub fn use_all_stakes() -> HashMap<Pubkey, Resource<GatewayResult<Stake>>> {
    use_context()
}

pub fn use_withdrawable_balances(
    liquidity_pair: Resource<GatewayResult<LiquidityPair>>,
    stake: Resource<GatewayResult<Stake>>,
) -> (
    Resource<GatewayResult<UiTokenAmount>>,
    Resource<GatewayResult<UiTokenAmount>>,
) {
    let stake_a_balance = use_resource(move || async move {
        let Some(Ok(stake)) = stake.cloned() else {
            return Err(GatewayError::Unknown);
        };
        let Some(Ok(liquidity_pair)) = liquidity_pair.cloned() else {
            return Err(GatewayError::Unknown);
        };
        let percentage_shares = stake.balance as f64 / liquidity_pair.shares as f64;
        let amount_f64 = liquidity_pair.balance_a_f64 * percentage_shares;
        let token_a_decimals = liquidity_pair.token_a.decimals;
        let amount_u64 = ui_amount_to_amount(amount_f64, token_a_decimals);
        Ok(UiTokenAmount {
            ui_amount: Some(amount_f64),
            ui_amount_string: format!("{:.1$}", amount_f64, token_a_decimals as usize)
                .trim_end_matches("0")
                .trim_end_matches(".")
                .to_string(),
            amount: amount_u64.to_string(),
            decimals: token_a_decimals as u8,
        })
    });

    let stake_b_balance = use_resource(move || async move {
        let Some(Ok(stake)) = stake.cloned() else {
            return Err(GatewayError::Unknown);
        };
        let Some(Ok(liquidity_pair)) = liquidity_pair.cloned() else {
            return Err(GatewayError::Unknown);
        };
        let percentage_shares = stake.balance as f64 / liquidity_pair.shares as f64;
        let amount_f64 = liquidity_pair.balance_b_f64 * percentage_shares;
        let token_b_decimals = liquidity_pair.token_b.decimals;
        let amount_u64 = ui_amount_to_amount(amount_f64, token_b_decimals);
        Ok(UiTokenAmount {
            ui_amount: Some(amount_f64),
            ui_amount_string: format!("{:.1$}", amount_f64, token_b_decimals as usize)
                .trim_end_matches("0")
                .trim_end_matches(".")
                .to_string(),
            amount: amount_u64.to_string(),
            decimals: token_b_decimals as u8,
        })
    });

    (stake_a_balance, stake_b_balance)
}
