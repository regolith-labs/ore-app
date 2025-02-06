use dioxus::prelude::*;
use ore_boost_api::state::{boost_pda, Boost};
use steel::Pubkey;

use crate::{config::{BoostMeta, LpType, Token, LISTED_TOKENS, LISTED_TOKENS_BY_TICKER}, gateway::{kamino::KaminoGateway, meteora::MeteoraGateway, ore::OreGateway, GatewayError, GatewayResult, Rpc, UiTokenAmount}};
use super::{get_token_balance, use_gateway, use_wallet, Wallet};

pub fn use_boost(mint: Pubkey) -> Resource<GatewayResult<Boost>> {
    use_resource(move || async move {
        let boost_address = boost_pda(mint).0;
        use_gateway().rpc.get_boost(boost_address).await.map_err(GatewayError::from)
    })
}

#[derive(Debug, Clone, PartialEq)]
pub struct LiquidityPair {
    pub token_a: Token,
    pub token_b: Token,
    pub balance_a_f64: f64,
    pub balance_b_f64: f64,
    pub total_value_usd: f64,
    pub shares: u64,
}

impl LiquidityPair {
    pub fn get_stake_amounts(&self, stake_balance: u64) -> (f64, f64, String, u8) {
        let stake_share = stake_balance as f64 / self.shares as f64;
        let stake_amount_a = self.balance_a_f64 * stake_share;
        let stake_amount_b = self.balance_b_f64 * stake_share;
        if self.token_a.ticker == "ORE" {
            (stake_amount_a, stake_amount_b, self.token_b.ticker.clone(), self.token_b.decimals)
        } else {
            (stake_amount_b, stake_amount_a, self.token_a.ticker.clone(), self.token_a.decimals)
        }
    }
}

pub fn use_liquidity_pair(boost_meta: BoostMeta) -> Resource<GatewayResult<LiquidityPair>> {
    let lp_type: LpType = boost_meta.lp_type;
    use_resource(move || async move {
        let lp_mint_supply = use_gateway().rpc.get_token_supply(&boost_meta.lp_mint).await?;
        match lp_type {
            LpType::Kamino => {
                let strategy_metrics = use_gateway().get_kamino_strategy_metrics(boost_meta.lp_id).await?;
                let token_a = LISTED_TOKENS_BY_TICKER.get(&strategy_metrics.token_a).unwrap();
                let token_b = LISTED_TOKENS_BY_TICKER.get(&strategy_metrics.token_b).unwrap();
                return Ok(LiquidityPair {
                    token_a: token_a.clone(),
                    token_b: token_b.clone(),
                    balance_a_f64: strategy_metrics.vault_balances.token_a.total,
                    balance_b_f64: strategy_metrics.vault_balances.token_b.total,
                    total_value_usd: strategy_metrics.total_value_locked,
                    shares: lp_mint_supply.amount.parse::<u64>().unwrap_or(0),
                });
            }
            LpType::Meteora => {
                let pool_metrics = use_gateway().get_meteora_pool_metrics(boost_meta.lp_id).await?;
                let token_a = LISTED_TOKENS.get(&pool_metrics.pool_token_mints[0]).unwrap();
                let token_b = LISTED_TOKENS.get(&pool_metrics.pool_token_mints[1]).unwrap();
                let balance_a = pool_metrics.pool_token_amounts[0];
                let balance_b = pool_metrics.pool_token_amounts[1];
                return Ok(LiquidityPair {
                    token_a: token_a.clone(),
                    token_b: token_b.clone(),
                    balance_a_f64: balance_a,
                    balance_b_f64: balance_b,
                    total_value_usd: pool_metrics.pool_tvl,
                    shares: lp_mint_supply.amount.parse::<u64>().unwrap_or(0),
                });
            }
        }
    })
}

pub fn use_liquidity_pair_balances(liquidity_pair: Resource<GatewayResult<LiquidityPair>>) -> (Resource<GatewayResult<UiTokenAmount>>, Resource<GatewayResult<UiTokenAmount>>) {
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
