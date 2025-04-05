use dioxus::prelude::*;

use crate::components::*;
use crate::hooks::{use_ore_holders, use_ore_market_cap, use_ore_supply};
use crate::utils::*;

use super::*;

pub fn TokenomicsContent() -> Element {
    rsx! {
        ContentSection {
            TokenomicsIntro {}
            TokenomicsHowItWorks {}
            TokenomicsSupplyChart {}
            TokenomicsKeyData {}
        }
    }
}

fn TokenomicsIntro() -> Element {
    rsx! {
        span {
            span {
                class: "font-semibold text-elements-highEmphasis",
                "ORE is a scarce digital commodity, mineable via proof-of-work on the Solana blockchain. "
            }
            "The total supply is capped at 5 million tokens, and emissions are automatically reduced by 10% every ~12 months."
        }
    }
}

fn TokenomicsHowItWorks() -> Element {
    rsx! {
        SectionTitle {
            "How it works"
        }
        BulletPointList {
            BulletPoint {
                "Token issuance is managed programmatically by a smart contract. Miners compete by expending physical energy to produce new units of the token."
            }
            BulletPoint {
                "The community can issue \"boost\" incentives via governance to allocate a portion of emissions towards funding growth initiatives such as marketing, development, liquidity, and more."
            }
            BulletPoint {
                "Mining and boosts work together to create a powerful and flexible distribution mechanism while simultaneously protecting holders from undue inflation."
            }
        }
    }
}

fn TokenomicsSupplyChart() -> Element {
    rsx! {
        SectionTitle {
            "Supply curve"
        }
        img {
            class: "relative w-full h-full object-contain z-10 pt-4 rounded-lg",
            src: asset!("/public/ore-supply-curve.png"),
            loading: "eager",
            decoding: "sync",
        }
    }
}

fn TokenomicsKeyData() -> Element {
    let holders = use_ore_holders();
    let market_cap = use_ore_market_cap();
    let supply = use_ore_supply();
    rsx! {
        SectionTitle {
            "Key metrics"
        }
        BulletPointList {
            TokenomicsDataPoint {
                label: "Max supply",
                OreValue {
                    ui_amount_string: "5000000",
                    with_decimal_units: false,
                    size: TokenValueSize::Small,
                }
            }
            TokenomicsDataPoint {
                label: "Current supply",
                if let Some(Ok(supply)) = supply.cloned() {
                    OreValue {
                        ui_amount_string: supply.ui_amount_string,
                        with_decimal_units: false,
                        size: TokenValueSize::Small,
                    }
                } else {
                    "–"
                }
            }
            TokenomicsDataPoint {
                label: "Emissions rate",
                "~1 ORE / min"
            }
            TokenomicsDataPoint {
                label: "Daily inflation",
                if let Some(Ok(supply)) = supply.cloned() {
                    if let Some(supply) = supply.ui_amount {
                        "{format_percentage(1440.0 / supply * 100.0)}"
                    } else {
                        "_"
                    }
                } else {
                    "–"
                }
            }
            TokenomicsDataPoint {
                label: "Market cap",
                if let Some(Ok(market_cap)) = market_cap.cloned() {
                    "${format_abbreviated_number(market_cap)}"
                } else {
                    "–"
                }
            }
            TokenomicsDataPoint {
                label: "Token holders",
                if let Some(Ok(holders)) = holders.cloned() {
                    "{format_abbreviated_number(holders as f64)}"
                } else {
                    "–"
                }
            }
        }
    }
}

#[component]
fn TokenomicsDataPoint(label: String, children: Element) -> Element {
    rsx! {
        Row {
            class: "w-full justify-between pt-4 text-base",
            span {
                class: "font-medium text-elements-lowEmphasis",
                "{label}"
            }
            span {
                class: "font-medium text-elements-highEmphasis",
                {children}
            }
        }
    }
}
