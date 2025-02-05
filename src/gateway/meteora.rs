use meteora_pools_sdk::accounts::Pool as MeteoraPool;
use meteora_vault_sdk::accounts::Vault as MeteoraVault;
use meteora_vault_sdk::types::LockedProfitTracker;
use solana_extra_wasm::program::{spl_associated_token_account::get_associated_token_address, spl_token};
use steel::{Instruction, Pubkey};
use serde::Deserialize;

use super::{spl::SplGateway, solana::SolanaGateway, Gateway, GatewayResult, Rpc};
use crate::{gateway::GatewayError, utils::{deserialize_pubkey_vec, deserialize_string_to_f64, deserialize_string_to_f64_vec}};

pub trait MeteoraGateway {
    // Fetch data
    async fn get_meteora_pool_metrics(&self, address: Pubkey) -> GatewayResult<MeteoraPoolMetrics>;   
    async fn get_meteora_pool(&self, address: Pubkey) -> GatewayResult<MeteoraPool>;
    async fn get_meteora_vault(&self, address: Pubkey) -> GatewayResult<MeteoraVault>;
    async fn get_meteora_pool_token_amount(&self, pool: MeteoraPool, vault_a: MeteoraVault, vault_b: MeteoraVault, max_amount_a: u64, max_amount_b: u64, slippage_rate: u64) -> GatewayResult<u64>;

    // Instruction builders
    async fn build_meteora_deposit_instruction(&self, pool_address: Pubkey, max_amount_a: u64, max_amount_b: u64, slippage_rate: u64, owner: Pubkey) -> GatewayResult<Instruction>;
    async fn build_meteora_withdraw_instruction(&self, pool_address: Pubkey, shares_amount: u64, amount_a: u64, amount_b: u64, slippage_rate: u64, owner: Pubkey) -> GatewayResult<Instruction>;
}

impl<R: Rpc + SplGateway + SolanaGateway> MeteoraGateway for Gateway<R> {
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

    async fn get_meteora_vault(&self, address: Pubkey) -> GatewayResult<MeteoraVault> {
        let account_data = self.rpc.get_account_data(&address).await?;
        let vault = MeteoraVault::from_bytes(&account_data)?;
        Ok(vault)
    }

    async fn get_meteora_pool_token_amount(&self, pool: MeteoraPool, vault_a: MeteoraVault, vault_b: MeteoraVault, max_amount_a: u64, max_amount_b: u64, slippage_rate: u64) -> GatewayResult<u64> {
        // Get token accounts
        let Some(pool_vault_a_lp_token) = self.rpc.get_token_account(&pool.a_vault_lp).await? else {
            return Err(GatewayError::Unknown);
        };
        let Some(pool_vault_b_lp_token) = self.rpc.get_token_account(&pool.b_vault_lp).await? else {
            return Err(GatewayError::Unknown);
        };

        // Get mints
        let pool_lp_mint = self.rpc.get_mint(&pool.lp_mint).await?;
        let vault_a_lp_mint = self.rpc.get_mint(&vault_a.lp_mint).await?;
        let vault_b_lp_mint = self.rpc.get_mint(&vault_b.lp_mint).await?;

        // Get current time
        let clock = self.rpc.get_clock().await?;
        let current_time = clock.unix_timestamp;

        // Get pool lp token balances
        let pool_vault_a_lp_token_amount_u64 = pool_vault_a_lp_token.amount.parse::<u64>().unwrap();
        let pool_vault_b_lp_token_amount_u64 = pool_vault_b_lp_token.amount.parse::<u64>().unwrap();

        // Get vault token amount
        let Some(pool_token_a_amount) = vault_a.get_amount_by_share(
            current_time as u64, 
            pool_vault_a_lp_token_amount_u64, 
            vault_a_lp_mint.supply
        ) else {
            return Err(GatewayError::Unknown);
        };
        let Some(pool_token_b_amount) = vault_b.get_amount_by_share(
            current_time as u64, 
            pool_vault_b_lp_token_amount_u64, 
            vault_b_lp_mint.supply
        ) else {
            return Err(GatewayError::Unknown);
        };

        // Calculate pool token amount
        let pool_token_by_a: u128 = (max_amount_a as u128)
            .checked_mul(pool_lp_mint.supply.into())
            .unwrap()
            .checked_div(pool_token_a_amount.into())
            .unwrap();
        let pool_token_by_b: u128 = (max_amount_b as u128)
            .checked_mul(pool_lp_mint.supply.into())
            .unwrap()
            .checked_div(pool_token_b_amount.into())
            .unwrap();
        let pool_token_amount = pool_token_by_a
            .min(pool_token_by_b)
            .checked_mul(100u128.checked_sub(slippage_rate as u128).unwrap())
            .unwrap()
            .checked_div(100)
            .unwrap();

        Ok(pool_token_amount as u64)
    }

    async fn build_meteora_deposit_instruction(&self, pool_address: Pubkey, max_amount_a: u64, max_amount_b: u64, slippage_rate: u64, owner: Pubkey) -> GatewayResult<Instruction> {
        // Get pool and vault data
        let pool = self.get_meteora_pool(pool_address).await?;      
        let vault_a = self.get_meteora_vault(pool.a_vault).await?;
        let vault_b = self.get_meteora_vault(pool.b_vault).await?;

        // Get token amount
        let pool_token_amount = self.get_meteora_pool_token_amount(
            pool.clone(),
            vault_a.clone(),
            vault_b.clone(),
            max_amount_a,
            max_amount_b,
            slippage_rate
        ).await?;

        // Derive token addresses
        let user_pool_lp = get_associated_token_address(&owner, &pool.lp_mint);
        
        // Build instruction
        let args: meteora_pools_sdk::instructions::AddBalanceLiquidityInstructionArgs = meteora_pools_sdk::instructions::AddBalanceLiquidityInstructionArgs {
            pool_token_amount,
            maximum_token_a_amount: max_amount_a,
            maximum_token_b_amount: max_amount_b,
        };
        let accounts = meteora_pools_sdk::instructions::AddBalanceLiquidity {
            pool: pool_address,
            lp_mint: pool.lp_mint,
            user_pool_lp,
            a_vault_lp: pool.a_vault_lp,
            b_vault_lp: pool.b_vault_lp,
            a_vault: pool.a_vault,
            b_vault: pool.b_vault,
            a_vault_lp_mint: vault_a.lp_mint,
            b_vault_lp_mint: vault_b.lp_mint,
            a_token_vault: vault_a.token_vault,
            b_token_vault: vault_b.token_vault,
            user_a_token: get_associated_token_address(&owner, &pool.token_a_mint),
            user_b_token: get_associated_token_address(&owner, &pool.token_b_mint),
            user: owner,
            vault_program: meteora_vault_sdk::programs::VAULT_ID,
            token_program: spl_token::ID,
        };
        Ok(accounts.instruction(args))
    }

    async fn build_meteora_withdraw_instruction(&self, pool_address: Pubkey, shares_amount: u64, _amount_a: u64, amount_b: u64, slippage_rate: u64, owner: Pubkey) -> GatewayResult<Instruction> {
        // Get pool and vault data
        let pool = self.get_meteora_pool(pool_address).await?;
        let vault_a = self.get_meteora_vault(pool.a_vault).await?;
        let vault_b = self.get_meteora_vault(pool.b_vault).await?;

        // Get min amounts
        let pool_token_amount = shares_amount;
        let min_amount_a = (amount_b as u128)
            .checked_mul((1000 - slippage_rate) as u128)
            .unwrap()
            .checked_div(1000)
            .unwrap() as u64;
        let min_amount_b = (amount_b as u128)
            .checked_mul((1000 - slippage_rate) as u128)
            .unwrap()
            .checked_div(1000)
            .unwrap() as u64;

        // Build instruction
        let args = meteora_pools_sdk::instructions::RemoveBalanceLiquidityInstructionArgs {
            pool_token_amount,
            minimum_a_token_out: min_amount_a,
            minimum_b_token_out: min_amount_b,
        };
        let accounts = meteora_pools_sdk::instructions::RemoveBalanceLiquidity {
            pool: pool_address,
            lp_mint: pool.lp_mint,
            user_pool_lp: get_associated_token_address(&owner, &pool.lp_mint),
            a_vault_lp: pool.a_vault_lp,
            b_vault_lp: pool.b_vault_lp,
            a_vault: pool.a_vault,
            b_vault: pool.b_vault,
            a_vault_lp_mint: vault_a.lp_mint,
            b_vault_lp_mint: vault_b.lp_mint,
            a_token_vault: vault_a.token_vault,
            b_token_vault: vault_b.token_vault,
            user_a_token: get_associated_token_address(&owner, &pool.token_a_mint),
            user_b_token: get_associated_token_address(&owner, &pool.token_b_mint),
            user: owner,
            vault_program: meteora_vault_sdk::programs::VAULT_ID,
            token_program: spl_token::ID,
        };
        Ok(accounts.instruction(args))
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

trait MeteoraVaultExt {
    fn get_amount_by_share(&self, current_time: u64, share: u64, total_supply: u64) -> Option<u64>;
    fn get_unlocked_amount(&self, current_time: u64) -> Option<u64>;
}

impl MeteoraVaultExt for MeteoraVault {
    /// Get amount by share
    fn get_amount_by_share(
        &self,
        current_time: u64,
        share: u64,
        total_supply: u64,
    ) -> Option<u64> {
        let total_amount = self.get_unlocked_amount(current_time)?;
        u64::try_from(
            u128::from(share)
                .checked_mul(u128::from(total_amount))?
                .checked_div(u128::from(total_supply))?,
        )
        .ok()
    }
    /// Get unlocked amount of vault
    fn get_unlocked_amount(&self, current_time: u64) -> Option<u64> {
        self.total_amount.checked_sub(
            self.locked_profit_tracker
                .calculate_locked_profit(current_time)?,
        )
    }
}

/// DENOMINATOR of degradation
pub const LOCKED_PROFIT_DEGRADATION_DENOMINATOR: u128 = 1_000_000_000_000;

trait LockedProfitTrackerExt {
    fn calculate_locked_profit(&self, current_time: u64) -> Option<u64>;
}

impl LockedProfitTrackerExt for LockedProfitTracker {
    /// Calculate locked profit, based from Yearn `https://github.com/yearn/yearn-vaults/blob/main/contracts/Vault.vy#L825`
    fn calculate_locked_profit(&self, current_time: u64) -> Option<u64> {
        let duration = u128::from(current_time.checked_sub(self.last_report)?);
        let locked_profit_degradation = u128::from(self.locked_profit_degradation);
        let locked_fund_ratio = duration.checked_mul(locked_profit_degradation)?;

        if locked_fund_ratio > LOCKED_PROFIT_DEGRADATION_DENOMINATOR {
            return Some(0);
        }
        let locked_profit = u128::from(self.last_updated_locked_profit);

        let locked_profit = (locked_profit
            .checked_mul(LOCKED_PROFIT_DEGRADATION_DENOMINATOR - locked_fund_ratio)?)
        .checked_div(LOCKED_PROFIT_DEGRADATION_DENOMINATOR)?;
        let locked_profit = u64::try_from(locked_profit).ok()?;
        Some(locked_profit)
    }
}