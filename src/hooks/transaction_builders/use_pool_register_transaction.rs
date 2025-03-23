use crate::{
    gateway::{GatewayError, GatewayResult},
    hooks::{
        use_gateway, use_member_resource_deprecated, use_pool, use_pool_deprecated, use_wallet,
        GetPubkey, APP_FEE, APP_FEE_ACCOUNT, COMPUTE_UNIT_LIMIT,
    },
};
use dioxus::prelude::*;
use ore_api::consts::MINT_ADDRESS;
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction,
    pubkey::Pubkey,
    system_instruction::transfer,
    transaction::{Transaction, VersionedTransaction},
};

pub fn use_pool_register_transaction() -> Resource<GatewayResult<VersionedTransaction>> {
    let pool = use_pool();
    let pool_deprecated = use_pool_deprecated();
    let member_deprecated = use_member_resource_deprecated();
    let wallet = use_wallet();
    use_resource(move || async move {
        if let (Some(pool), Some(pool_deprecated)) = (pool.cloned(), pool_deprecated.cloned()) {
            let pubkey = wallet.pubkey()?;
            // Aggregate instructions
            let mut ixs = vec![];

            // Set compute unit limit
            ixs.push(ComputeBudgetInstruction::set_compute_unit_limit(
                COMPUTE_UNIT_LIMIT,
            ));

            // Set priority fee
            ixs.push(ComputeBudgetInstruction::set_compute_unit_price(10_000));

            // Check for deprecated member account
            if let Some(Ok(m)) = member_deprecated.cloned() {
                let ata = crate::solana::spl_associated_token_account::get_associated_token_address(
                    &pubkey,
                    &MINT_ADDRESS,
                );
                let claim = ore_pool_api::sdk::claim(
                    pubkey,
                    ata,
                    pool_deprecated.address,
                    member_balance_deprecated,
                );
                ixs.push(claim);
            }

            // Build join instruction
            let join_ix = ore_pool_api::sdk::join(pubkey, pool.address, pubkey);
            ixs.push(join_ix);

            // Include ORE app fee
            let app_fee_account = Pubkey::from_str_const(APP_FEE_ACCOUNT);
            ixs.push(transfer(&pubkey, &app_fee_account, APP_FEE));

            // Build initial transaction to estimate priority fee
            let tx = Transaction::new_with_payer(&ixs, Some(&pubkey)).into();

            // Get priority fee estimate
            let gateway = use_gateway();
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

            // Build transaction with priority fee
            let tx_with_priority_fee = Transaction::new_with_payer(&ixs, Some(&pubkey)).into();

            Ok(tx_with_priority_fee)
        } else {
            Err(GatewayError::AccountNotFound)
        }
    })
}
