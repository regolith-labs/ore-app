use dioxus::prelude::*;
use ore_api::consts::{MINT_ADDRESS, TOKEN_DECIMALS};

use crate::{
    gateway::{GatewayResult, UiTokenAmount},
    hooks::{use_all_liquidity_pairs, use_all_stakes},
    solana::spl_token::{amount_to_ui_amount, ui_amount_to_amount},
};

pub fn use_net_deposits() -> Memo<GatewayResult<UiTokenAmount>> {
    let stake_accounts = use_all_stakes();
    let liquidity_pairs = use_all_liquidity_pairs();

    use_memo(move || {
        // Iterate through all stake accounts and sum the deposits
        let mut net_deposits = 0;
        for (mint, stake) in stake_accounts.iter() {
            if let Some(Ok(stake)) = *stake.read() {
                if mint == &MINT_ADDRESS {
                    net_deposits += stake.balance + stake.balance_pending;
                } else if let Some(liquidity_pair_resource) = liquidity_pairs.get(&mint) {
                    if let Some(Ok(liquidity_pair)) = liquidity_pair_resource.cloned() {
                        let (ore_amount_f64, _token_amount_f64, _token_ticker, _token_decimals) =
                            liquidity_pair.get_stake_amounts(stake.balance);
                        let (
                            ore_amount_pending_f64,
                            _token_amount_pending_f64,
                            _token_ticker_pending,
                            _token_decimals_pending,
                        ) = liquidity_pair.get_stake_amounts(stake.balance_pending);
                        let ore_amount_u64 = ui_amount_to_amount(ore_amount_f64, TOKEN_DECIMALS);
                        let ore_amount_pending_u64 =
                            ui_amount_to_amount(ore_amount_pending_f64, TOKEN_DECIMALS);
                        net_deposits += ore_amount_u64 + ore_amount_pending_u64;
                    }
                }
            }
        }

        // Convert to a UI amount
        let net_deposits_f64 = amount_to_ui_amount(net_deposits, TOKEN_DECIMALS);
        Ok(UiTokenAmount {
            ui_amount: Some(net_deposits_f64),
            ui_amount_string: format!("{:.1$}", net_deposits_f64, TOKEN_DECIMALS as usize)
                .trim_end_matches("0")
                .trim_end_matches(".")
                .to_string(),
            amount: net_deposits.to_string(),
            decimals: TOKEN_DECIMALS,
        })
    })
}
