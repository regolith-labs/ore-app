use dioxus::prelude::*;
use ore_boost_api::state::{Boost, Stake};
use solana_sdk::transaction::{Transaction, VersionedTransaction};

use crate::{
    gateway::{GatewayError, GatewayResult},
    hooks::{use_wallet, Wallet},
};

pub fn use_lp_deposit_transaction(
    boost: Resource<GatewayResult<Boost>>,
    stake: Resource<GatewayResult<Stake>>,
) -> Resource<GatewayResult<VersionedTransaction>> {
    let wallet = use_wallet();
    use_resource(move || async move {
        // Check if wallet is connected
        let Wallet::Connected(authority) = *wallet.read() else {
            return Err(GatewayError::WalletDisconnected);
        };

        // Get resources
        let Some(Ok(boost)) = *boost.read() else {
            return Err(GatewayError::Unknown);
        };

        // Aggregate instructions
        let mut ixs = vec![];

        // Open the stake account, if needed
        if let Some(Ok(_stake)) = stake.read().as_ref() {
            // Do nothing
        } else {
            ixs.push(ore_boost_api::sdk::open(authority, authority, boost.mint));
        }

        // Deposit LP tokens
        ixs.push(ore_boost_api::sdk::deposit(authority, boost.mint, u64::MAX));

        // Build transaction
        let tx = Transaction::new_with_payer(&ixs, Some(&authority)).into();
        Ok(tx)
    })
}
