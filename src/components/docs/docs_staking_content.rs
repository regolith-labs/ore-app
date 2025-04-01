use dioxus::prelude::*;

use crate::components::*;
use crate::route::Route;

use super::*;

pub fn StakingContent() -> Element {
    rsx! {
        ContentSection {
            span {
                span {
                    class: "font-semibold",
                    "Staking allows liquidity providers to earn yield by making markets more efficient for traders. "
                }
                "These incentives help offset the risk that liquidity providers inherently take on by supporting the market."
            }
        }
    }
}
