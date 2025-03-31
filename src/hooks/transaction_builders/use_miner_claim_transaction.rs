use crate::{gateway::Rpc, pages::MemberBalance};
use dioxus::prelude::*;
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction,
    pubkey::Pubkey,
    system_instruction::transfer,
    transaction::{Transaction, VersionedTransaction},
};

use crate::{
    gateway::{GatewayError, GatewayResult},
    hooks::{use_gateway, use_wallet, Wallet, APP_FEE_ACCOUNT, COMPUTE_UNIT_LIMIT},
    solana::{
        spl_associated_token_account::{self, get_associated_token_address},
        spl_token,
    },
};

pub fn use_miner_claim_transaction(
    member_on_chain: Resource<GatewayResult<ore_pool_api::state::Member>>,
    member_claimable_balance: Memo<MemberBalance>,
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

        // Get the member balance
        let member_balance = match *member_claimable_balance.read() {
            MemberBalance::Balance(balance) => balance,
            _ => return Err(GatewayError::Unknown),
        };

        // Check if miner has no balance to claim
        if member_balance <= 0 {
            return Err(GatewayError::Unknown);
        }

        // Aggregate instructions
        let mut ixs = vec![];

        // Set compute unit limit
        ixs.push(ComputeBudgetInstruction::set_compute_unit_limit(
            COMPUTE_UNIT_LIMIT,
        ));

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

        // Add the commit (attribute) instruction
        // TODO:

        // Add claim instruction
        ixs.push(ore_pool_api::sdk::claim(
            authority,
            beneficiary,
            member_data.pool,
            member_data.balance,
        ));

        // Include ORE app fee
        let app_fee_account = Pubkey::from_str_const(APP_FEE_ACCOUNT);
        ixs.push(transfer(&authority, &app_fee_account, 5000));

        // Build initial transaction to estimate priority fee
        let tx = Transaction::new_with_payer(&ixs, Some(&authority)).into();

        // Get priority fee estimate
        let dynamic_priority_fee = match gateway.get_recent_priority_fee_estimate(&tx).await {
            Ok(fee) => fee,
            Err(_) => {
                log::error!("Failed to fetch priority fee estimate");
                return Err(GatewayError::Unknown);
            }
        };

        // Add priority fee instruction
        ixs.insert(
            1,
            ComputeBudgetInstruction::set_compute_unit_price(dynamic_priority_fee),
        );

        // Build final tx with priority fee
        let tx: VersionedTransaction = Transaction::new_with_payer(&ixs, Some(&authority)).into();

        Ok(tx)
    })
}
