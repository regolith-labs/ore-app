use dioxus::prelude::*;

use crate::{
    components::{CircleStackIcon, EyeDropperIcon, OreValue},
    hooks::use_ore_balance,
    route::Route,
};

pub fn Stake() -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-8 w-screen",
            span {
                class: "flex flex-row justify-between sm:hidden mx-5 sm:mx-8 h-10 font-wide text-2xl font-semibold",
                span {
                    class: "my-auto",
                    "Stake"
                }
                DepositButton {}
            }
            StakingYield {}
            LiquidityTable {}
        }
    }
}

fn StakingYield() -> Element {
    let balance = use_ore_balance();
    rsx! {
        div {
            class: "flex flex-col gap-2 sm:gap-4 px-5 sm:px-8",
            span {
                class: "font-medium text-xs sm:text-sm text-gray-700",
                "Yield"
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
                    Some(_balance) => {
                        rsx! {
                            OreValue {
                                ui_amount_string: "0.000"
                            }
                        }
                    }
                }
                div {
                    class: "flex flex-row gap-4",
                    ClaimButton {}
                    span {
                        class: "hidden sm:flex",
                        DepositButton {}
                    }
                }
            }
        }
    }
}

fn DepositButton() -> Element {
    rsx! {
        Link {
            to: Route::Deposit {},
            class: "controls-square controls-primary",
            EyeDropperIcon {
                class: "h-5 w-5 mx-auto my-auto"
            }
        }
    }
}

fn ClaimButton() -> Element {
    rsx! {
        Link {
            to: Route::Pay {},
            class: "controls-square controls-secondary",
            CircleStackIcon {
                class: "h-5 w-5 mx-auto my-auto"
            }
        }
    }
}

fn LiquidityTable() -> Element {
    // TODO Read from config file

    rsx! {
        div {
            class: "flex flex-col sm:mx-5",
            LiquidityTableHeader {}
            // for asset in listed_assets {
            //     AssetRow {
            //         asset: asset
            //     }
            // }
        }
    }
}

fn LiquidityTableHeader() -> Element {
    rsx! {
        div {
            class: "flex flex-row h-8 h-10 px-5 sm:px-3 justify-between font-medium text-xs sm:text-sm text-gray-700",
            span {
                class: "my-auto",
                "Liquidity"
            }
            div {
                class: "flex flex-row text-right",
                span {
                    class: "my-auto w-28",
                    "Fees"
                }
                span {
                    class: "my-auto w-28",
                    "Volume"
                }
            }
        }
    }
}
