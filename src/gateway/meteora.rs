use meteora_pools_sdk::accounts::Pool as MeteoraPool;
use solana_extra_wasm::program::{spl_associated_token_account::get_associated_token_address, spl_token};
use steel::{Instruction, Pubkey};
use serde::Deserialize;

use super::{Gateway, GatewayResult, Rpc};
use crate::utils::{deserialize_pubkey_vec, deserialize_string_to_f64, deserialize_string_to_f64_vec};

pub trait MeteoraGateway {
    // Fetch data
    async fn get_meteora_pool_metrics(&self, address: Pubkey) -> GatewayResult<MeteoraPoolMetrics>;   
    async fn get_meteora_pool(&self, address: Pubkey) -> GatewayResult<MeteoraPool>;

    // Instruction builders
    async fn build_meteora_deposit_instruction(&self, pool_address: Pubkey, amount_a: f64, amount_b: f64, owner: Pubkey) -> GatewayResult<Instruction>;
    async fn build_meteora_withdraw_instruction(&self, pool_address: Pubkey, shares_amount: u64, owner: Pubkey) -> GatewayResult<Instruction>;
}

impl<R: Rpc> MeteoraGateway for Gateway<R> {
    async fn get_meteora_pool_metrics(&self, address: Pubkey) -> GatewayResult<MeteoraPoolMetrics> {
        let url = format!("https://app.meteora.ag/amm/pools?address={address}");
        let resp = self.http.get(url).send().await?;
        let metrics = resp.json::<Vec<MeteoraPoolMetrics>>().await?;
        Ok(metrics.get(0).unwrap().clone())
    }

    async fn get_meteora_pool(&self, address: Pubkey) -> GatewayResult<MeteoraPool> {
        let account_data = self.rpc.get_account_data(&address).await?;
        let pool = MeteoraPool::from_bytes(&account_data)?;
        Ok(pool)
    }

    async fn build_meteora_deposit_instruction(&self, pool_address: Pubkey, amount_a: f64, amount_b: f64, owner: Pubkey) -> GatewayResult<Instruction> {

        // TODO Generate sdk for meteora vault program

        // Get pool
        let pool = self.get_meteora_pool(pool_address).await?;      

        let user_pool_lp = get_associated_token_address(&owner, &pool.lp_mint);

        let args: meteora_pools_sdk::instructions::AddBalanceLiquidityInstructionArgs = meteora_pools_sdk::instructions::AddBalanceLiquidityInstructionArgs {
            pool_token_amount: todo!(),
            maximum_token_a_amount: todo!(),
            maximum_token_b_amount: todo!(),
        };
        let accounts = meteora_pools_sdk::instructions::AddBalanceLiquidity {
            pool: pool_address,
            lp_mint: pool.lp_mint,
            user_pool_lp: user_pool_lp,
            a_vault_lp: pool.a_vault_lp,
            b_vault_lp: pool.b_vault_lp,
            a_vault: pool.a_vault,
            b_vault: pool.b_vault,
            a_vault_lp_mint: todo!(), // TODO Get vault data
            b_vault_lp_mint: todo!(), // TODO Get vault data
            a_token_vault: todo!(), // TODO Get vault data
            b_token_vault: todo!(), // TODO Get vault data
            user_a_token: get_associated_token_address(&owner, &pool.token_a_mint),
            user_b_token: get_associated_token_address(&owner, &pool.token_b_mint),
            user: owner,
            vault_program: meteora_pools_sdk::programs::AMM_ID, // TODO Vault program
            token_program: spl_token::ID,
        };
        Ok(accounts.instruction(args))
    }

    async fn build_meteora_withdraw_instruction(&self, pool_address: Pubkey, shares_amount: u64, owner: Pubkey) -> GatewayResult<Instruction> {
        todo!("Not implemented")
    }
}

#[derive(Clone, Deserialize, Debug)]
pub struct MeteoraPoolMetrics {
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