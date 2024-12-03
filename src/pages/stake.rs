use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;

use crate::{
    components::*, hooks::{Asset, ASSETS}, route::Route
};

pub fn Stake() -> Element {
    rsx! {
        Col {
            class: "w-full h-full pb-20 sm:pb-16",
            gap: 4,
            Heading {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                title: "Stake",
                subtitle: "Provide liquidity for traders and earn yield."
            }
            StakeForm {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                mint: Pubkey::new_unique(),
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
                "Stake"
            }
            DepositButton {}
        }
    }
}

fn DepositButton() -> Element {
    rsx! {
        Link {
            to: Route::Deposit {},
            class: "h-10 controls-primary rounded-full px-4 gap-2 -mr-2",
            PlusIcon {
                class: "h-4 w-4 mx-auto my-auto"
            }
            span {
                class: "my-auto",
                "Deposit"
            }
        }
    }
}
fn LiquidityTable() -> Element {
    let listed_assets = ASSETS.values().collect::<Vec<_>>();
    rsx! {
        Col {
            gap: 2,
            Table {
                header: rsx! {
                    TableHeader {
                        left: "Pair",
                        right_1: "Liquidity", 
                        right_2: "Volume",
                        right_3: "Yield"
                    }
                },
                rows: rsx! {
                    for asset in listed_assets {
                        LiquidityRow { asset: asset.clone() }
                    }
                }
            }
        }
    }
}

#[component]
fn LiquidityRow(asset: Asset) -> Element {
    rsx! {
        TableRowLink {
            to: Route::Pair { 
                pair: format!("{}-ORE", asset.ticker.clone())
            },
            left: rsx! {
                Row {
                    class: "grow shrink-0",
                    gap: 4,
                    Row {
                        class: "shrink-0",
                        img {
                            class: "w-10 h-10 shrink-0 my-auto rounded-full",
                            src: "{asset.image}"
                        }
                        img {
                            class: "w-10 h-10 shrink-0 -ml-2 my-auto rounded-full",
                            src: "icon.png"
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
            right_1: rsx! {
                OreValueSmall {
                    ui_amount_string: "4209.202"
                }
            },
            right_2: rsx! {
                OreValueSmall {
                    ui_amount_string: "602.204"
                }
            },
            right_3: rsx! {
                span {
                    class: "text-elements-gold",
                    OreValueSmall {
                        ui_amount_string: "2.054"
                    }
                }
            },
        }
    }
}

