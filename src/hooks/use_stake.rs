use std::collections::HashMap;

use dioxus::prelude::*;
use ore_api::consts::{MINT_ADDRESS, TOKEN_DECIMALS};
use ore_boost_api::state::{boost_pda, stake_pda, Stake};
use solana_extra_wasm::program::spl_token::{amount_to_ui_amount, ui_amount_to_amount};
use steel::Pubkey;

use crate::{config::LISTED_BOOSTS, gateway::{ore::OreGateway, GatewayError, GatewayResult, UiTokenAmount}};
use super::{use_gateway, use_liquidity_pair, use_wallet, LiquidityPair, Wallet};

pub fn use_stake(mint: Pubkey) -> Resource<GatewayResult<Stake>> {
    let wallet = use_wallet();
    use_resource(move || async move {
        match *wallet.read() {
            Wallet::Disconnected => Err(GatewayError::WalletDisconnected),
            Wallet::Connected(address) => {
                let boost_address = boost_pda(mint).0;
                let stake_address = stake_pda(address, boost_address).0;
                use_gateway().rpc.get_stake(stake_address).await.map_err(GatewayError::from)
            }
        }
    })
}

pub fn use_stake_balances(
    liquidity_pair: Resource<GatewayResult<LiquidityPair>>, 
    stake: Resource<GatewayResult<Stake>>
) -> (Resource<GatewayResult<UiTokenAmount>>, Resource<GatewayResult<UiTokenAmount>>) {
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

pub fn use_stake_accounts() -> HashMap<Pubkey, Resource<GatewayResult<Stake>>> {
    let mut resources = HashMap::with_capacity(LISTED_BOOSTS.len() + 1);

    // Insert idle stake account
    resources.insert(MINT_ADDRESS, use_stake(MINT_ADDRESS));

    // Insert liquidity pair stake accounts
    for boost_meta in LISTED_BOOSTS.iter() {
        resources.insert(boost_meta.lp_mint, use_stake(boost_meta.lp_mint));
    }

    resources
}

pub fn use_liquidity_pairs() -> HashMap<Pubkey, Resource<GatewayResult<LiquidityPair>>> {
    let mut resources = HashMap::with_capacity(LISTED_BOOSTS.len());
    for boost_meta in LISTED_BOOSTS.iter() {
        resources.insert(boost_meta.lp_mint, use_liquidity_pair(boost_meta.clone()));
    }
    resources
}

pub fn use_net_deposits(
    stake_accounts: HashMap<Pubkey, Resource<GatewayResult<Stake>>>, 
    liquidity_pairs: HashMap<Pubkey, Resource<GatewayResult<LiquidityPair>>>
) -> Resource<GatewayResult<UiTokenAmount>> {
    use_resource(move || {
        let stake_accounts = stake_accounts.clone();
        let liquidity_pairs = liquidity_pairs.clone();
        async move {
            // Iterate through all stake accounts and sum the deposits
            let mut net_deposits = 0;
            for (mint, stake) in stake_accounts.iter() {
                if let Some(Ok(stake)) = *stake.read() {
                    if mint == &MINT_ADDRESS { 
                        net_deposits += stake.balance;
                    } else if let Some(liquidity_pair_resource) = liquidity_pairs.get(&mint) {
                        if let Some(Ok(liquidity_pair)) = liquidity_pair_resource.cloned() {
                            let (ore_amount_f64, _token_amount_f64, _token_ticker, _token_decimals) = liquidity_pair.get_stake_amounts(stake.balance);
                            let ore_amount_u64 = ui_amount_to_amount(ore_amount_f64, TOKEN_DECIMALS);
                            net_deposits += ore_amount_u64;
                        }
                    }
                }
            }

            // Convert to a UI amount
            let net_deposits_f64 = amount_to_ui_amount(net_deposits, TOKEN_DECIMALS);
            Ok(UiTokenAmount {
                ui_amount: Some(net_deposits_f64),
                ui_amount_string: format!("{:.1$}", net_deposits_f64, TOKEN_DECIMALS as usize)
                    .trim_end_matches("0")
                    .trim_end_matches(".")
                    .to_string(),
                amount: net_deposits.to_string(),
                decimals: TOKEN_DECIMALS,
            })
        }
    })
}
pub fn use_net_yield(stake_accounts: HashMap<Pubkey, Resource<GatewayResult<Stake>>>) -> Resource<GatewayResult<UiTokenAmount>> {
    use_resource(move || {
        let stake_accounts = stake_accounts.clone();
        async move {
            // Iterate through all stake accounts and sum the rewards
            let mut net_yield = 0;
            for (_mint, stake) in stake_accounts.iter() {
                if let Some(Ok(stake)) = *stake.read() {
                    net_yield += stake.rewards;
                }
            }

            // Convert to a UI amount
            let net_yield_f64 = amount_to_ui_amount(net_yield, TOKEN_DECIMALS);
            Ok(UiTokenAmount {
                ui_amount: Some(net_yield_f64),
                ui_amount_string: format!("{:.1$}", net_yield_f64, TOKEN_DECIMALS as usize)
                    .trim_end_matches("0")
                    .trim_end_matches(".")
                    .to_string(),
                amount: net_yield.to_string(),
                decimals: TOKEN_DECIMALS,
            })
        }
    })
}