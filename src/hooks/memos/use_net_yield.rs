use dioxus::prelude::*;
use ore_api::consts::TOKEN_DECIMALS;
use solana_extra_wasm::program::spl_token::amount_to_ui_amount;

use crate::{gateway::{GatewayResult, UiTokenAmount}, hooks::use_all_stakes};

pub fn use_net_yield() -> Memo<GatewayResult<UiTokenAmount>> {
    let stake_accounts = use_all_stakes();
    
    use_memo(move || {
        // Iterate through all stake accounts and sum the rewards
        let mut net_yield = 0;
        for (_mint, stake) in stake_accounts.iter() {
            if let Some(Ok(stake)) = *stake.read() {
                net_yield += stake.rewards;
            }
        }

        // Convert to a UI amount
        let net_yield_f64 = amount_to_ui_amount(net_yield, TOKEN_DECIMALS);
        Ok(UiTokenAmount {
            ui_amount: Some(net_yield_f64),
            ui_amount_string: format!("{:.1$}", net_yield_f64, TOKEN_DECIMALS as usize)
                .trim_end_matches("0")
                .trim_end_matches(".")
                .to_string(),
            amount: net_yield.to_string(),
            decimals: TOKEN_DECIMALS,
        })
    })
}