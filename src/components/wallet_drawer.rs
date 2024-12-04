use dioxus::prelude::*;
use solana_extra_wasm::account_decoder::parse_token::UiTokenAmount;

use crate::components::*;
use crate::gateway::GatewayResult;
use crate::hooks::{use_token_balance, Asset, ASSETS};
use crate::route::Route;

#[component]
pub fn WalletDrawer(on_close: EventHandler) -> Element {
    rsx! {
        div {
            class: "h-full w-96 elevated elevated-border text-white p-4 z-50",
            onclick: move |e| e.stop_propagation(),
            AssetTable {}
        }
    }
}

fn AssetTable() -> Element {
    let listed_assets = ASSETS.values().collect::<Vec<_>>();
    rsx! {
        Col {
            gap: 4,
            Table {
                header:  rsx! {
                    TableHeader {
                        left: "Token",
                        right_1: "Price", 
                        right_2: "Value"
                    }
                },
                rows: rsx! {
                    for asset in listed_assets {
                        AssetRow { asset: asset.clone() }
                    }
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
            // to: Route::Market { market: asset.ticker.clone() },
            to: Route::Trade {},
            left: rsx! { AssetNameAndBalance { asset: asset.clone(), balance: balance } },
            right_1: rsx! { AssetQuote { asset: asset.clone() } },
            right_2: rsx! { AssetValue { asset: asset, balance: balance } },
        }
    }
}

#[component]
fn AssetNameAndBalance(asset: Asset, balance: Resource<GatewayResult<UiTokenAmount>>) -> Element {
    rsx! {
        Row {
            gap: 4,
            img {
                class: "w-10 h-10 my-auto bg-gray-900 rounded-full",
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
