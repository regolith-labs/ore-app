use dioxus::prelude::*;
use solana_extra_wasm::program::spl_token::amount_to_ui_amount;

use crate::{
    components::{Appearance, OreIcon, QrCodeIcon},
    hooks::{
        use_appearance, use_escrow_proof, use_ore_balance, use_ore_v1_balance, use_proof,
        use_wallet_adapter::{use_wallet_adapter, WalletAdapter},
    },
    route::Route,
};

pub fn Balance() -> Element {
    let balance = use_ore_balance();
    let wallet_adapter = use_wallet_adapter();

    if let Some(balance) = balance.cloned() {
        let amount = balance
            .map(|b| b.real_number_string_trimmed())
            .unwrap_or_else(|_| "0.00".to_owned());

        return rsx! {
            div {
                class: "flex flex-row w-full min-h-16 rounded justify-between",
                div {
                    class: "flex flex-col grow gap-2 sm:gap-4 max-w-full",
                    h2 {
                        class: "text-lg sm:text-xl md:text-2xl font-bold",
                        "Balance"
                    }
                    div {
                        class: "flex flex-row grow justify-between max-w-full",
                        div {
                            class: "flex flex-row my-auto gap-2.5 md:gap-4 truncate",
                            OreIcon {
                                class: "my-auto w-7 h-7 sm:w-8 sm:h-8 md:w-10 md:h-10 shrink-0"
                            }
                            h2 {
                                class: "text-3xl sm:text-4xl md:text-5xl",
                                "{amount}"
                            }
                        }
                        if let WalletAdapter::Connected(_) = *wallet_adapter.read() {
                            div {
                                class: "flex flex-row gap-4",
                                QrButton {}
                                SendButton {}
                            }
                        }
                    }
                    OreV1Balance {}
                    StakeBalance {}
                }
            }
        };
    }

    rsx! {
        div {
            class: "flex flex-row w-full min-h-24 grow loading rounded",
        }
    }
}

pub fn StakeBalance() -> Element {
    let proof = use_proof();
    let escrow_proof = use_escrow_proof();

    if let Some(Ok(proof)) = *escrow_proof.read() {
        return rsx! {
            div {
                class: "flex flex-row grow justify-between mt-4",
                div {
                    class: "flex flex-col gap-2",
                    p {
                        class: "font-medium text-sm text-gray-300",
                        "Stake"
                    }
                    div {
                        class: "flex flex-row gap-2",
                        OreIcon {
                            class: "my-auto w-4 h-4"
                        }
                        p {
                            class: "font-semibold",
                            "{amount_to_ui_amount(proof.balance, ore_api::consts::TOKEN_DECIMALS)}"
                        }
                    }
                }
                div {
                    class: "mt-auto flex flex-row gap-1 sm:gap-2 -mb-2",
                    ClaimButton {}
                }
            }
        };
    } else if let Some(Ok(proof)) = *proof.read() {
        return rsx! {
            div {
                class: "flex flex-row grow justify-between mt-4",
                div {
                    class: "flex flex-col gap-2",
                    p {
                        class: "font-medium text-sm text-gray-300",
                        "Stake"
                    }
                    div {
                        class: "flex flex-row gap-2",
                        OreIcon {
                            class: "my-auto w-4 h-4"
                        }
                        p {
                            class: "font-semibold",
                            "{amount_to_ui_amount(proof.balance, ore_api::consts::TOKEN_DECIMALS)}"
                        }
                    }
                }
                div {
                    class: "mt-auto flex flex-row gap-1 sm:gap-2 -mb-2",
                    ClaimButton {}
                    StakeButton {}
                }
            }
        };
    }

    rsx! {
        // div {
            // class: "flex flex-row w-full min-h-20 grow loading rounded",
        // }
    }
}

pub fn OreV1Balance() -> Element {
    let balance = use_ore_v1_balance();

    if let Some(balance) = balance.cloned() {
        if let Ok(balance) = balance {
            if let Some(amountf64) = balance.ui_amount {
                if amountf64.gt(&0f64) {
                    return rsx! {
                        div {
                            class: "flex flex-row grow justify-between mt-4",
                            div {
                                class: "flex flex-col gap-2",
                                p {
                                    class: "font-medium text-sm text-gray-300",
                                    "OREv1"
                                }
                                div {
                                    class: "flex flex-row gap-2",
                                    OreIcon {
                                        class: "my-auto w-4 h-4"
                                    }
                                    p {
                                        class: "font-semibold",
                                        "{balance.ui_amount_string}"
                                    }
                                }
                            }
                            div {
                                class: "mt-auto flex flex-row gap-1 sm:gap-2 -mb-2",
                                UpgradeButton {}
                            }
                        }
                    };
                }
            }
        }
    }

    return rsx! {};
}

#[component]
pub fn SendButton(to: Option<String>) -> Element {
    rsx! {
        Link {
            to: Route::Send { to: to.clone().unwrap_or("".to_string()) },
            class: "flex h-12 w-12 my-auto rounded-full justify-center text-2xl font-bold transition-all bg-black text-white hover:shadow hover:scale-110 dark:bg-white dark:text-black",
            span {
                class: "my-auto bg-transparent",
                "↑"
            }
        }
    }
}

#[component]
pub fn QrButton(to: Option<String>) -> Element {
    let appearance = use_appearance();
    let button_color = match *appearance.read() {
        Appearance::Light => "text-gray-300 hover:text-black ",
        Appearance::Dark => "text-gray-300 hover:text-white ",
    };
    rsx! {
        Link {
            to: Route::Pay {},
            class: "flex h-12 w-12 my-auto rounded-full justify-center text-2xl font-bold transition-all {button_color} hover-100 active-200",
            QrCodeIcon {
                class: "w-6 h-6 my-auto",
            }
        }
    }
}

pub fn ClaimButton() -> Element {
    let appearance = use_appearance();
    let button_color = match *appearance.read() {
        Appearance::Light => "text-gray-300 hover:text-black ",
        Appearance::Dark => "text-gray-300 hover:text-white ",
    };
    rsx! {
        Link {
            class: "flex transition transition-colors font-semibold text-sm px-3 h-10 rounded-full hover-100 active-200 {button_color}",
            to: Route::Claim {},
            span {
                class: "my-auto",
                "Claim"
            }
        }
    }
}

pub fn StakeButton() -> Element {
    let appearance = use_appearance();
    let button_color = match *appearance.read() {
        Appearance::Light => "text-gray-300 hover:text-black ",
        Appearance::Dark => "text-gray-300 hover:text-white ",
    };
    rsx! {
        Link {
            class: "flex transition transition-colors font-semibold text-sm px-3 h-10 rounded-full hover-100 active-200 {button_color}",
            to: Route::Stake {},
            span {
                class: "my-auto",
                "Stake"
            }
        }
    }
}

pub fn UpgradeButton() -> Element {
    let appearance = use_appearance();
    let button_color = match *appearance.read() {
        Appearance::Light => "text-gray-300 hover:text-black ",
        Appearance::Dark => "text-gray-300 hover:text-white ",
    };
    rsx! {
        Link {
            class: "flex transition transition-colors font-semibold text-sm px-3 h-10 rounded-full hover-100 active-200 {button_color}",
            to: Route::Upgrade {},
            span {
                class: "my-auto",
                "Upgrade"
            }
        }
    }
}
