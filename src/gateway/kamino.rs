use steel::Pubkey;
use serde::Deserialize;

use super::{Gateway, GatewayResult, Rpc};
use crate::utils::{deserialize_pubkey, deserialize_string_to_f64};

pub trait KaminoGateway {
    async fn get_strategy_metrics(&self, strategy: Pubkey) -> GatewayResult<KaminoStrategyMetrics>;   
}

impl<R: Rpc> KaminoGateway for Gateway<R> {
    async fn get_strategy_metrics(&self, strategy: Pubkey) -> GatewayResult<KaminoStrategyMetrics> {
        let url = format!("https://api.kamino.finance/strategies/{strategy}/metrics/?env=mainnet-beta&status=LIVE");
        let resp = self.http.get(url).send().await?;
        let metrics = resp.json::<KaminoStrategyMetrics>().await?;
        Ok(metrics)
    }
}

#[derive(Deserialize, Debug)]
pub struct KaminoStrategyMetrics {
    #[serde(deserialize_with = "deserialize_pubkey")]
    pub strategy: Pubkey,
    #[serde(rename = "tokenAMint", deserialize_with = "deserialize_pubkey")]
    pub token_a_mint: Pubkey,
    #[serde(rename = "tokenBMint", deserialize_with = "deserialize_pubkey")]
    pub token_b_mint: Pubkey,
    #[serde(rename = "tokenA")]
    pub token_a: String,
    #[serde(rename = "tokenB")]
    pub token_b: String,
    
    // #[serde(rename = "rewardMints", deserialize_with = "deserialize_pubkey")]
    // pub reward_mints: Vec<Pubkey>,
    // #[serde(rename = "kRewardMints", deserialize_with = "deserialize_pubkey")]
    // pub k_reward_mints: Vec<Pubkey>,

    #[serde(rename = "profitAndLoss")]
    pub profit_and_loss: String,
    #[serde(rename = "sharePrice", deserialize_with = "deserialize_string_to_f64")]
    pub share_price: f64,
    #[serde(rename = "sharesIssued", deserialize_with = "deserialize_string_to_f64")]
    pub shares_issued: f64,
    #[serde(rename = "totalValueLocked", deserialize_with = "deserialize_string_to_f64")]
    pub total_value_locked: f64,

    // TODO APY
    // TODO Kamino APY

    #[serde(rename = "vaultBalances")]
    pub vault_balances: KaminoStrategyVaultBalances,
}

#[derive(Deserialize, Debug)]
pub struct KaminoStrategyVaultBalances {
    #[serde(rename = "tokenA")]
    pub token_a: KaminoStrategyVaultBalancesToken,
    #[serde(rename = "tokenB")]
    pub token_b: KaminoStrategyVaultBalancesToken,
}

#[derive(Deserialize, Debug)]
pub struct KaminoStrategyVaultBalancesToken {
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    pub invested: f64,
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    pub available: f64,
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    pub total: f64,
    // #[serde(rename = "totalUSD", deserialize_with = "deserialize_string_to_f64")]
    // pub total_usd: f64,
}