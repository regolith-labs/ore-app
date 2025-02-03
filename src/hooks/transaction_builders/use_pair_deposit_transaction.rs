use dioxus::prelude::*;
use ore_boost_api::state::Stake;
use solana_extra_wasm::program::{spl_associated_token_account::{get_associated_token_address, instruction::{create_associated_token_account, create_associated_token_account_idempotent}}, spl_token::{self, instruction::{close_account, sync_native}}};
use solana_sdk::{native_token::sol_to_lamports, system_instruction::transfer, transaction::{Transaction, VersionedTransaction}};

use crate::{
    components::TokenInputError, config::{BoostMeta, LpType, Token}, gateway::{kamino::KaminoGateway, meteora::MeteoraGateway, GatewayError, GatewayResult, UiTokenAmount}, hooks::{use_gateway, use_wallet, BoostDeposits, Wallet}
};

// Build pair deposit transaction
pub fn use_pair_deposit_transaction(
    boost_meta: BoostMeta,
    boost_deposits: Resource<GatewayResult<BoostDeposits>>,
    lp_balance: Resource<GatewayResult<UiTokenAmount>>,
    stake: Resource<GatewayResult<Stake>>,
    token_a_balance: Resource<GatewayResult<UiTokenAmount>>,
    token_b_balance: Resource<GatewayResult<UiTokenAmount>>,
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
        let Some(Ok(token_a_balance)) = token_a_balance.cloned() else {
            err.set(None);
            return Err(GatewayError::Unknown);
        };
        let Some(Ok(token_b_balance)) = token_b_balance.cloned() else {
            err.set(None);
            return Err(GatewayError::Unknown);
        };
    
        // Parse input amounts
        let Ok(amount_a_f64) = input_amount_a.cloned().parse::<f64>() else {
            err.set(None);
            return Err(GatewayError::Unknown);
        };
        let Ok(amount_b_f64) = input_amount_b.cloned().parse::<f64>() else {
            err.set(None);
            return Err(GatewayError::Unknown);
        };
        if amount_a_f64 == 0f64 || amount_b_f64 == 0f64 {
            err.set(None);
            return Err(GatewayError::Unknown);
        }
    
        // Check if wallet balances are sufficient
        if amount_a_f64 > token_a_balance.ui_amount.unwrap_or(0.0) {
            err.set(Some(TokenInputError::InsufficientBalance(boost_deposits.token_a.clone())));
            return Err(GatewayError::Unknown);
        }
        if amount_b_f64 > token_b_balance.ui_amount.unwrap_or(0.0) {
            err.set(Some(TokenInputError::InsufficientBalance(boost_deposits.token_b.clone())));
            return Err(GatewayError::Unknown);
        }
    
        // Aggregate instructions
        let mut ixs: Vec<steel::Instruction> = vec![];

        // Create ata for lp shares, if needed
        if let Some(Ok(_)) = lp_balance.cloned() {
            // Do nothing
        } else {
            ixs.push(
                create_associated_token_account(&authority, &authority, &boost_meta.lp_mint, &spl_token::ID)
            );
        }

        // Wrap SOL, if needed 
        let sol_mint = Token::sol().mint;
        let wsol_amount = if boost_deposits.token_a.ticker == "SOL" {
            amount_a_f64
        } else if boost_deposits.token_b.ticker == "SOL" {
            amount_b_f64
        } else {
            0f64
        };
        let wsol_ata = get_associated_token_address(&authority, &sol_mint);
        if wsol_amount > 0f64 {
            ixs.push(
                create_associated_token_account_idempotent(&authority, &authority, &sol_mint, &spl_token::ID)
            );
            ixs.push(
                transfer(&authority, &wsol_ata, sol_to_lamports(wsol_amount))
            );
            ixs.push(
                sync_native(&spl_token::ID, &wsol_ata).unwrap()
            );
        }
    
        // Build the instruction
        let deposit_ix = match boost_meta.lp_type {
            LpType::Kamino => {
                let Ok(ix) = use_gateway().build_kamino_deposit_instruction(
                    boost_meta.lp_id,
                    amount_a_f64,
                    amount_b_f64,
                    authority,
                ).await else {
                    err.set(None);
                    return Err(GatewayError::Unknown);
                };
                ix
            }
            LpType::Meteora => {
                // let amount_a_u64 = ui_amount_to_amount(amount_a_f64, token_a_balance.decimals) as u64;
                // let amount_b_u64 = ui_amount_to_amount(amount_b_f64, token_b_balance.decimals) as u64;
                let amount_a_u64 = token_a_balance.amount.parse::<u64>().unwrap();
                let amount_b_u64 = token_b_balance.amount.parse::<u64>().unwrap();
                let Ok(ix) = use_gateway().build_meteora_deposit_instruction(
                    boost_meta.lp_id,
                    amount_a_u64,
                    amount_b_u64,
                    1,
                    authority,
                ).await else {
                    err.set(None);
                    return Err(GatewayError::Unknown);
                };
                ix
            }
        };
        ixs.push(deposit_ix);
    
        // Close the wSOL ata
        if wsol_amount > 0f64 {
            ixs.push(
                close_account(&spl_token::ID, &wsol_ata, &authority, &authority, &[&authority]).unwrap()
            );
        }

        // Open the stake account, if needed
        if let Some(Ok(_stake)) = stake.read().as_ref() {
            // Do nothing
        } else {
            ixs.push(ore_boost_api::sdk::open(authority, authority, boost_meta.lp_mint));
        }
    
        // Stake LP tokens into boost program
        ixs.push(
            ore_boost_api::sdk::deposit(authority, boost_meta.lp_mint, u64::MAX)
        );
    
        // Build transaction
        let tx = Transaction::new_with_payer(&ixs, Some(&authority));
        Ok(tx.into())
    })        
}
