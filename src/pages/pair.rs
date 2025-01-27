use std::str::FromStr;

use dioxus::prelude::*;
use steel::Pubkey;

use crate::{components::{Col, Heading, PairStakeForm}, config::LISTED_BOOSTS_BY_MINT, hooks::use_kamino_strategy_metrics};

#[component]
pub fn Pair(lp_mint: String) -> Element {
    let lp_mint = Pubkey::from_str(&lp_mint).unwrap();
    let boost_meta = LISTED_BOOSTS_BY_MINT.get(&lp_mint).unwrap();
    let strategy_metrics = use_kamino_strategy_metrics(boost_meta.lp_id);

    // TODO Get the boost
    // TODO Show error if boost is not listed
    
    rsx! {
        Col {
            class: "w-full h-full pb-20 sm:pb-16",
            gap: 8,
            Heading {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                title: boost_meta.name.clone(),
                subtitle: "Manage your liquidity position."
            }
            PairStakeForm {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                mint: lp_mint
            }
        }
    }
}

