use dioxus::prelude::*;

use crate::{components::OreValue, hooks::use_ore_balance};

pub fn Mine() -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-8 w-screen px-5 sm:px-8",
            span {
                class: "sm:hidden font-wide text-2xl font-semibold",
                "Mine"
            }
            Yield {}
        }
    }
}

fn Yield() -> Element {
    let balance = use_ore_balance();
    rsx! {
        div {
            class: "flex flex-col gap-2 sm:gap-4",
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
            }
        }
    }
}
