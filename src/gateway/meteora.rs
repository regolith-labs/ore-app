use steel::{Instruction, Pubkey};
use serde::Deserialize;

use super::{Gateway, GatewayResult, Rpc};
use crate::utils::{deserialize_pubkey_vec, deserialize_string_to_f64, deserialize_string_to_f64_vec};

pub trait MeteoraGateway {
    // Fetch data
    async fn get_meteora_amm_metrics(&self, amm: Pubkey) -> GatewayResult<MeteoraAmmMetrics>;   

    // Instruction builders
    async fn build_meteora_deposit_instruction(&self, amm: Pubkey, amount_a: f64, amount_b: f64, owner: Pubkey) -> GatewayResult<Instruction>;
    async fn build_meteora_withdraw_instruction(&self, amm: Pubkey, shares_amount: u64, owner: Pubkey) -> GatewayResult<Instruction>;
}

impl<R: Rpc> MeteoraGateway for Gateway<R> {
    async fn get_meteora_amm_metrics(&self, amm: Pubkey) -> GatewayResult<MeteoraAmmMetrics> {
        let url = format!("https://app.meteora.ag/amm/pools?address={amm}");
        let resp = self.http.get(url).send().await?;
        let metrics = resp.json::<Vec<MeteoraAmmMetrics>>().await?;
        Ok(metrics.get(0).unwrap().clone())
    }

    async fn build_meteora_deposit_instruction(&self, amm: Pubkey, amount_a: f64, amount_b: f64, owner: Pubkey) -> GatewayResult<Instruction> {
        todo!("Not implemented")
    }

    async fn build_meteora_withdraw_instruction(&self, amm: Pubkey, shares_amount: u64, owner: Pubkey) -> GatewayResult<Instruction> {
        todo!("Not implemented")
    }
}

#[derive(Clone, Deserialize, Debug)]
pub struct MeteoraAmmMetrics {
    // #[serde(deserialize_with = "deserialize_pubkey")]
    // pub pool_address: Pubkey,
    #[serde(deserialize_with = "deserialize_pubkey_vec")]
    pub pool_token_mints: Vec<Pubkey>,
    #[serde(deserialize_with = "deserialize_string_to_f64_vec")]
    pub pool_token_amounts: Vec<f64>,
    // #[serde(deserialize_with = "deserialize_string_to_f64_vec")]
    // pub pool_token_usd_amounts: Vec<f64>,
    // #[serde(deserialize_with = "deserialize_pubkey_vec")]
    // pub vaults: Vec<Pubkey>,
    // #[serde(deserialize_with = "deserialize_pubkey")]
    // pub lp_mint: Pubkey,
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    pub pool_tvl: f64,

    // TODO Rest of the fields
}