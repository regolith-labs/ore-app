use dioxus::prelude::*;
use ore_api::consts::TOKEN_DECIMALS;

use crate::{
    gateway::{GatewayResult, UiTokenAmount},
    hooks::{
        calculate_claimable_yield, use_all_boosts, use_all_stakes, use_boost_config_wss,
        use_boost_proof_wss,
    },
    solana::spl_token::amount_to_ui_amount,
};

pub fn use_net_yield() -> Memo<GatewayResult<UiTokenAmount>> {
    let boosts = use_all_boosts();
    let boost_proof = use_boost_proof_wss();
    let boost_config = use_boost_config_wss();
    let stakes = use_all_stakes();

    use_memo(move || {
        // Iterate through all stake accounts and sum the rewards
        let mut net_yield = 0;
        for (_pubkey, stake) in stakes.iter() {
            if let Ok(stake) = stake.cloned() {
                let boost = boosts.get(&stake.boost).unwrap();
                if let Ok(boost) = boost.cloned() {
                    if let Ok(boost_proof) = boost_proof.cloned() {
                        if let Ok(boost_config) = boost_config.cloned() {
                            let claimable_yield =
                                calculate_claimable_yield(boost, boost_proof, stake, boost_config);
                            net_yield += claimable_yield;
                        }
                    }
                }
            }
        }

        if net_yield == 0 {
            Ok(UiTokenAmount {
                ui_amount: Some(0.0),
                ui_amount_string: "0.000".to_string(),
                amount: "0".to_string(),
                decimals: TOKEN_DECIMALS,
            })
        } else {
            let net_yield_f64 = amount_to_ui_amount(net_yield, TOKEN_DECIMALS);
            Ok(UiTokenAmount {
                ui_amount: Some(net_yield_f64),
                ui_amount_string: format!("{:.1$}", net_yield_f64, TOKEN_DECIMALS as usize)
                    .to_string(),
                amount: net_yield.to_string(),
                decimals: TOKEN_DECIMALS,
            })
        }
    })
}
