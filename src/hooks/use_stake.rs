use dioxus::prelude::*;
use ore_boost_api::state::{boost_pda, stake_pda, Stake};
use solana_extra_wasm::program::spl_token::ui_amount_to_amount;
use steel::Pubkey;

use crate::gateway::{ore::OreGateway, GatewayError, GatewayResult, UiTokenAmount};
use super::{use_gateway, use_wallet, LiquidityPair, Wallet};

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

pub fn use_net_deposits() -> Resource<GatewayResult<UiTokenAmount>> {
    
    // TODO get all stake accounts
    // TODO convert stake accounts into ore liquidity pair balances

    use_resource(move || async move {
        Ok(UiTokenAmount {
            ui_amount: Some(0.0),
            ui_amount_string: "0.0".to_string(),
            amount: "0".to_string(),
            decimals: 8,
        })
    })
}