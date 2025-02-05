use std::collections::HashMap;

use dioxus::prelude::*;
use ore_boost_api::state::Stake;
use solana_extra_wasm::program::spl_associated_token_account;
use solana_sdk::transaction::{Transaction, VersionedTransaction};
use steel::Pubkey;

use crate::{
    gateway::{GatewayError, GatewayResult}, hooks::{use_wallet, Wallet}
};

pub fn use_boost_claim_all_transaction(
    stake_accounts: HashMap<Pubkey, Resource<GatewayResult<Stake>>>,
) -> Resource<GatewayResult<VersionedTransaction>> {
    let wallet = use_wallet();
    use_resource(move || {
        let stake_accounts = stake_accounts.clone();
        async move {
            // Check if wallet is connected
            let Wallet::Connected(authority) = *wallet.read() else {
                return Err(GatewayError::WalletDisconnected);
            };
        
            // Derive beneficiary
            let beneficiary = spl_associated_token_account::get_associated_token_address(&authority, &ore_api::consts::MINT_ADDRESS);
        
            // Get resources
            let mut ixs = vec![];
            for (pubkey, stake) in stake_accounts.iter() {
                if let Some(Ok(stake)) = stake.cloned() {
                    if stake.rewards > 0 {
                        ixs.push(ore_boost_api::sdk::claim(authority, beneficiary, *pubkey, stake.rewards));
                    }
                }
            }
        
            if ixs.is_empty() {
                return Err(GatewayError::Unknown);
            }
        
            // Build transaction
            let tx = Transaction::new_with_payer(&ixs, Some(&authority)).into();
            Ok(tx)
        }
    })
}
