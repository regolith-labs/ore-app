use dioxus::prelude::*;

use crate::{
    components::*, gateway::GatewayResult, hooks::{use_token_balance, Asset, ASSETS}, route::Route, steel_app::solana::{account_decoder::parse_token::UiTokenAmount, sdk::pubkey::Pubkey}
};

pub fn Trade() -> Element {
    rsx! {
        Col {
            class: "w-full h-full pb-20 sm:pb-16",
            gap: 4,
            Subheading {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                title: "Trade",
                subtitle: "Swap tokens at the best available market price."
            }
            SwapForm {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                mint_a: Pubkey::new_unique(),
                mint_b: Pubkey::new_unique(),
            }
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
                "Balance"
            }
        }
    }
}

fn SwapButton() -> Element {
    rsx! {
        Link {
            to: Route::Swap {},
            class: "h-10 controls-primary rounded-full px-4 gap-2 -mr-2",
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
    let listed_assets = ASSETS.values().collect::<Vec<_>>();
    rsx! {
        Col {
            gap: 4,
            Table {
                header: rsx! {
                    TableHeader {
                        left: "Market",
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
            to: Route::Market { market: asset.ticker.clone() },
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
