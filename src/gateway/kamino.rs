use std::str::FromStr;

use kliquidity_sdk::accounts::{GlobalConfig, WhirlpoolStrategy};
use serde::Deserialize;
use steel::{sysvar, Instruction, Pubkey};

use super::{Gateway, GatewayResult, Rpc};
use crate::solana::{
    spl_associated_token_account::get_associated_token_address, spl_memo, spl_token, spl_token_2022,
};
use crate::{gateway::GatewayError, utils::deserialize_string_to_f64};

pub trait KaminoGateway {
    // Fetch data
    async fn get_kamino_strategy_metrics(
        &self,
        strategy: Pubkey,
    ) -> GatewayResult<KaminoStrategyMetrics>;
    async fn get_kamino_whirlpool_strategy(
        &self,
        strategy: Pubkey,
    ) -> GatewayResult<WhirlpoolStrategy>;
    async fn get_kamino_global_config(&self) -> GatewayResult<GlobalConfig>;

    // Instruction builders
    async fn build_kamino_deposit_instruction(
        &self,
        strategy: Pubkey,
        amount_a: f64,
        amount_b: f64,
        owner: Pubkey,
    ) -> GatewayResult<Instruction>;
    async fn build_kamino_withdraw_instruction(
        &self,
        strategy_address: Pubkey,
        shares_amount: u64,
        owner: Pubkey,
    ) -> GatewayResult<Instruction>;
}

impl<R: Rpc> KaminoGateway for Gateway<R> {
    async fn get_kamino_strategy_metrics(
        &self,
        strategy: Pubkey,
    ) -> GatewayResult<KaminoStrategyMetrics> {
        let url = format!("https://api.kamino.finance/strategies/{strategy}/metrics/?env=mainnet-beta&status=LIVE");
        let resp = self.http.get(url).send().await?;
        let metrics = resp.json::<KaminoStrategyMetrics>().await?;
        Ok(metrics)
    }

    async fn get_kamino_global_config(&self) -> GatewayResult<GlobalConfig> {
        let address = Pubkey::from_str("GKnHiWh3RRrE1zsNzWxRkomymHc374TvJPSTv2wPeYdB").unwrap();
        let account_data = self.rpc.get_account_data(&address).await?;
        let config = GlobalConfig::from_bytes(&account_data)?;
        Ok(config)
    }

    async fn get_kamino_whirlpool_strategy(
        &self,
        strategy: Pubkey,
    ) -> GatewayResult<WhirlpoolStrategy> {
        let account_data = self.rpc.get_account_data(&strategy).await?;
        let strategy = WhirlpoolStrategy::from_bytes(&account_data)?;
        Ok(strategy)
    }

    /// Builds a deposit instruction for a Kamino strategy.
    ///
    /// Logic copied from kliquidity typescript sdk.
    /// https://github.com/Kamino-Finance/kliquidity-sdk/blob/9787fcec784a5a19baede4b6b4819d6883c7e954/src/Kamino.ts#L2910
    async fn build_kamino_deposit_instruction(
        &self,
        strategy_address: Pubkey,
        amount_a: f64,
        amount_b: f64,
        owner: Pubkey,
    ) -> GatewayResult<Instruction> {
        // Check amounts
        if amount_a <= 0.0 || amount_b <= 0.0 {
            return Err(GatewayError::Unknown);
        }

        // Get onchain kamino accounts
        let config = self.get_kamino_global_config().await?;
        let strategy = self.get_kamino_whirlpool_strategy(strategy_address).await?;

        // Get token accounts
        let user_shares_ata = get_associated_token_address(&owner, &strategy.shares_mint);
        let token_a_ata = get_associated_token_address(&owner, &strategy.token_a_mint);
        let token_b_ata = get_associated_token_address(&owner, &strategy.token_b_mint);

        // Convert to base units
        let units_a = (amount_a * 10f64.powf(strategy.token_a_mint_decimals as f64)) as u64;
        let units_b = (amount_b * 10f64.powf(strategy.token_b_mint_decimals as f64)) as u64;

        // Build args
        let args = kliquidity_sdk::instructions::DepositInstructionArgs {
            token_max_a: units_a,
            token_max_b: units_b,
        };
        let accounts = kliquidity_sdk::instructions::Deposit {
            user: owner,
            strategy: strategy_address,
            global_config: strategy.global_config,
            pool: strategy.pool,
            position: strategy.position,
            tick_array_lower: strategy.tick_array_lower,
            tick_array_upper: strategy.tick_array_upper,
            token_a_vault: strategy.token_a_vault,
            token_b_vault: strategy.token_b_vault,
            base_vault_authority: strategy.base_vault_authority,
            token_a_ata,
            token_b_ata,
            token_a_mint: strategy.token_a_mint,
            token_b_mint: strategy.token_b_mint,
            user_shares_ata,
            shares_mint: strategy.shares_mint,
            shares_mint_authority: strategy.shares_mint_authority,
            scope_prices: strategy.scope_prices,
            token_infos: config.token_infos,
            token_program: spl_token::ID,
            token_a_token_program: strategy.token_a_token_program,
            token_b_token_program: strategy.token_b_token_program,
            instruction_sysvar_account: sysvar::instructions::ID,
        };
        Ok(accounts.instruction(args))
    }

    /// Builds a withdraw instruction for a Kamino strategy.
    ///
    /// Logic copied from kliquidity typescript sdk.
    /// https://github.com/Kamino-Finance/kliquidity-sdk/blob/9787fcec784a5a19baede4b6b4819d6883c7e954/src/Kamino.ts#L2550
    async fn build_kamino_withdraw_instruction(
        &self,
        strategy_address: Pubkey,
        shares_amount: u64,
        owner: Pubkey,
    ) -> GatewayResult<Instruction> {
        // Parse amounts
        if shares_amount == 0 {
            return Err(GatewayError::Unknown);
        }

        // Get strategy state
        let strategy = self.get_kamino_whirlpool_strategy(strategy_address).await?;

        // Get treasury pda vaults
        let treasury_fee_token_a_vault = Pubkey::find_program_address(
            &[b"treasury_fee_vault", strategy.token_a_mint.as_ref()],
            &kliquidity_sdk::programs::YVAULTS_ID,
        )
        .0;
        let treasury_fee_token_b_vault = Pubkey::find_program_address(
            &[b"treasury_fee_vault", strategy.token_b_mint.as_ref()],
            &kliquidity_sdk::programs::YVAULTS_ID,
        )
        .0;

        // Get event authority
        let event_authority = kliquidity_sdk::programs::YVAULTS_ID;

        // Get atas
        let user_shares_ata = get_associated_token_address(&owner, &strategy.shares_mint);
        let token_a_ata = get_associated_token_address(&owner, &strategy.token_a_mint);
        let token_b_ata = get_associated_token_address(&owner, &strategy.token_b_mint);

        // Get whirlpool program id
        let whirlpool_program_id =
            Pubkey::from_str("whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc").unwrap();

        // Build instruction
        let args = kliquidity_sdk::instructions::WithdrawInstructionArgs { shares_amount };
        let accounts = kliquidity_sdk::instructions::Withdraw {
            user: owner,
            strategy: strategy_address,
            global_config: strategy.global_config,
            pool: strategy.pool,
            position: strategy.position,
            tick_array_lower: strategy.tick_array_lower,
            tick_array_upper: strategy.tick_array_upper,
            token_a_vault: strategy.token_a_vault,
            token_b_vault: strategy.token_b_vault,
            base_vault_authority: strategy.base_vault_authority,
            pool_token_vault_a: strategy.pool_token_vault_a,
            pool_token_vault_b: strategy.pool_token_vault_b,
            token_a_ata,
            token_b_ata,
            token_a_mint: strategy.token_a_mint,
            token_b_mint: strategy.token_b_mint,
            user_shares_ata,
            shares_mint: strategy.shares_mint,
            treasury_fee_token_a_vault,
            treasury_fee_token_b_vault,
            token_program: spl_token::ID,
            token_program2022: spl_token_2022::ID,
            token_a_token_program: strategy.token_a_token_program,
            token_b_token_program: strategy.token_b_token_program,
            memo_program: spl_memo::ID,
            position_token_account: strategy.position_token_account,
            pool_program: whirlpool_program_id,
            instruction_sysvar_account: sysvar::instructions::ID,
            event_authority: Some(event_authority),
        };
        Ok(accounts.instruction(args))
    }
}

#[derive(Deserialize, Debug)]
pub struct KaminoStrategyMetrics {
    // #[serde(deserialize_with = "deserialize_pubkey")]
    // pub strategy: Pubkey,
    // #[serde(rename = "tokenAMint", deserialize_with = "deserialize_pubkey")]
    // pub token_a_mint: Pubkey,
    // #[serde(rename = "tokenBMint", deserialize_with = "deserialize_pubkey")]
    // pub token_b_mint: Pubkey,
    #[serde(rename = "tokenA")]
    pub token_a: String,
    #[serde(rename = "tokenB")]
    pub token_b: String,

    // #[serde(rename = "rewardMints", deserialize_with = "deserialize_pubkey")]
    // pub reward_mints: Vec<Pubkey>,
    // #[serde(rename = "kRewardMints", deserialize_with = "deserialize_pubkey")]
    // pub k_reward_mints: Vec<Pubkey>,

    // #[serde(rename = "profitAndLoss")]
    // pub profit_and_loss: String,
    // #[serde(rename = "sharePrice", deserialize_with = "deserialize_string_to_f64")]
    // pub share_price: f64,
    // #[serde(rename = "sharesIssued", deserialize_with = "deserialize_string_to_f64")]
    // pub shares_issued: f64,
    #[serde(
        rename = "totalValueLocked",
        deserialize_with = "deserialize_string_to_f64"
    )]
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
    // #[serde(deserialize_with = "deserialize_string_to_f64")]
    // pub invested: f64,
    // #[serde(deserialize_with = "deserialize_string_to_f64")]
    // pub available: f64,
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    pub total: f64,
    // #[serde(rename = "totalUSD", deserialize_with = "deserialize_string_to_f64")]
    // pub total_usd: f64,
}
