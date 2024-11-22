use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;

use crate::{
    components::*, hooks::{Asset, ASSETS}, route::Route
};
#[component]
pub fn Market(market: String) -> Element {
    let asset = ASSETS.get(&market).expect("Asset not found");
    rsx! {
        Row {
            class: "w-full pb-20 sm:pb-16 md:pr-8",
            gap: 4,
            Col {
                class: "w-full",
                gap: 8,
                Col {
                    class: "w-full px-5 md:pl-8 md:pr-0",
                    gap: 4,
                    Header {
                        asset: asset.clone()
                    },
                    PriceChart {
                        asset: asset.clone()
                    },
                    Actions {
                        asset: asset.clone()
                    },
                },
                span {
                    class: "px-5 md:px-8",
                    Stats {
                        asset: asset.clone()
                    }
                }
                TransactionTable {
                    asset: asset.clone()
                }
            }
            span {
                class: "hidden lg:flex mt-24",
                SwapForm {
                    mint_a: asset.mint,
                    mint_b: Pubkey::new_unique(),
                }
            }
        }
    }
}

#[component]
fn Header(asset: Asset) -> Element {
    rsx! {
        Row {
            class: "justify-between h-10",
            Row {
                gap: 2,
                img {
                    class: "w-10 h-10 rounded-full border border-gray-800 -ml-1 my-auto",
                    src: "{asset.image}",
                }
                span {
                    class: "font-wide text-2xl sm:text-3xl font-semibold my-auto",
                    "{asset.ticker}"
                }
            }
            Row {
                gap: 2,
                OrePrice {
                    ui_amount_string: "1.042",
                }
                span {
                    class: "font-medium text-green-500 text-sm mt-auto mb-2 sm:mb-1.5",
                    "0.2%"
                }
            }
        }
    }
}

#[component]
fn Actions(asset: Asset) -> Element {
    rsx! {
        div {
            class: "flex flex-col-reverse sm:flex-row gap-4",
            About {
                asset: asset.clone()
            }
            Row {
                class: "lg:hidden",
                gap: 2,
                SocialButtons { asset: asset.clone() }
                SwapButton { asset: asset.clone() }
            }
        }
    }
}

#[component]
fn SwapButton(asset: Asset) -> Element {
    rsx! {
        Link {
            to: Route::Swap {},
            class: "flex controls-primary h-10 rounded-full w-full sm:w-40",
            span {
                class: "mx-auto my-auto",
                "Swap"
            }
        }
    }
}

#[component]
fn SocialButtons(asset: Asset) -> Element {
    rsx! {
        Row {
            gap: 2,
            a {
                href: "{asset.twitter}",
                target: "_blank",
                class: "flex controls-secondary h-10 w-10 rounded-full shrink-0",
                XIcon {
                    class: "w-4 h-4 mx-auto my-auto"
                }
            }
            a {
                href: "{asset.homepage}",
                target: "_blank",
                class: "flex controls-secondary h-10 w-10 rounded-full shrink-0", 
                GlobeIcon {
                    class: "w-5 h-5 mx-auto my-auto"
                }
            }
        }
    }
}
#[component]
fn About(asset: Asset) -> Element {
    rsx! {
        Col {
            gap: 2,
            Subheading {
                title: "About".to_string(),
            }
            span {
                class: "text-elements-lowEmphasis w-full",
                "{asset.description}"
            }
        }
    }
}

#[component]
fn Stats(asset: Asset) -> Element {
    rsx! {
        Col {
            gap: 2,
            Subheading {
                title: "Stats".to_string(),
            }
            Col {
                gap: 1,
                div {
                    class: "flex flex-col sm:flex-row gap-1",
                    StatValue {
                        title: "SUPPLY",
                        value: 123
                    }
                    StatValue {
                        title: "LIQUIDITY",
                        value: 123
                    }
                }
                div {
                    class: "flex flex-col sm:flex-row gap-1",
                    StatValue {
                        title: "VOLUME",
                        value: 123
                    }
                    StatValue {
                        title: "VALUATION",
                        value: 123
                    }
                }
            }
        }
    }
}

#[component]
fn StatValue(title: String, value: u64) -> Element {
    rsx! {
        Row {
            class: "justify-between elevated h-12 w-full px-4 rounded-sm",
            span {
                class: "font-medium my-auto text-xs sm:text-sm text-elements-lowEmphasis",
                "{title}"
            }
            span {
                class: "my-auto text-elements-midEmphasis",
                OreValueSmall {
                    ui_amount_string: "1.202"
                }
            }
        }
    }
}

#[component]
fn PriceChart(asset: Asset) -> Element {
    rsx! {
        Col {
            gap: 3,
            Row {
                class: "text-sm text-elements-midEmphasis",
                TimeFrameButton {
                    title: "1D"
                }
                TimeFrameButton {
                    title: "1W"
                }
                TimeFrameButton {
                    title: "1M"
                }
                TimeFrameButton {
                    title: "1Y"
                }
                TimeFrameButton {
                    title: "All"
                }
            }
            div {
                class: "flex w-full h-80 elevated rounded",
                span {
                    class: "text-xs mx-auto my-auto",
                    "Chart goes here"
                }
            }
        }
    }
}

#[component]
fn TimeFrameButton(title: String) -> Element {
    rsx! {
        button {
            class: "py-1 w-10 rounded text-center transition text-elements-lowEmphasis hover:bg-controls-tertiaryHover hover:text-elements-midEmphasis",
            "{title}"
        }
    }
}

#[component]
fn TransactionTable(asset: Asset) -> Element {
    rsx! {
        Col {
            gap: 2,
            Subheading {
                class: "px-5 md:pl-8 md:pr-0",
                title: "Transactions",
            }
            Table {
                header: rsx! {
                    TableHeader {
                        left: "Time",
                        right_1: "Amount",
                        right_2: "By"
                    }
                },
                rows: rsx! {
                    // TODO: Replace with actual transaction data
                    TransactionRow {
                        timestamp: "3m ago",
                        amount: "12.5",
                        by: "Hf12...3x9k",
                        asset_image: asset.image.clone()
                    }
                    TransactionRow {
                        timestamp: "6m ago",
                        amount: "5.2",
                        by: "Kp98...2m4j",
                        asset_image: asset.image.clone()
                    }
                }
            }
        }
    }
}
#[component]
fn TransactionRow(
    timestamp: String,
    amount: String,
    by: String,
    asset_image: String
) -> Element {
    rsx! {
        TableRowLink {
            to: Route::Pay {},
            left: rsx! {
                span {
                    class: "whitespace-nowrap",
                    "{timestamp}"
                }
            },
            right_1: rsx! {
                Row {
                    class: "gap-2",
                    TokenValueSmall {
                        amount: amount,
                        image: asset_image
                    }
                    span {
                        class: "text-elements-lowEmphasis",
                        "→"
                    }
                    OreValueSmall {
                        ui_amount_string: "1.202".to_string()
                    }
                }
            },
            right_2: rsx! {
                span {
                    class: "text-elements-highEmphasis",
                    "{by}"
                }
            }
        }
    }
}
