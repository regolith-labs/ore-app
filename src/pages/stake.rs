use dioxus::prelude::*;

use crate::{
    components::*, hooks::{use_assets, Asset}, route::Route
};

pub fn Stake() -> Element {
    rsx! {
        Col {
            class: "w-full pb-20 sm:pb-16",
            gap: 8,
            Header {}
            Col {
                class: "md:flex-row md:gap-0 px-5 sm:px-8",
                gap: 8,
                Balance {}
                Yield {}
            }
            LiquidityTable {}
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
            class: "h-10 controls-primary rounded-full px-4 gap-2",
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
    let listed_assets = use_assets();
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
                    match listed_assets.cloned() {
                        None => rsx! {
                            TableRowLink {
                                to: Route::Pair { pair: "Loading".to_string() },
                                left: rsx! { div { class: "h-10 w-48 loading rounded" } },
                                right_1: rsx! { div { class: "h-10 w-24 loading rounded" } },
                                right_2: rsx! { div { class: "h-10 w-24 loading rounded" } },
                                right_3: rsx! { div { class: "h-10 w-24 loading rounded" } }
                            }
                        },
                        Some(assets) => {
                            rsx! {
                                for asset in assets {
                                    LiquidityRow { asset: asset }
                                }
                            }
                        }
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
            to: Route::Pair { pair: asset.ticker.clone() },
            left: rsx! {
                Row {
                    class: "grow shrink-0",
                    gap: 4,
                    Row {
                        class: "shrink-0",
                        img {
                            class: "w-10 h-10 shrink-0 my-auto rounded-full border border-gray-800",
                            src: "{asset.image}"
                        }
                        img {
                            class: "w-10 h-10 shrink-0 -ml-2 my-auto rounded-full border border-gray-800",
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

