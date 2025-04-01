use dioxus::prelude::*;

use crate::components::*;
use crate::route::Route;

use super::*;

pub fn TokenomicsContent() -> Element {
    rsx! {
        ContentSection {
            span {
                span {
                    class: "font-semibold",
                    "The issuance of ORE is managed by a smart contract on the Solana blockchain. "
                }
                "The total supply is capped at 5 million tokens, and emissions automatically reduce by 10% every ~12 months."
            }
            // Key data
            // TODO Current supply
            // TODO Max supply
            // TODO Target emissions rate
            // TODO Daily inflation rate
        }
    }
}
