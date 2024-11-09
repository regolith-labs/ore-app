use dioxus::prelude::*;

use crate::{
    components::{OreIcon, QrCodeIcon},
    gateway::GatewayResult,
    hooks::{use_ore_balance, use_token_balance},
    route::Route,
    steel_app::solana::{account_decoder::parse_token::UiTokenAmount, sdk::pubkey::Pubkey},
};

pub fn Trade() -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-8 w-screen pb-28 sm:pb-16",
            Balance {}
            AssetTable {}
        }
    }
}

fn Balance() -> Element {
    let balance = use_ore_balance();
    rsx! {
        div {
            class: "flex flex-col gap-4 mx-5 sm:mx-8",
            span {
                class: "font-wide font-semibold text-lg",
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
                QrButton {}
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

#[component]
fn OreValue(ui_amount_string: String) -> Element {
    let units: Vec<_> = ui_amount_string.split('.').collect();
    let big_units = units[0];
    let small_units = units[1];
    rsx! {
        div {
            class: "flex flex-row gap-3 h-10 w-min",
            OreIcon {
                class: "h-9 w-9 my-auto"
            }
            div {
                class: "flex flex-row my-auto",
                span {
                    class: "mt-auto font-wide font-semibold text-4xl",
                    "{big_units}"
                }
                span {
                    class: "mt-auto font-wide font-semibold text-3xl text-gray-700",
                    ".{small_units}"
                }
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
            mint: Pubkey::new_from_array([0; 32]),
            name: "Solana".to_owned(),
            ticker: "SOL".to_owned(),
            description: "".to_owned(),
            image: "https://upload.wikimedia.org/wikipedia/en/b/b9/Solana_logo.png".to_owned(),
        },
        Asset {
            mint: Pubkey::new_from_array([0; 32]),
            name: "International Stable Currency".to_owned(),
            ticker: "USDC".to_owned(),
            description: "".to_owned(),
            image: "https://cdn.prod.website-files.com/66327d2c71b7019a2a9a1b62/667454fd94c7f58e94f4a009_USDC-webclip-256x256.png"
                .to_owned(),
        },
        Asset {
            mint: Pubkey::new_from_array([0; 32]),
            name: "International Stable Currency".to_owned(),
            ticker: "ISC".to_owned(),
            description: "".to_owned(),
            image: "https://raw.githubusercontent.com/theISCTeam/isc_meta/master/logo.png"
                .to_owned(),
        },
        Asset {
            mint: Pubkey::new_from_array([0; 32]),
            name: "Mobile".to_owned(),
            ticker: "MOBILE".to_owned(),
            description: "".to_owned(),
            image: "https://shdw-drive.genesysgo.net/6tcnBSybPG7piEDShBcrVtYJDPSvGrDbVvXmXKpzBvWP/mobile.png".to_owned(),
        },
        Asset {
            mint: Pubkey::new_from_array([0; 32]),
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
            class: "flex flex-row h-10 px-5 sm:px-3 justify-between font-medium text-sm text-gray-700",
            span {
                class: "my-auto",
                "Asset"
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
    // TODO Fetch balance
    // TODO Fetch price
    // TODO Fetch 24h change
    rsx! {
        Link {
            to: Route::Asset { asset: asset.ticker.clone() },
            class: "flex flex-row w-full px-5 sm:px-3 py-4 justify-between transition sm:rounded-md hover:bg-gray-900 hover:cursor-pointer",
            div {
                class: "flex flex-row gap-4",
                img {
                    class: "w-9 h-9 my-auto bg-gray-900 rounded-full",
                    src: "{asset.image}"
                }
                div {
                    class: "flex flex-col",
                    span {
                        class: "font-wide font-semibold text-lg",
                        "{asset.ticker}"
                    }
                    span {
                        class: "text-gray-700",
                        "0.00"
                    }
                }
            }
            div {
                class: "flex flex-col text-right",
                OreValueSmall {
                    ui_amount_string: "1.20245"
                }
                span {
                    class: "text-green-500",
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
            class: "flex flex-row gap-2 w-min",
            OreIcon {
                class: "h-5 w-5 my-auto"
            }
            div {
                class: "flex flex-row my-auto",
                span {
                    class: "mt-auto font-wide font-semibold text-lg",
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
