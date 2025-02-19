use dioxus::prelude::*;
use ore_api::consts::{MINT_ADDRESS, TOKEN_DECIMALS};
use ore_boost_api::state::Stake;
use solana_extra_wasm::program::spl_token::{amount_to_ui_amount, ui_amount_to_amount};
use solana_sdk::{native_token::sol_to_lamports, transaction::{Transaction, VersionedTransaction}};

use crate::{
    components::TokenInputError, config::Token, gateway::{GatewayError, GatewayResult}, hooks::{use_wallet, Wallet, _use_sol_balance}
};

pub fn use_idle_withdraw_transaction(
    stake: Resource<GatewayResult<Stake>>,
    input_amount: Signal<String>,
    mut err: Signal<Option<TokenInputError>>
) -> Resource<GatewayResult<VersionedTransaction>> {
    let wallet = use_wallet();
    let sol_balance = _use_sol_balance();
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

        // If they don't have enough SOL, disable
        if let Some(Ok(ore_balance)) = sol_balance.cloned() {
            if ore_balance < sol_to_lamports(0.1) {
                err.set(Some(TokenInputError::InsufficientSol));
                return Err(GatewayError::Unknown);
            }
        }    

        // If amount is greater than stake balance, disable
        if let Some(Ok(stake)) = stake.read().as_ref() {
            if amount_to_ui_amount(stake.balance + stake.balance_pending, TOKEN_DECIMALS) < amount_f64 {
                err.set(Some(TokenInputError::InsufficientBalance(Token::ore())));
                return Err(GatewayError::Unknown);
            }
        } else {
            err.set(Some(TokenInputError::InsufficientBalance(Token::ore())));
            return Err(GatewayError::Unknown);
        }

        // Aggregate instructions
        let mut ixs = vec![];

        // Build withdraw instruction
        let amount_u64 = ui_amount_to_amount(amount_f64, TOKEN_DECIMALS);
        ixs.push(ore_boost_api::sdk::withdraw(authority, MINT_ADDRESS, amount_u64));

        // Build transaction
        let tx = Transaction::new_with_payer(&ixs, Some(&authority)).into();
        Ok(tx)
    })
        
}
