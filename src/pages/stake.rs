use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;

use crate::{
    components::{
        CircleStackIcon, OreValue, OreValueSmall, PlusIcon, Table, TableHeader, TableRowLink,
    },
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
        Table {
            TableHeader {
                left: "Pair",
                right: vec!["Liquidity".to_string(), "Volume".to_string()]
            }
            for liquidity in listed_liquidity {
                TableRowLink {
                    to: Route::Pair { pair: liquidity.name.clone() },
                    left: rsx! {
                        div {
                            class: "flex flex-row grow gap-4 w-48 shrink-0",
                            div {
                                class: "flex flex-row gap-0 shrink-0",
                                img {
                                    class: "w-10 h-10 shrink-0 my-auto rounded-full border border-gray-800",
                                    src: "{liquidity.image}"
                                }
                                img {
                                    class: "w-10 h-10 shrink-0 -ml-2 my-auto rounded-full border border-gray-800",
                                    src: "icon.png"
                                }
                            }
                            div {
                                class: "flex flex-col my-auto min-w-32 shrink-0",
                                span {
                                    class: "font-medium whitespace-nowrap",
                                    "{liquidity.name}"
                                }
                                span {
                                    class: "font-medium text-gray-700 h-5 text-sm",
                                    "0"
                                }
                            }
                        }
                    },
                    right: vec![
                        rsx! {
                            OreValueSmall {
                                ui_amount_string: "4209.202"
                            }
                        },
                        rsx! {
                            OreValueSmall {
                                ui_amount_string: "602.204"
                            }
                        }
                    ]
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
