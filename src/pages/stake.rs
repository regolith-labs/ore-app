use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;

use crate::{
    components::{CircleStackIcon, OreValue, OreValueSmall, PlusIcon},
    hooks::use_ore_balance,
    route::Route,
};

pub fn Stake() -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-8 w-full pb-20 sm:pb-16",
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
            PlusIcon {
                class: "h-4 w-4 mx-auto my-auto"
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
    let listed_liquidity = vec![
        Liquidity {
            name: "SOL-ORE".to_string(),
            mint: Pubkey::new_unique(),
            lp: Pubkey::new_unique(),
            image: "https://upload.wikimedia.org/wikipedia/en/b/b9/Solana_logo.png".to_owned(),
        },
        Liquidity {
            name: "USDC-ORE".to_string(),
            mint: Pubkey::new_unique(),
            lp: Pubkey::new_unique(),
            image: "https://cdn.prod.website-files.com/66327d2c71b7019a2a9a1b62/667454fd94c7f58e94f4a009_USDC-webclip-256x256.png"
                .to_owned(),
        },
        Liquidity {
            name: "ISC-ORE".to_string(),
            mint: Pubkey::new_unique(),
            lp: Pubkey::new_unique(),
            image: "https://raw.githubusercontent.com/theISCTeam/isc_meta/master/logo.png"
                .to_owned(),
        },
        Liquidity {
            name: "MOBILE-ORE".to_string(),
            mint: Pubkey::new_unique(),
            lp: Pubkey::new_unique(),
            image: "https://shdw-drive.genesysgo.net/6tcnBSybPG7piEDShBcrVtYJDPSvGrDbVvXmXKpzBvWP/mobile.png".to_owned(),
        },
        Liquidity {
            name: "HONEY-ORE".to_string(),
            mint: Pubkey::new_unique(),
            lp: Pubkey::new_unique(),
            image: "https://hivemapper-marketing-public.s3.us-west-2.amazonaws.com/Hivemapper_HONEY_token.png".to_owned(),
        },
    ];

    rsx! {
        div {
            class: "flex flex-col sm:mx-5",
            LiquidityTableHeader {}
            for liquidity in listed_liquidity {
                LiquidityRow {
                    liquidity: liquidity
                }
            }
        }
    }
}

fn LiquidityTableHeader() -> Element {
    rsx! {
        div {
            class: "flex flex-row h-8 h-10 px-5 sm:px-3 justify-between font-medium text-xs sm:text-sm text-gray-700",
            span {
                class: "my-auto",
                "Pair"
            }
            div {
                class: "flex flex-row text-right",
                span {
                    class: "my-auto w-28 sm:w-40",
                    "Liquidity"
                }
                span {
                    class: "my-auto w-28 sm:w-40",
                    "Volume"
                }
            }
        }
    }
}

#[component]
fn LiquidityRow(liquidity: Liquidity) -> Element {
    rsx! {
        Link {
            to: Route::Pair { pair: liquidity.name.clone() },
            class: "flex flex-row w-full px-5 sm:px-3 py-4 justify-between transition sm:rounded-md hover:bg-gray-900 hover:cursor-pointer",
            div {
                class: "flex flex-row gap-4",
                div {
                    class: "flex flex-row gap-0",
                    img {
                        class: "w-10 h-10 my-auto bg-gray-900 rounded-full border border-gray-800",
                        src: "{liquidity.image}"
                    }
                    img {
                        class: "w-10 h-10 -ml-2 my-auto bg-gray-900 rounded-full border border-gray-800",
                        src: "icon.png"
                    }
                }
                div {
                    class: "flex flex-col my-auto",
                    span {
                        class: "font-medium",
                        "{liquidity.name}"
                    }
                    span {
                        class: "font-medium text-gray-700 h-5 text-sm",
                        "0"
                    }
                }
            }
            div {
                class: "flex flex-row text-right my-auto",
                span {
                    class: "flex w-28 sm:w-40 justify-end",
                    OreValueSmall {
                        ui_amount_string: "4209.202"
                    }
                }
                span {
                    class: "flex w-28 sm:w-40 justify-end",
                    OreValueSmall {
                        ui_amount_string: "602.204"
                    }
                }
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Liquidity {
    name: String,
    mint: Pubkey,
    lp: Pubkey,
    image: String,
}
