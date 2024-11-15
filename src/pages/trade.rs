use std::str::FromStr;

use dioxus::prelude::*;

use crate::{
    components::{Col, OreValue, OreValueSmall, QrCodeIcon, Row, SwapIcon, Table, TableHeader, TableRowLink},
    gateway::GatewayResult,
    hooks::{use_ore_balance,  use_token_balance},
    route::Route,
    steel_app::solana::{account_decoder::parse_token::UiTokenAmount, sdk::pubkey::Pubkey},
};

pub fn Trade() -> Element {
    rsx! {
        Col {
            class: "w-full pb-20 sm:pb-16",
            gap: 8,
            Row {
                class: "justify-between sm:hidden mx-5 sm:mx-8 h-10",
                gap: 4,
                span {
                    class: "font-wide text-2xl sm:text-3xl font-semibold align-text-bottom my-auto",
                    "Trade"
                }
                SwapButton {}
            }
            Balance {}
            AssetTable {}
        }
    }
}

fn Balance() -> Element {
    let balance = use_ore_balance();
    rsx! {
        Col {
            class: "sm:gap-4 mx-5 sm:mx-8",
            gap: 2,
            span {
                class: "font-medium text-xs sm:text-sm text-gray-700",
                "Balance"
            }
            Row {
                class: "justify-between align-top",
                match balance.cloned() {
                    None => {
                        rsx! {
                            span {
                                class: "h-10 w-64 loading rounded"
                            }
                        }
                    }
                    Some(balance) => {
                        rsx! {
                            OreBalance {
                                balance: balance
                            }
                        }
                    }
                }
                Row {
                    gap: 4,
                    PayButton {}
                    span {
                        class: "hidden sm:flex",
                        SwapButton {}
                    }
                }
            }
        }
    }
}

#[component]
fn OreBalance(balance: GatewayResult<UiTokenAmount>) -> Element {
    match balance {
        Ok(balance) => {
            rsx! {
                OreValue {
                    ui_amount_string: balance.ui_amount_string
                }
            }
        }
        Err(err) => {
            rsx! {
                Col {
                    gap: 2,
                    OreValue {
                        ui_amount_string: "0.000"
                    }
                    span {
                        class: "text-sm font-medium text-red-500",
                        "Error: {err:?}"
                    }
                }
            }
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

fn PayButton() -> Element {
    rsx! {
        Link {
            to: Route::Pay {},
            class: "h-10 controls-secondary rounded-full px-4 gap-2",
            QrCodeIcon {
                class: "h-6 w-6 mx-auto my-auto"
            }
            span {
                class: "my-auto",
                "Pay"
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
            TableHeader {
                left: "Market",
                    left_width: 40,
                right: vec!["Price".to_string()]
            }
            for asset in listed_assets {
                TableRowLink {
                    to: Route::Market { market: asset.ticker.clone() },
                    left_width: 40,
                    left: rsx! {
                        AssetNameAndBalance { asset: asset.clone() }
                    },
                    right: vec![
                        rsx! { AssetQuote { asset: asset } },
                    ]
                }
            }
        }
    }
}

#[component]
fn AssetNameAndBalance(asset: Asset) -> Element {
    let balance = use_token_balance(asset.mint);
    rsx! {
        Row {
            // class: "w-40",
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

#[derive(Clone, PartialEq, Eq)]
struct Asset {
    mint: Pubkey,
    name: String,
    ticker: String,
    description: String,
    image: String,
}
