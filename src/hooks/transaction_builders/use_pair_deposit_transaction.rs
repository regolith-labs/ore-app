use dioxus::prelude::*;
use ore_boost_api::state::Stake;
use solana_extra_wasm::program::{spl_associated_token_account::{get_associated_token_address, instruction::{create_associated_token_account, create_associated_token_account_idempotent}}, spl_token::{self, instruction::{close_account, sync_native}, ui_amount_to_amount}};
use solana_sdk::{native_token::sol_to_lamports, system_instruction::transfer, transaction::{Transaction, VersionedTransaction}};

use crate::{
    components::TokenInputError, config::{BoostMeta, LpType, Token}, gateway::{kamino::KaminoGateway, meteora::MeteoraGateway, GatewayError, GatewayResult, UiTokenAmount}, hooks::{use_gateway, use_wallet, Wallet, use_token_balance}, utils::LiquidityPair
};

// Build pair deposit transaction
pub fn use_pair_deposit_transaction(
    boost_meta: BoostMeta,
    liquidity_pair: Resource<GatewayResult<LiquidityPair>>,
    lp_balance: Resource<GatewayResult<UiTokenAmount>>,
    stake: Resource<GatewayResult<Stake>>,
    token_a_balance: Resource<GatewayResult<UiTokenAmount>>,
    token_b_balance: Resource<GatewayResult<UiTokenAmount>>,
    input_amount_a: Signal<String>,
    input_amount_b: Signal<String>,
    mut err: Signal<Option<TokenInputError>>
) -> Resource<GatewayResult<VersionedTransaction>> {
    let wallet = use_wallet();
    let sol_balance = use_token_balance(Token::sol().mint);
    use_resource(move || async move {
        // Reset error
        err.set(None);

        // Check if wallet is connected
        let Wallet::Connected(authority) = *wallet.read() else {
            return Err(GatewayError::WalletDisconnected);
        };

        // Parse input amounts
        let Ok(amount_a_f64) = input_amount_a.cloned().parse::<f64>() else {
            return Err(GatewayError::Unknown);
        };
        let Ok(amount_b_f64) = input_amount_b.cloned().parse::<f64>() else {
            return Err(GatewayError::Unknown);
        };
        if amount_a_f64 == 0f64 || amount_b_f64 == 0f64 {
            return Err(GatewayError::Unknown);
        }

        // Check if user has enough Sol to begin with
        if let Some(Ok(token_amount)) = sol_balance.cloned() {
            let sol_amount = ui_amount_to_amount(token_amount.ui_amount.unwrap(), token_amount.decimals);
            if sol_amount < sol_to_lamports(0.1) {
                err.set(Some(TokenInputError::InsufficientSol));
                return Err(GatewayError::Unknown);
            }
        }
    
        // Get resources
        let Some(Ok(liquidity_pair)) = liquidity_pair.cloned() else {
            return Err(GatewayError::Unknown);
        };
        let Some(Ok(token_a_balance)) = token_a_balance.cloned() else {
            if amount_a_f64 > 0f64 {
                err.set(Some(TokenInputError::InsufficientBalance(liquidity_pair.token_a.clone()))); // 
            }
            return Err(GatewayError::Unknown);
        };
        let Some(Ok(token_b_balance)) = token_b_balance.cloned() else {
            if amount_b_f64 > 0f64 {
                err.set(Some(TokenInputError::InsufficientBalance(liquidity_pair.token_b.clone())));
            }
            return Err(GatewayError::Unknown);
        };
    
        // Check if wallet balances are sufficient
        if amount_a_f64 > token_a_balance.ui_amount.unwrap_or(0.0) {
            err.set(Some(TokenInputError::InsufficientBalance(liquidity_pair.token_a.clone())));
            return Err(GatewayError::Unknown);
        }
        if amount_b_f64 > token_b_balance.ui_amount.unwrap_or(0.0) {
            err.set(Some(TokenInputError::InsufficientBalance(liquidity_pair.token_b.clone())));
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
        let wsol_amount = if liquidity_pair.token_a.is_sol() {
            amount_a_f64
        } else if liquidity_pair.token_b.is_sol() {
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
                let amount_a_u64 = ui_amount_to_amount(amount_a_f64, liquidity_pair.token_a.decimals);
                let amount_b_u64 = ui_amount_to_amount(amount_b_f64, liquidity_pair.token_b.decimals);
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
