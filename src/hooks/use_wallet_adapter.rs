use solana_client_wasm::solana_sdk::{
    instruction::Instruction, pubkey::Pubkey, transaction::Transaction,
};
use solana_extra_wasm::program::{
    spl_associated_token_account::instruction::create_associated_token_account, spl_memo, spl_token,
};

use crate::gateway::{ore_token_account_address, GatewayError, GatewayResult};

use super::use_gateway;

pub async fn build_transfer_tx(
    from: &Pubkey,
    to: &Pubkey,
    amount: u64,
    memo: String,
) -> GatewayResult<Transaction> {
    // from token account must exist
    let from_token_account = ore_token_account_address(*from);
    // to token account might exist
    // so pack ix to create account if not
    let to_token_account = ore_token_account_address(*to);
    let maybe_create_to_token_account_ix = build_create_token_account_ix(to, from).await;
    // build ixs
    let memo_ix = spl_memo::build_memo(&memo.into_bytes(), &[from]);
    let transfer_ix = spl_token::instruction::transfer(
        &spl_token::id(), // TODO: maybe ::ID?
        &from_token_account,
        &to_token_account,
        &to_token_account,
        &[from],
        amount,
    )
    .map_err(GatewayError::from)?;
    let ixs = match maybe_create_to_token_account_ix {
        Some(create_token_account_ix) => {
            vec![memo_ix, create_token_account_ix, transfer_ix]
        }
        None => {
            vec![memo_ix, transfer_ix]
        }
    };
    // build transaction
    Ok(Transaction::new_with_payer(ixs.as_slice(), Some(from)))
}

async fn build_create_token_account_ix(owner: &Pubkey, signer: &Pubkey) -> Option<Instruction> {
    let gateway = use_gateway();
    let token_account_address = ore_token_account_address(*owner);
    match gateway.rpc.get_token_account(&token_account_address).await {
        Ok(Some(_)) => None,
        _ => {
            let ix = create_associated_token_account(
                &signer,
                &owner,
                &ore::MINT_ADDRESS,
                &spl_token::id(),
            );
            Some(ix)
        }
    }
}

// pub fn build_upgrade_ix(wallet: &Pubkey) -> Resource<GatewayResult<Instruction>> {
//     let gateway = use_gateway();
//     use_resource(move || {
//         let gateway = gateway.clone();
//         async move {
//             // v2 token account may or may not exist
//             // we'll build an ix to create this token account if needed
//             // for the wallet adapter to sign
//             let v2_token_account_result = gateway.get_token_account_ore_from_pubkey(*wallet).await;
//             // the v1 token account *must* exist
//             // we'll return immediately if not
//             let v1_token_account_result =
//                 gateway.get_token_account_ore_from_pubkey_v1(*wallet).await;
//         }
//     })
// }
