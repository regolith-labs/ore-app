use std::str::FromStr;

use dioxus::prelude::*;

use crate::{
    components::{OreIcon, OreValue, QrCodeIcon, TradeIcon},
    gateway::GatewayResult,
    hooks::{use_ore_balance, use_quote, use_token_balance},
    route::Route,
    steel_app::solana::{account_decoder::parse_token::UiTokenAmount, sdk::pubkey::Pubkey},
};

pub fn Trade() -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-8 w-screen pb-20 sm:pb-16",
            span {
                class: "flex flex-row justify-between sm:hidden mx-5 sm:mx-8 font-wide text-2xl font-semibold",
                "Trade"
                TradeButton {}
            }
            Balance {}
            AssetTable {}
        }
    }
}

fn Balance() -> Element {
    let balance = use_ore_balance();
    rsx! {
        div {
            class: "flex flex-col gap-2 sm:gap-4 mx-5 sm:mx-8",
            span {
                class: "font-medium text-xs sm:text-sm text-gray-700",
                "Balance"
            }
            div {
                class: "flex flex-row justify-between align-top",
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
                div {
                    class: "flex flex-row gap-4",
                    QrButton {}
                    span {
                        class: "hidden sm:flex",
                        TradeButton {}
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
                div {
                    class: "flex flex-col gap-2",
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


fn TradeButton() -> Element {
    rsx! {
        button {
            class: "flex h-10 w-10 rounded-md transition text-black bg-white",
            onclick: move |_| {
                // TODO Send/receive modal
            },
            TradeIcon {
                class: "h-5 w-5 mx-auto my-auto"
            }
        }
    }
}

fn QrButton() -> Element {
    rsx! {
        button {
            class: "flex h-10 w-10 rounded-md transition text-gray-200 bg-gray-900 hover:bg-gray-800 hover:text-white",
            onclick: move |_| {
                // TODO Send/receive modal
            },
            QrCodeIcon {
                class: "h-6 w-6 mx-auto my-auto"
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
        div {
            class: "flex flex-col sm:mx-5",
            AssetTableHeader {}
            for asset in listed_assets {
                AssetRow {
                    asset: asset
                }
            }
        }
    }
}

fn AssetTableHeader() -> Element {
    rsx! {
        div {
            class: "flex flex-row h-8 h-10 px-5 sm:px-3 justify-between font-medium text-xs sm:text-sm text-gray-700",
            span {
                class: "my-auto",
                "Market"
            }
            span {
                class: "my-auto",
                "Price"
            }
        }
    }
}

#[component]
fn AssetRow(asset: Asset) -> Element {
    let balance = use_token_balance(asset.mint);
    let quote = use_quote(asset.mint);
    // TODO Fetch price
    // TODO Fetch 24h change
    rsx! {
        Link {
            to: Route::Asset { asset: asset.ticker.clone() },
            class: "flex flex-row w-full px-5 sm:px-3 py-4 justify-between transition sm:rounded-md hover:bg-gray-900 hover:cursor-pointer",
            div {
                class: "flex flex-row gap-4",
                img {
                    class: "w-10 h-10 my-auto bg-gray-900 rounded-full",
                    src: "{asset.image}"
                }
                div {
                    class: "flex flex-col",
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
                                    Err(_) => rsx!{ "0.00" },
                                    Ok(b) => rsx!{ "{b.ui_amount_string}" },
                                }
                            }
                        }
                    }
                }
            }
            div {
                class: "flex flex-col text-right",
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
}

#[component]
fn OreValueSmall(ui_amount_string: String) -> Element {
    rsx! {
        div {
            class: "flex flex-row gap-1.5 w-min",
            OreIcon {
                class: "h-3.5 w-3.5 my-auto"
            }
            div {
                class: "flex flex-row font-medium my-auto",
                span {
                    class: "mt-auto",
                    "{ui_amount_string}"
                }
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
