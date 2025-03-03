mod use_boost_claim_all_transaction;
mod use_boost_claim_transaction;
mod use_idle_deposit_transaction;
mod use_idle_withdraw_transaction;
mod use_lp_deposit_transaction;
mod use_miner_claim_transaction;
mod use_pair_deposit_transaction;
mod use_pair_withdraw_transaction;
mod use_pool_register_transaction;
mod use_swap_transaction;

pub const MIN_SOL_BALANCE: f64 = 0.1;
pub const APP_FEE: u64 = 5_000;
pub const SOLANA_BASE_FEE: u64 = 5_000;
pub const APP_FEE_ACCOUNT: &str = "tHCCE3KWKx8i8cDjX2DQ3Z7EMJkScAVwkfxdWz8SqgP";
pub const COMPUTE_UNIT_LIMIT: u32 = 500_000;

pub use use_boost_claim_all_transaction::*;
pub use use_boost_claim_transaction::*;
pub use use_idle_deposit_transaction::*;
pub use use_idle_withdraw_transaction::*;
pub use use_lp_deposit_transaction::*;
pub use use_miner_claim_transaction::*;
pub use use_pair_deposit_transaction::*;
pub use use_pair_withdraw_transaction::*;
pub use use_pool_register_transaction::*;
pub use use_swap_transaction::*;
