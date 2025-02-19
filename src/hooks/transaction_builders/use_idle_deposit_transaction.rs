use dioxus::prelude::*;
use ore_api::consts::{MINT_ADDRESS, TOKEN_DECIMALS};
use ore_boost_api::state::Stake;
use solana_extra_wasm::program::spl_token::ui_amount_to_amount;
use solana_sdk::transaction::{Transaction, VersionedTransaction};

use crate::{
    components::TokenInputError, config::Token, gateway::{GatewayError, GatewayResult, UiTokenAmount}, hooks::{use_wallet, Wallet, use_sol_balance, MIN_SOL_BALANCE}
};

pub fn use_idle_deposit_transaction(
    stake: Resource<GatewayResult<Stake>>,
    ore_balance: Resource<GatewayResult<UiTokenAmount>>,
    input_amount: Signal<String>,
    mut err: Signal<Option<TokenInputError>>
) -> Resource<GatewayResult<VersionedTransaction>> {
    let wallet = use_wallet();
    let sol_balance = use_sol_balance();
    use_resource(move || async move {
        err.set(None);

        // Check if wallet is connected
        let Wallet::Connected(authority) = *wallet.read() else {
            return Err(GatewayError::WalletDisconnected);
        };

         // If empty, disable
         let amount_str = input_amount.cloned();
         if amount_str.is_empty() {
            return Err(GatewayError::Unknown);
        }

        // If input isn't a number, disable
        let Ok(amount_f64) = amount_str.parse::<f64>() else {
            return Err(GatewayError::Unknown);
        };

        // If amount is 0, disable
        if amount_f64 == 0f64 {
            return Err(GatewayError::Unknown);
        }

        // Check if user has enough Sol to begin with
        if let Some(Ok(sol_balance)) = sol_balance.cloned() {
            if sol_balance.ui_amount.unwrap() < MIN_SOL_BALANCE {
                err.set(Some(TokenInputError::InsufficientSol));
                return Err(GatewayError::Unknown);
            }
        }

        // If amount is greater than ore balance, disable
        if let Some(Ok(ore_balance)) = ore_balance.read().as_ref() {
            if ore_balance.ui_amount.unwrap_or(0.0) < amount_f64 {
                err.set(Some(TokenInputError::InsufficientBalance(Token::ore())));
                return Err(GatewayError::Unknown);
            }
        } else {
            err.set(Some(TokenInputError::InsufficientBalance(Token::ore())));
            return Err(GatewayError::Unknown);
        }

        // Aggregate instructions
        let mut ixs = vec![];

        // Open stake account, if necessary
        if let Some(Ok(_)) = *stake.read() {
            // Do nothing
        } else {
            ixs.push(ore_boost_api::sdk::open(authority, authority, MINT_ADDRESS));
        }

        // Build deposit instruction
        let amount_u64 = ui_amount_to_amount(amount_f64, TOKEN_DECIMALS);
        ixs.push(ore_boost_api::sdk::deposit(authority, MINT_ADDRESS, amount_u64));
    
        // Build transaction
        let tx = Transaction::new_with_payer(&ixs, Some(&authority)).into();
        Ok(tx)
    })
        
}
