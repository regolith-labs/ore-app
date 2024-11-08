use dioxus::prelude::*;

use crate::{components::OreIcon, hooks::use_ore_balance, steel_app::solana::sdk::pubkey::Pubkey};

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
                    match balance {
                        Ok(balance) => {
                            let units: Vec<_> = balance.ui_amount_string.split('.').collect();
                            let big_units = units[0];
                            let small_units = units[1];
                            rsx! {
                                div {
                                    class: "flex flex-row gap-3 h-10",
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
                        Err(err) => {
                            rsx! {
                                span {
                                    class: "h-10 font-wide text-3xl font-semibold",
                                    "Error {err:?}"
                                }
                            }
                        }
                    }
                }
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
