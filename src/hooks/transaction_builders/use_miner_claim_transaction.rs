use crate::gateway::Rpc;
use dioxus::prelude::*;
use solana_sdk::transaction::{Transaction, VersionedTransaction};

use crate::{
    gateway::{GatewayError, GatewayResult},
    hooks::{use_gateway, use_wallet, Wallet},
    solana::{
        spl_associated_token_account::{self, get_associated_token_address},
        spl_token,
    },
};

pub fn use_miner_claim_transaction(
    member_on_chain: Resource<GatewayResult<ore_pool_api::state::Member>>,
) -> Resource<GatewayResult<VersionedTransaction>> {
    let wallet = use_wallet();
    use_resource(move || async move {
        // Check if wallet is connected
        let Wallet::Connected(authority) = *wallet.read() else {
            return Err(GatewayError::WalletDisconnected);
        };

        // Destructure member_on_chain
        let data = &*member_on_chain.value().read_unchecked();

        // Get member data from Member
        let member_data = match data {
            Some(Ok(member)) => member, // Extract the Member if successful
            Some(Err(_err)) => return Err(GatewayError::Unknown), // Handle the Err case
            None => return Err(GatewayError::Unknown), // Handle the None case
        };

        // Check if miner has no balance to claim
        if member_data.balance <= 0 {
            return Err(GatewayError::Unknown);
        }

        // Aggregate instructions
        let mut ixs = vec![];
        let gateway = use_gateway();

        // Get the associated token address for miner
        let ata_address = get_associated_token_address(&authority, &ore_api::consts::MINT_ADDRESS);

        // Check if the associated token account exists
        if gateway.rpc.get_token_account(&ata_address).await.is_err() {
            let create_ata_ix =
                spl_associated_token_account::instruction::create_associated_token_account(
                    &authority,
                    &authority,
                    &ore_api::consts::MINT_ADDRESS,
                    &spl_token::ID,
                );
            // Miner does not have ata, so create associated token account
            // Add create ata instruction to the transactions
            ixs.push(create_ata_ix);
        }

        // Use the ata_address directly since it should now exist
        let beneficiary = ata_address;

        // Add claim transaction
        ixs.push(ore_pool_api::sdk::claim(
            authority,
            beneficiary,
            member_data.pool,
            member_data.balance,
        ));

        // Build transaction
        let tx: VersionedTransaction = Transaction::new_with_payer(&ixs, Some(&authority)).into();
        Ok(tx)
    })
}
