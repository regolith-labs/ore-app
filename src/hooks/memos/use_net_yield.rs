use dioxus::prelude::*;
use ore_api::{consts::TOKEN_DECIMALS, state::proof_pda};

use crate::{
    gateway::{GatewayResult, UiTokenAmount},
    hooks::{calculate_claimable_yield, use_all_boost_proofs, use_all_boosts, use_all_stakes},
    solana::spl_token::amount_to_ui_amount,
};

pub fn use_net_yield() -> Memo<GatewayResult<UiTokenAmount>> {
    let boosts = use_all_boosts();
    let boost_proofs = use_all_boost_proofs();
    let stakes = use_all_stakes();

    use_memo(move || {
        // Iterate through all stake accounts and sum the rewards
        let mut net_yield = 0;
        for (_pubkey, stake) in stakes.iter() {
            if let Ok(stake) = stake.cloned() {
                let boost = boosts.get(&stake.boost).unwrap();
                if let Ok(boost) = boost.cloned() {
                    let proof_address = proof_pda(stake.boost).0;
                    let boost_proof = boost_proofs.get(&proof_address).unwrap();
                    if let Ok(boost_proof) = boost_proof.cloned() {
                        let claimable_yield = calculate_claimable_yield(boost, boost_proof, stake);
                        net_yield += claimable_yield;
                    }
                }
            }
        }

        // Convert to a UI amount
        let net_yield_f64 = amount_to_ui_amount(net_yield, TOKEN_DECIMALS);
        Ok(UiTokenAmount {
            ui_amount: Some(net_yield_f64),
            ui_amount_string: format!("{:.1$}", net_yield_f64, TOKEN_DECIMALS as usize).to_string(),
            amount: net_yield.to_string(),
            decimals: TOKEN_DECIMALS,
        })
    })
}
