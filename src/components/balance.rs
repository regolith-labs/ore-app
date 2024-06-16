use dioxus::prelude::*;
use solana_extra_wasm::program::spl_token::amount_to_ui_amount;

use crate::{
    components::OreIcon,
    gateway::AsyncResult,
    hooks::{use_ore_balance, use_proof},
    route::Route,
};

pub fn Balance() -> Element {
    let balance = use_ore_balance();
    rsx! {
        div {
            class: "flex flex-row w-full min-h-16 rounded justify-between",
            match balance.read().clone() {
                AsyncResult::Ok(b) => {
                    rsx! {
                        div {
                            class: "flex flex-col grow gap-2 sm:gap-4",
                            h2 {
                                class: "text-lg sm:text-xl md:text-2xl font-bold",
                                "Balance"
                            }
                            div {
                                class: "flex flex-row grow justify-between",
                                div {
                                    class: "flex flex-row my-auto gap-2.5 md:gap-4",
                                    OreIcon {
                                        class: "my-auto w-7 h-7 sm:w-8 sm:h-8 md:w-10 md:h-10"
                                    }
                                    h2 {
                                        class: "text-3xl sm:text-4xl md:text-5xl",
                                        "{b.real_number_string_trimmed()}"
                                    }
                                }
                                SendButton {}
                            }
                            StakeBalance {}
                        }
                    }
                }
                _ => {
                    rsx! {
                        div {
                            class: "flex flex-row grow loading rounded",
                        }
                    }
                }
            }
        }
    }
}

pub fn StakeBalance() -> Element {
    let proof = use_proof();
    if let AsyncResult::Ok(proof) = *proof.read() {
        if proof.balance.gt(&0) {
            return rsx! {
                div {
                    class: "flex flex-row grow justify-between mt-4 -mr-2",
                    div {
                        class: "flex flex-col gap-2",
                        p {
                            class: "font-medium text-xs text-gray-300",
                            "Staking balance"
                        }
                        div {
                            class: "flex flex-row gap-2",
                            OreIcon {
                                class: "my-auto w-4 h-4"
                            }
                            p {
                                class: "font-semibold",
                                "{amount_to_ui_amount(proof.balance, ore::TOKEN_DECIMALS)}"
                            }
                        }
                    }
                    span {
                        class: "mt-auto",
                        ClaimButton {}
                    }
                }
            };
        }
    }

    None
}

#[component]
pub fn SendButton(to: Option<String>) -> Element {
    rsx! {
        Link {
            to: Route::Send { to: to.clone().unwrap_or("".to_string()) },
            class: "flex h-10 w-10 my-auto rounded-full justify-center text-2xl font-bold transition-all bg-black text-white hover:shadow hover:scale-110 dark:bg-white dark:text-black",
            span {
                class: "my-auto bg-transparent",
                "↑"
            }
        }
    }
}

pub fn ClaimButton() -> Element {
    rsx! {
        Link {
            class: "flex transition transition-colors font-semibold px-3 h-10 rounded-full hover-100 active-200",
            to: Route::Claim {},
            span {
                class: "my-auto",
                "Claim"
            }
        }
    }
}
