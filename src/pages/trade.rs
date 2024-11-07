use dioxus::prelude::*;

use crate::steel_app::solana::sdk::pubkey::Pubkey;

pub fn Trade() -> Element {
    let listed_assets = [Asset {
        mint: Pubkey::new_from_array([0; 32]),
        name: "Solana".to_owned(),
        ticker: "SOL".to_owned(),
        description: "".to_owned(),
        icon: "".to_owned(),
    }];

    rsx! {
        div {
            class: "flex flex-col",
            "Trade"
            for asset in listed_assets {
                AssetRow {
                    asset: asset
                }
            }
        }
    }
}

#[component]
fn AssetRow(asset: Asset) -> Element {
    // TODO Fetch balance
    // TODO Fetch price
    // TODO Fetch 24h change
    rsx! {
        div {
            class: "flex flex-row w-full",
            div {
                class: "flex flex-row gap-8",
                div {
                    class: "w-16 h-16 bg-grey-200 rounded-full",
                }
                div {
                    class: "flex flex-col",
                    "{asset.ticker}"
                    "0.00"
                }
            }
            div {
                class: "flex flex-col",
                "1.2"
                "+0.2%"
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Asset {
    mint: Pubkey,
    name: String,
    ticker: String,
    description: String,
    icon: String,
}
