use dioxus::prelude::*;
use solana_extra_wasm::account_decoder::parse_token::UiTokenAmount;

use crate::{
    components::{OreIcon, QrCodeIcon},
    gateway::GatewayResult,
    hooks::use_ore_balance,
    steel_app::solana::sdk::pubkey::Pubkey,
};

pub fn Trade() -> Element {
    rsx! {
        div {
            class: "flex flex-col w-screen px-5 sm:px-8",
            Balance {}
            // AssetTable {}
        }
    }
}

fn Balance() -> Element {
    let balance = use_ore_balance();
    rsx! {
        div {
            class: "flex flex-col gap-4",
            span {
                class: "font-wide font-semibold text-lg",
                "Balance"
            }
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
                        div {
                            class: "flex flex-row justify-between align-top",
                            OreBalance {
                                balance: balance
                            }
                            QrButton {}
                        }
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
                class: "flex flex-row gap-1 my-auto",
                span {
                    class: "mt-auto font-wide font-semibold text-4xl",
                    "{big_units}."
                }
                span {
                    class: "mt-auto font-wide font-semibold text-3xl text-gray-700",
                    "{small_units}"
                }
            }
        }
    }
}

fn QrButton() -> Element {
    rsx! {
        button {
            class: "flex h-10 w-10 rounded-md bg-gray-800",
            onclick: move |_| {
                // TODO Send/receive modal
            },
            QrCodeIcon {
                class: "h-6 w-6 mx-auto my-auto text-gray-200"
            }
        }
    }
}

fn AssetTable() -> Element {
    // TODO Read from file
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
