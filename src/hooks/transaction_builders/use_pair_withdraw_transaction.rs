use dioxus::prelude::*;
use ore_boost_api::state::Stake;
use solana_extra_wasm::program::{spl_associated_token_account::{get_associated_token_address, instruction::create_associated_token_account_idempotent}, spl_token::{self, instruction::{close_account, sync_native}}};
use solana_sdk::{hash::Hash, message::{v0, VersionedMessage}, transaction::{Transaction, VersionedTransaction}};

use crate::{
    components::TokenInputError, config::{BoostMeta, LpType}, gateway::{GatewayError, kamino::KaminoGateway, meteora::MeteoraGateway, GatewayResult}, hooks::{use_gateway, use_wallet, BoostDeposits, Wallet}
};

// Build pair deposit transaction
pub fn use_pair_withdraw_transaction(
    boost_meta: BoostMeta,
    boost_deposits: Resource<GatewayResult<BoostDeposits>>,
    stake: Resource<GatewayResult<Stake>>,
    input_amount_a: Signal<String>,
    input_amount_b: Signal<String>,
    mut err: Signal<Option<TokenInputError>>
) -> Resource<GatewayResult<VersionedTransaction>> {
    let wallet = use_wallet();
    use_resource(move || async move {
        // Check if wallet is connected
        let Wallet::Connected(authority) = *wallet.read() else {
            err.set(None);
            return Err(GatewayError::WalletDisconnected);
        };

        // Get resources
        let Some(Ok(boost_deposits)) = boost_deposits.cloned() else {
            err.set(None);
            return Err(GatewayError::Unknown);
        };
    
        // TODO Convert input amounts to LP shares
        let shares_amount = 0;
       
        // Aggregate instructions
        let mut ixs = vec![];

        // Build ore boost withdraw instruction
        ixs.push(
            ore_boost_api::sdk::withdraw(
                authority,
                boost_meta.lp_mint,
                shares_amount,
            )
        );
    
        // Build sol ata
        let token_a_ata = get_associated_token_address(&authority, &boost_meta.pair_mint);
        let is_sol = boost_deposits.token_a.ticker == "SOL";
        if is_sol {
            ixs.push(
                create_associated_token_account_idempotent(&authority, &authority, &boost_meta.pair_mint, &spl_token::ID)
            );
            ixs.push(
                sync_native(&spl_token::ID, &token_a_ata).unwrap()
            );
        }

        // Append kamino withdraw instructions
        let withdraw_ix = match boost_meta.lp_type {
            LpType::Kamino => {
                let Ok(ix) = use_gateway().build_kamino_withdraw_instruction(
                    boost_meta.lp_id,
                    shares_amount,
                    authority,
                ).await else {
                    err.set(None);
                    return Err(GatewayError::Unknown);
                };
                ix
            }
            LpType::Meteora => {
                let Ok(ix) = use_gateway().build_meteora_withdraw_instruction(
                    boost_meta.lp_id,
                    shares_amount,
                    authority,
                ).await else {
                    err.set(None);
                    return Err(GatewayError::Unknown);
                };
                ix
            }
        };
        ixs.push(withdraw_ix);

        // Close the wSOL ata
        if is_sol {
            ixs.push(
                close_account(&spl_token::ID, &token_a_ata, &authority, &authority, &[&authority]).unwrap()
            );
        }

        // Send instructions
        let _tx_legacy = Transaction::new_with_payer(&ixs, Some(&authority));
        let tx = VersionedTransaction {
            signatures: vec![],
            message: VersionedMessage::V0(
                v0::Message::try_compile(
                    &authority,
                    &ixs,
                    &[], // TODO LUT
                    Hash::default(),
                ).unwrap()
            ),
        };
        Ok(tx)
    })
        
}