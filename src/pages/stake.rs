use dioxus::prelude::*;

use crate::components::*;

pub fn Stake() -> Element {
    rsx! {
        Col {
            class: "w-full h-full pb-20 sm:pb-16",
            gap: 16,
            Heading {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                title: "Stake",
                subtitle: "Provide liquidity for traders and earn yield."
            }
            AccountSummary {}
            StakeTable {}
        }
    }
}

fn AccountSummary() -> Element {
    rsx! {
        Col {
            class: "mx-auto w-full px-5 sm:px-8 justify-between",
            span {
                class: "text-elements-highEmphasis font-semibold text-2xl mb-6",
                "Account"
            }
            Row {
                class: "w-full justify-between px-0 sm:px-2",
                Col {
                    gap: 4,
                    span {
                        class: "text-elements-lowEmphasis font-medium",
                        "Net deposits"
                    }
                    OreValue {
                        ui_amount_string: "2.324330".to_string(),
                    }
                }
                Row {
                    Col {
                        class: "min-w-56",
                        gap: 4,
                        span {
                            class: "text-elements-lowEmphasis font-medium text-right",
                            "Net liquidity"
                        }
                        UsdValue {
                            class: "ml-auto",
                            amount: "1230.12".to_string(),
                        }
                    }
                    Col {
                        class: "min-w-56",
                        gap: 4,
                        span {
                            class: "text-elements-lowEmphasis font-medium text-right",
                            "Net yield"
                        }
                        OreValue {
                            class: "text-elements-gold ml-auto",
                            ui_amount_string: "1.213".to_string(),
                        }
                    }
                }
            }
        }
    }
}

fn _YieldOverview() -> Element {
    // TODO Get all stake accounts
    // TODO Calculate total claimable yield
    // TODO Provide claim button
    rsx! {
        Row {
            class: "mx-5 sm:mx-8 py-8 justify-between",
            // div {
            //     class: "flex w-full",
            //     OreValue {
            //         class: "mx-auto my-auto",
            //         ui_amount_string: "2.324330".to_string(),
            //     }
            // }
            div {
                class: "flex w-full",
                span {
                    class: "text-elements-midEmphasis font-bold text-2xl sm:text-3xl my-auto mx-auto",
                    "0.04%"
                }   
            }
            div {
                class: "flex w-full",
                OreValueGold {
                    class: "mx-auto my-auto",
                    ui_amount_string: "2.324330".to_string(),
                }   
            }
        }
    }
}
