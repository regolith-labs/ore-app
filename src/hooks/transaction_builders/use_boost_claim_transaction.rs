use dioxus::prelude::*;
use ore_boost_api::state::{Boost, Stake};
use solana_sdk::transaction::{Transaction, VersionedTransaction};

use crate::{
    gateway::{GatewayError, GatewayResult},
    hooks::{use_wallet, Wallet},
    solana::spl_associated_token_account,
};

pub fn use_boost_claim_transaction(
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
        let Some(Ok(stake)) = *stake.read() else {
            return Err(GatewayError::Unknown);
        };
        let Some(Ok(boost)) = *boost.read() else {
            return Err(GatewayError::Unknown);
        };

        // Check if stake has rewards to claim
        if stake.rewards == 0 {
            return Err(GatewayError::Unknown);
        }

        // Aggregate instructions
        let mut ixs = vec![];
        let beneficiary = spl_associated_token_account::get_associated_token_address(
            &authority,
            &ore_api::consts::MINT_ADDRESS,
        );
        ixs.push(ore_boost_api::sdk::claim(
            authority,
            beneficiary,
            boost.mint,
            stake.rewards,
        ));

        // Build transaction
        let tx = Transaction::new_with_payer(&ixs, Some(&authority)).into();
        Ok(tx)
    })
}
