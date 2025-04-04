use dioxus::prelude::*;
use ore_boost_api::state::{Boost, Stake};
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction,
    pubkey::Pubkey,
    system_instruction::transfer,
    transaction::{Transaction, VersionedTransaction},
};

use crate::{
    gateway::{GatewayError, GatewayResult},
    hooks::{use_wallet, Wallet, APP_FEE_ACCOUNT, COMPUTE_UNIT_LIMIT},
};

#[cfg(not(feature = "web"))]
use super::tip_ix;

pub fn use_lp_deposit_transaction(
    boost: Signal<GatewayResult<Boost>>,
    stake: Signal<GatewayResult<Stake>>,
) -> Resource<GatewayResult<VersionedTransaction>> {
    let wallet = use_wallet();
    use_resource(move || async move {
        // Check if wallet is connected
        let Wallet::Connected(authority) = *wallet.read() else {
            return Err(GatewayError::WalletDisconnected);
        };

        // Get resources
        let Ok(boost) = *boost.read() else {
            return Err(GatewayError::Unknown);
        };

        // Aggregate instructions
        let mut ixs = vec![];

        // Set compute unit limit
        ixs.push(ComputeBudgetInstruction::set_compute_unit_limit(
            COMPUTE_UNIT_LIMIT,
        ));

        // Open the stake account, if needed
        if let Ok(_) = stake.read().as_ref() {
            // Do nothing
        } else {
            ixs.push(ore_boost_api::sdk::open(authority, authority, boost.mint));
        }

        // Include ORE app fee
        let app_fee_account = Pubkey::from_str_const(APP_FEE_ACCOUNT);
        ixs.push(transfer(&authority, &app_fee_account, 5000));

        // Deposit LP tokens
        ixs.push(ore_boost_api::sdk::deposit(authority, boost.mint, u64::MAX));

        #[cfg(not(feature = "web"))]
        // Add jito tip
        ixs.push(tip_ix(&authority));

        // Build tx
        let tx = Transaction::new_with_payer(&ixs, Some(&authority)).into();

        Ok(tx)
    })
}
