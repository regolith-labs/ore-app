use dioxus::prelude::*;

use crate::{
    components::{CircleStackIcon, Col, OreValue, QrCodeIcon, Row},
    gateway::{UiTokenAmount, GatewayResult},
    hooks::use_ore_balance,
    route::Route,
};

pub fn Balance() -> Element {
    let balance = use_ore_balance();
    rsx! {
        Col {
            class: "w-full sm:gap-4",
            gap: 2,
            // span {
            //     class: "font-medium text-xs sm:text-sm text-gray-700",
            //     "Balance"
            // }
            Row {
                class: "md:flex-col md:gap-2 w-full justify-between md:justify-start align-top",
                gap: 0,
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
                PayButton {}
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
                Col {
                    gap: 2,
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

fn PayButton() -> Element {
    rsx! {
        Link {
            to: Route::Pay {},
            class: "flex flex-row h-10 w-min text-elements-lowEmphasis transition hover:bg-controls-secondaryHover hover:text-elements-highEmphasis rounded-full px-4 gap-2 -mr-2 md:-ml-3",
            QrCodeIcon {
                class: "h-6 w-6 mx-auto my-auto"
            }
            span {
                class: "my-auto",
                "Pay"
            }
        }
    }
}

pub fn Yield() -> Element {
    let balance = use_ore_balance();
    rsx! {
        Col {
            class: "w-full sm:gap-4",
            gap: 2,
            span {
                class: "font-medium text-xs sm:text-sm text-gray-700 md:ml-auto",
                "Yield"
            }
            Row {
                class: "md:flex-col sm:gap-2 justify-between md:justify-start align-top md:ml-auto",
                gap: 0,
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
                                // class: "text-elements-gold",
                                ui_amount_string: "0.000"
                            }
                        }
                    }
                }
                // ClaimButton {}
            }
        }
    }
}
