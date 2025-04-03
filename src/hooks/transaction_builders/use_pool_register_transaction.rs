use crate::{
    gateway::{GatewayError, GatewayResult, Rpc},
    hooks::{
        use_gateway, use_member_resource_deprecated, use_pool, use_pool_deprecated, use_wallet,
        GetPubkey, APP_FEE, APP_FEE_ACCOUNT, COMPUTE_UNIT_LIMIT,
    },
    solana::spl_associated_token_account,
};
use dioxus::prelude::*;
use ore_api::consts::MINT_ADDRESS;
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction,
    pubkey::Pubkey,
    system_instruction::transfer,
    transaction::{Transaction, VersionedTransaction},
};

#[cfg(not(feature = "web"))]
use super::tip_ix;

pub fn use_pool_register_transaction() -> Resource<GatewayResult<VersionedTransaction>> {
    // wallet
    let wallet = use_wallet();
    // pool and deprecated pool
    let pool = use_pool();
    let pool_deprecated = use_pool_deprecated();
    // deprecated member account
    let member_deprecated = use_member_resource_deprecated();
    use_resource(move || async move {
        if let (Some(pool), Some(pool_deprecated)) = (pool.cloned(), pool_deprecated.cloned()) {
            let pubkey = wallet.pubkey()?;
            // aggregate instructions
            let mut ixs = vec![];
            // set compute unit limit
            ixs.push(ComputeBudgetInstruction::set_compute_unit_limit(
                COMPUTE_UNIT_LIMIT,
            ));

            // if deprecated member exists
            if let Some(Ok(member_deprecated)) = member_deprecated.cloned() {
                // claim from deprecated pool
                let claim_ata = spl_associated_token_account::get_associated_token_address(
                    &pubkey,
                    &MINT_ADDRESS,
                );
                // check that beneficiary token account exists o.w. create
                let gateway = use_gateway();
                let claim_ata_data = gateway.rpc.get_account_data(&claim_ata).await;
                if let Err(_err) = claim_ata_data {
                    let create_ata =
                        spl_associated_token_account::instruction::create_associated_token_account(
                            &pubkey,
                            &pubkey,
                            &MINT_ADDRESS,
                            &crate::solana::spl_token::ID,
                        );
                    ixs.push(create_ata);
                };
                // build claim instruction from deprecated pool
                let claim_amount = member_deprecated.balance;
                let claim_ix = ore_pool_api::sdk::claim(
                    pubkey,
                    claim_ata,
                    pool_deprecated.address,
                    claim_amount,
                );
                ixs.push(claim_ix);
            }

            // build join instruction
            let join_ix = ore_pool_api::sdk::join(pubkey, pool.address, pubkey);
            ixs.push(join_ix);

            // include app fee
            let app_fee_account = Pubkey::from_str_const(APP_FEE_ACCOUNT);
            ixs.push(transfer(&pubkey, &app_fee_account, APP_FEE));

            #[cfg(not(feature = "web"))]
            // Add jito tip
            ixs.push(tip_ix(&pubkey));

            // build transaction with priority fee
            let tx_with_priority_fee = Transaction::new_with_payer(&ixs, Some(&pubkey)).into();
            Ok(tx_with_priority_fee)
        } else {
            Err(GatewayError::AccountNotFound)
        }
    })
}
