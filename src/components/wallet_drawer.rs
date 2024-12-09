use dioxus::document::eval;
use dioxus::prelude::*;
use solana_extra_wasm::account_decoder::parse_token::UiTokenAmount;

use crate::components::*;
use crate::gateway::GatewayResult;
use crate::hooks::{use_token_balance, Asset, ASSETS};
use crate::route::Route;

#[derive(Clone, Copy, PartialEq)]
pub enum WalletTab {
    Tokens,
    Liquidity,
}

#[component]
pub fn WalletDrawer(on_close: EventHandler<MouseEvent>, wallet_remount: Signal<bool>) -> Element {
    let tab = use_signal(|| WalletTab::Tokens);

    rsx! {
        div {
            class: "flex flex-col gap-8 h-full sm:w-96 w-screen elevated elevated-border text-white py-8 z-50",
            onclick: move |e| e.stop_propagation(),
            
            // "TODO: Wallet address + copy button"

            DisconnectButton { wallet_remount },
            Col {
                WalletTabs { tab },
                match *tab.read() {
                    WalletTab::Tokens => rsx! {
                        TokenTable { on_close }
                    },
                    WalletTab::Liquidity => rsx! {
                        LiquidityTable { on_close }
                    }
                }
            }
        }
    }
}

#[component]
fn DisconnectButton(wallet_remount: Signal<bool>) -> Element {
    rsx! {
        button {
            onclick: move |_| {
                wallet_remount.set(true);
                let disconnect = eval(r#"window.OreWalletDisconnecter(); return"#);
                spawn(async move {
                    let _ = disconnect.await;
                });
            },
            "Disconnect"
        }
    }
}

#[component]
fn TokenTable(on_close: EventHandler<MouseEvent>) -> Element {
    let listed_tokens = ASSETS.values().collect::<Vec<_>>();
    rsx! {
        Col {
            gap: 4,
            TableSimple {
                rows: rsx! {
                    for token in listed_tokens {
                        TokenRow { token: token.clone(), on_close: on_close }
                    }
                }
            }
        }
    }
}

#[component]
fn TokenRow(token: Asset, on_close: EventHandler<MouseEvent>) -> Element {
    let balance = use_token_balance(token.mint);
    rsx! {
        TableSimpleRowLink {
            to: Route::Trade {},
            left: rsx! { TokenNameAndBalance { token: token.clone(), balance } },
            right: rsx! { TokenQuote { token } },
            onclick: move |e| {
                on_close.call(e);
            }
        }
    }
}

#[component]
fn TokenNameAndBalance(token: Asset, balance: Resource<GatewayResult<UiTokenAmount>>) -> Element {
    rsx! {
        Row {
            gap: 4,
            img {
                class: "w-8 h-8 my-auto bg-gray-900 rounded-full",
                src: "{token.image}"
            }
            Col {
                span {
                    class: "font-medium",
                    "{token.ticker}"
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
fn TokenQuote(token: Asset) -> Element {
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
fn TokenValue(token: Asset, balance: Resource<GatewayResult<UiTokenAmount>>) -> Element {
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

#[component]
fn LiquidityTable(on_close: EventHandler<MouseEvent>) -> Element {
    let listed_assets = ASSETS.values().collect::<Vec<_>>();
    rsx! {
        TableSimple {
            rows: rsx! {
                for asset in listed_assets {
                    LiquidityRow { 
                        asset: asset.clone(),
                        on_close: on_close
                    }
                }
            }
        }
    }
}

#[component]
fn LiquidityRow(asset: Asset, on_close: EventHandler<MouseEvent>) -> Element {
    rsx! {
        TableSimpleRowLink {
            // to: Route::Pair { 
            //     pair: format!("{}-ORE", asset.ticker.clone())
            // },
            to: Route::Stake {},
            onclick: move |e| {
                on_close.call(e);
            },
            left: rsx! {
                Row {
                    class: "grow shrink-0",
                    gap: 4,
                    Row {
                        class: "shrink-0",
                        img {
                            class: "w-8 h-8 shrink-0 my-auto rounded-full",
                            src: "{asset.image}"
                        }
                        img {
                            class: "w-8 h-8 shrink-0 -ml-2 my-auto rounded-full",
                            src: asset!("/public/icon.png")
                        }
                    }
                    Col {
                        class: "my-auto min-w-32 shrink-0",
                        span {
                            class: "font-medium whitespace-nowrap",
                            "{asset.ticker}-ORE"
                        }
                        span {
                            class: "font-medium text-gray-700 h-5 text-sm",
                            "0"
                        }
                    }
                }
            },
            right: rsx! {
                Col {
                    class: "text-right",
                    OreValueSmall {
                        ui_amount_string: "2.054"
                    }
                    span {
                        class: "font-medium text-elements-gold text-sm",
                        "+0.123"
                    }
                }
            }
        }
    }
}



#[component]
fn WalletTabs(tab: Signal<WalletTab>) -> Element {
    let tokens_class = if *tab.read() == WalletTab::Tokens { 
        "flex-1 h-12 transition-colors text-elements-highEmphasis border-b-2 border-white hover:bg-controls-tertiary"
    } else {
        "flex-1 h-12 transition-colors text-elements-lowEmphasis hover:bg-controls-tertiary"
    };

    let liquidity_class = if *tab.read() == WalletTab::Liquidity {
        "flex-1 h-12 transition-colors text-elements-highEmphasis border-b-2 border-white hover:bg-controls-tertiary"
    } else {
        "flex-1 h-12 transition-colors text-elements-lowEmphasis hover:bg-controls-tertiary"
    };

    rsx! {
        Row {
            class: "w-full",
            button {
                class: "{tokens_class}",
                onclick: move |_| tab.set(WalletTab::Tokens),
                "Balances"
            }
            button {
                class: "{liquidity_class}",
                onclick: move |_| tab.set(WalletTab::Liquidity),
                "Liquidity"
            }
        }
    }
}
