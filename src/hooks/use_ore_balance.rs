use std::str::FromStr;

use dioxus::prelude::*;
use ore_boost_api::state::Stake;
use solana_extra_wasm::program::spl_token::ui_amount_to_amount;
use solana_sdk::{native_token::lamports_to_sol, pubkey::Pubkey};

use crate::gateway::{
    spl::SplGateway, UiTokenAmount, GatewayError, GatewayResult, Rpc,
};

use super::{use_gateway, use_wallet, LiquidityPair, Wallet};

pub fn use_ore_balance() -> Resource<GatewayResult<UiTokenAmount>> {
    let wallet_status = use_wallet();
    use_resource(move || async move {
        match *wallet_status.read() {
            Wallet::Disconnected => Err(GatewayError::AccountNotFound.into()),
            Wallet::Connected(pubkey) => use_gateway().rpc.get_ore_balance(&pubkey).await,
        }
    })
}

pub fn use_ore_supply() -> Resource<GatewayResult<UiTokenAmount>> {
    use_resource(move || async move {
        use_gateway()
            .rpc
            .get_token_supply(&ore_api::consts::MINT_ADDRESS)
            .await
            .map_err(GatewayError::from)
    })
}

pub fn use_token_balance(mint: Pubkey) -> Resource<GatewayResult<UiTokenAmount>> {
    let wallet_status = use_wallet();
    use_resource(move || async move {
        match *wallet_status.read() {
            Wallet::Disconnected => Err(GatewayError::AccountNotFound.into()),
            Wallet::Connected(pubkey) => get_token_balance(pubkey, mint).await,
        }
    })
}

pub fn use_token_balances(liquidity_pair: Resource<GatewayResult<LiquidityPair>>) -> (Resource<GatewayResult<UiTokenAmount>>, Resource<GatewayResult<UiTokenAmount>>) {
    let wallet = use_wallet();

    let token_a_balance = use_resource(move || async move {
        if let Some(Ok(liquidity_pair)) = liquidity_pair.read().as_ref() {
            match *wallet.read() {
                Wallet::Disconnected => Err(GatewayError::AccountNotFound.into()),
                Wallet::Connected(authority) => get_token_balance(authority, liquidity_pair.token_a.mint).await,
            }
        } else {
            Err(GatewayError::Unknown)
        }
    });

    let token_b_balance = use_resource(move || async move {
        if let Some(Ok(liquidity_pair)) = liquidity_pair.read().as_ref() {
            match *wallet.read() {
                Wallet::Disconnected => Err(GatewayError::AccountNotFound.into()),
                Wallet::Connected(authority) => get_token_balance(authority, liquidity_pair.token_b.mint).await,
            }
        } else {
            Err(GatewayError::Unknown)
        }
    });

    (token_a_balance, token_b_balance)
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

pub async fn get_token_balance(pubkey: Pubkey, mint: Pubkey) -> GatewayResult<UiTokenAmount> {
    if mint == Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap() {
        use_gateway()
            .rpc
            .get_balance(&pubkey)
            .await
            .map(|lamports| {
                let sol = lamports_to_sol(lamports);
                UiTokenAmount {
                    ui_amount: Some(sol),
                    decimals: 8,
                    amount: format!("{}", lamports).to_owned(),
                    ui_amount_string: format!("{}", sol).to_owned(),
                }
            })
            .map_err(GatewayError::from)
    } else {
        use_gateway()
            .rpc
            .get_token_balance(&pubkey, &mint)
            .await
            .map_err(GatewayError::from)
    }
}
