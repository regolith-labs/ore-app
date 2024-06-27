use solana_client_wasm::solana_sdk::compute_budget::ComputeBudgetInstruction;
use solana_client_wasm::solana_sdk::{
    instruction::Instruction, pubkey::Pubkey, transaction::Transaction,
};
use solana_extra_wasm::account_decoder::parse_token::UiTokenAmount;
use solana_extra_wasm::program::{
    spl_associated_token_account::instruction::create_associated_token_account, spl_token,
};

use crate::gateway::{ore_token_account_address, GatewayResult, CU_LIMIT_UPGRADE};
use crate::hooks::use_gateway;

#[derive(Clone, Copy)]
pub struct WalletAdapter {
    pub pubkey: Pubkey,
}

impl WalletAdapter {
    pub async fn build_upgrade_tx(&self, amount: u64) -> GatewayResult<Transaction> {
        let gateway = use_gateway();
        // v2 token account may or may not exist
        // we'll build an ix to create this token account if needed
        // for the wallet adapter to sign
        let v2_token_account_result = gateway.get_token_account_ore_from_pubkey(self.pubkey).await;
        // the v1 token account *must* exist
        // return immediately if not
        let v1_token_account = gateway
            .get_token_account_ore_from_pubkey_v1(self.pubkey)
            .await?;
        // build upgrade ix
        let build_upgrade_ore_ix = |v2_token_account_address: &Pubkey| -> Instruction {
            ore::instruction::upgrade(
                self.pubkey,
                *v2_token_account_address,
                v1_token_account,
                amount,
            )
        };
        // build ixs
        let ixs = match v2_token_account_result {
            // v2 token account exists
            Ok(token_account_address) => {
                // compute limit ix
                let cu_limit_ix =
                    ComputeBudgetInstruction::set_compute_unit_limit(CU_LIMIT_UPGRADE);
                // upgrade ix
                let upgrade_ix = build_upgrade_ore_ix(&token_account_address);
                // pack ixs
                vec![cu_limit_ix, upgrade_ix]
            }
            Err(_) => {
                // compute limit ix
                // TODO: exact amount for creating token account
                let cu_limit_ix = ComputeBudgetInstruction::set_compute_unit_limit(1_400_000);
                // create token account ix
                let create_token_account_ix = create_associated_token_account(
                    &self.pubkey,
                    &self.pubkey,
                    &ore::MINT_ADDRESS,
                    &spl_token::id(),
                );
                // upgrade ix
                let upgrade_ix = build_upgrade_ore_ix(&ore_token_account_address(self.pubkey));
                // pack ixs
                vec![cu_limit_ix, create_token_account_ix, upgrade_ix]
            }
        };
        Ok(Transaction::new_with_payer(
            ixs.as_slice(),
            Some(&self.pubkey),
        ))
    }

    // async fn _build_transfer_tx(
    //     &self,
    //     to: &Pubkey,
    //     amount: u64,
    //     memo: String,
    // ) -> GatewayResult<Transaction> {
    //     // from token account must exist
    //     let from_token_account = ore_token_account_address(self.pubkey);
    //     // to token account might exist
    //     // so pack ix to create account if not
    //     let to_token_account = ore_token_account_address(*to);
    //     let maybe_create_to_token_account_ix = self.build_create_token_account_ix(to).await;
    //     // build ixs
    //     let memo_ix = spl_memo::build_memo(&memo.into_bytes(), &[&self.pubkey]);
    //     let transfer_ix = spl_token::instruction::transfer(
    //         &spl_token::id(),
    //         &from_token_account,
    //         &to_token_account,
    //         &to_token_account,
    //         &[&self.pubkey],
    //         amount,
    //     )
    //     .map_err(GatewayError::from)?;
    //     let ixs = match maybe_create_to_token_account_ix {
    //         Some(create_token_account_ix) => {
    //             vec![memo_ix, create_token_account_ix, transfer_ix]
    //         }
    //         None => {
    //             vec![memo_ix, transfer_ix]
    //         }
    //     };
    //     // build transaction
    //     Ok(Transaction::new_with_payer(
    //         ixs.as_slice(),
    //         Some(&self.pubkey),
    //     ))
    // }

    // async fn build_create_token_account_ix(&self, owner: &Pubkey) -> Option<Instruction> {
    //     let gateway = use_gateway();
    //     let token_account_address = ore_token_account_address(*owner);
    //     match gateway.rpc.get_token_account(&token_account_address).await {
    //         Ok(Some(_)) => None,
    //         _ => {
    //             let ix = create_associated_token_account(
    //                 &self.pubkey,
    //                 &owner,
    //                 &ore::MINT_ADDRESS,
    //                 &spl_token::id(),
    //             );
    //             Some(ix)
    //         }
    //     }
    // }
}
