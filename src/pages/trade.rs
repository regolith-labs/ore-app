use std::str::FromStr;

use dioxus::prelude::*;

use crate::{
    components::{Balance, Col, OreValueSmall,  Row, SwapIcon, Table, TableHeader, TableRowLink, Worth}, gateway::GatewayResult, hooks::use_token_balance, route::Route, steel_app::solana::{account_decoder::parse_token::UiTokenAmount, sdk::pubkey::Pubkey}
};

pub fn Trade() -> Element {
    rsx! {
        Col {
            class: "w-full pb-20 sm:pb-16",
            gap: 8,
            Header {}
            Col {
                class: "md:flex-row md:gap-0 px-5 sm:px-8",
                gap: 8,
                Balance {}
                Worth {}
            }
            AssetTable {}
        }
    }
}

fn Header() -> Element {
    rsx! {
        Row {
            class: "justify-between h-10 px-5 sm:px-8",
            gap: 4,
            span {
                class: "font-wide text-2xl sm:text-3xl font-semibold align-text-bottom my-auto",
                "Trade"
            }
            SwapButton {}
        }
    }
}

fn SwapButton() -> Element {
    rsx! {
        Link {
            to: Route::Swap {},
            class: "h-10 controls-primary rounded-full px-4 gap-2",
            SwapIcon {
                class: "h-5 w-5 mx-auto my-auto"
            }
            span {
                class: "my-auto",
                "Swap"
            }
        }
    }
}

fn AssetTable() -> Element {
    // TODO Read from config file
    let listed_assets = [
        Asset {
            mint: Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap(),
            name: "Solana".to_owned(),
            ticker: "SOL".to_owned(),
            description: "".to_owned(),
            image: "https://upload.wikimedia.org/wikipedia/en/b/b9/Solana_logo.png".to_owned(),
        },
        Asset {
            mint: Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap(),
            name: "USDC".to_owned(),
            ticker: "USDC".to_owned(),
            description: "".to_owned(),
            image: "https://cdn.prod.website-files.com/66327d2c71b7019a2a9a1b62/667454fd94c7f58e94f4a009_USDC-webclip-256x256.png"
                .to_owned(),
        },
        Asset {
            mint: Pubkey::from_str("J9BcrQfX4p9D1bvLzRNCbMDv8f44a9LFdeqNE4Yk2WMD").unwrap(),
            name: "International Stable Currency".to_owned(),
            ticker: "ISC".to_owned(),
            description: "".to_owned(),
            image: "https://raw.githubusercontent.com/theISCTeam/isc_meta/master/logo.png"
                .to_owned(),
        },
        Asset {           
            mint: Pubkey::from_str("mb1eu7TzEc71KxDpsmsKoucSSuuoGLv1drys1oP2jh6").unwrap(),
            name: "Mobile".to_owned(),
            ticker: "MOBILE".to_owned(),
            description: "".to_owned(),
            image: "https://shdw-drive.genesysgo.net/6tcnBSybPG7piEDShBcrVtYJDPSvGrDbVvXmXKpzBvWP/mobile.png".to_owned(),
        },
        Asset {
            mint: Pubkey::from_str("4vMsoUT2BWatFweudnQM1xedRLfJgJ7hswhcpz4xgBTy").unwrap(),
            name: "Honey".to_owned(),
            ticker: "HONEY".to_owned(),
            description: "".to_owned(),
            image: "https://hivemapper-marketing-public.s3.us-west-2.amazonaws.com/Hivemapper_HONEY_token.png".to_owned(),
        },
    ];

    // TODO Sort by token balances
    rsx! {
        Table {
            header: rsx! {
                TableHeader {
                    left: "Market",
                    right_1: "Price",
                    right_2: "Valuation"
                }
            },
            rows: rsx! {
                for asset in listed_assets {
                    AssetRow { asset: asset }
                }
            }
        }
    }
}

#[component]
fn AssetRow(asset: Asset) -> Element {
    let balance = use_token_balance(asset.mint);
    rsx! {
        TableRowLink {
            to: Route::Market { market: asset.ticker.clone() },
            left: rsx! { AssetNameAndBalance { asset: asset.clone(), balance: balance } },
            right_1: rsx! { AssetQuote { asset: asset.clone() } },
            right_2: rsx! { AssetValue { asset: asset, balance: balance } },
            // right_2: rsx! { },
        }
    }
    
}

#[component]
fn AssetNameAndBalance(asset: Asset, balance: Resource<GatewayResult<UiTokenAmount>>) -> Element {
    rsx! {
        Row {
            gap: 4,
            img {
                class: "w-10 h-10 my-auto bg-gray-900 rounded-full border border-gray-800",
                src: "{asset.image}"
            }
            Col {
                span {
                    class: "font-medium",
                    "{asset.ticker}"
                }
                span {
                    class: "font-medium text-gray-700 h-5 text-sm",
                    match balance.cloned() {
                        None => rsx! {
                            div { class: "h-5 w-20 loading rounded"}
                        },
                        Some(balance) => {
                            match balance {
                                Err(_) => rsx!{ "0" },
                                Ok(b) => rsx!{ "{b.ui_amount_string}" },
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn AssetQuote(asset: Asset) -> Element {
    rsx! {
        Col {
            class: "text-right",
            OreValueSmall {
                ui_amount_string: "1.20245"
            }
            span {
                class: "font-medium text-green-500 text-sm",
                "0.2%"
            }
        }
    }
}

#[component]
fn AssetValue(asset: Asset, balance: Resource<GatewayResult<UiTokenAmount>>) -> Element {
    let mut value = use_signal(|| "0.000".to_string());
    let price = 1.2; // TODO

    use_effect(move || {
        if let Some(balance) = balance.cloned() {
            match balance {
               Err(_) => {
                   value.set("0.000".to_string());
               }
               Ok(balance) => {
                   if let Some(ui_amount) = balance.ui_amount {
                       value.set(format!("{:.3}", ui_amount * price));
                   }
               }
            }
        }
    });

    rsx! {
        if let Some(_balance) = balance.cloned() {
            OreValueSmall {
                ui_amount_string: "{*value.read()}"
            }
        } else {
            div {
                class: "loading w-24 h-8 rounded",
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
    image: String,
}
