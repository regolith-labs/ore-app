mod use_boost_claim_all_transaction;
mod use_boost_claim_transaction;
mod use_idle_deposit_transaction;
mod use_idle_withdraw_transaction;
mod use_lp_deposit_transaction;
mod use_pair_deposit_transaction;
mod use_pair_withdraw_transaction;
mod use_pool_commit_claim_transaction;
mod use_pool_register_transaction;
mod use_swap_transaction;
#[cfg(feature = "web")]
mod use_topup_transaction;
mod use_transfer_transaction;

pub use use_boost_claim_all_transaction::*;
pub use use_boost_claim_transaction::*;
pub use use_idle_deposit_transaction::*;
pub use use_idle_withdraw_transaction::*;
pub use use_lp_deposit_transaction::*;
pub use use_pair_deposit_transaction::*;
pub use use_pair_withdraw_transaction::*;
pub use use_pool_commit_claim_transaction::*;
pub use use_pool_register_transaction::*;
pub use use_swap_transaction::*;
#[cfg(feature = "web")]
pub use use_topup_transaction::*;
pub use use_transfer_transaction::*;

pub const APP_FEE: u64 = 5_000;
pub const APP_FEE_ACCOUNT: &str = "tHCCE3KWKx8i8cDjX2DQ3Z7EMJkScAVwkfxdWz8SqgP";
pub const SOLANA_BASE_FEE: u64 = 5_000;
pub const COMPUTE_UNIT_LIMIT: u32 = 500_000;
pub const MIN_SOL_BALANCE: f64 = 0.1;
pub const JITO_TIP_AMOUNT: u64 = 2_000;

#[cfg(not(feature = "web"))]
use solana_sdk::instruction::Instruction;
#[cfg(not(feature = "web"))]
use solana_sdk::pubkey::Pubkey;

#[cfg(not(feature = "web"))]
pub fn tip_ix(signer: &Pubkey) -> Instruction {
    let address = get_jito_tip_address();
    solana_sdk::system_instruction::transfer(signer, &address, JITO_TIP_AMOUNT)
}
#[cfg(not(feature = "web"))]
fn get_jito_tip_address() -> Pubkey {
    let addresses = [
        solana_sdk::pubkey!("96gYZGLnJYVFmbjzopPSU6QiEV5fGqZNyN9nmNhvrZU5"),
        solana_sdk::pubkey!("HFqU5x63VTqvQss8hp11i4wVV8bD44PvwucfZ2bU7gRe"),
        solana_sdk::pubkey!("Cw8CFyM9FkoMi7K7Crf6HNQqf4uEMzpKw6QNghXLvLkY"),
        solana_sdk::pubkey!("ADaUMid9yfUytqMBgopwjb2DTLSokTSzL1zt6iGPaS49"),
        solana_sdk::pubkey!("DfXygSm4jCyNCybVYYK6DwvWqjKee8pbDmJGcLWNDXjh"),
        solana_sdk::pubkey!("ADuUkR4vqLUMWXxW9gh6D6L8pMSawimctcNZ5pGwDcEt"),
        solana_sdk::pubkey!("DttWaMuVvTiduZRnguLF7jNxTgiMBZ1hyAumKUiL2KRL"),
        solana_sdk::pubkey!("3AVi9Tg9Uo68tJfuvoKvqKNWKkC5wPdSSdeBnizKZ6jT"),
    ];

    let random_index = rand::random::<usize>() % addresses.len();
    addresses[random_index]
}
