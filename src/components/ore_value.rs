use dioxus::prelude::*;
use ore_api::consts::TOKEN_DECIMALS;
use ore_boost_api::state::Stake;

use crate::{components::{Col, OreIcon, Row}, config::{LISTED_TOKENS, LISTED_TOKENS_BY_TICKER}, hooks::LiquidityPair};

#[component]
pub fn OreValue(ui_amount_string: String, class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    let units: Vec<_> = ui_amount_string.split('.').collect();
    let big_units = units[0];
    let small_units = units[1];

    // let is_thousands = big_units.len() > 3;
    // let k = if is_thousands { "k" } else { "" };
    // let big_units_display = if is_thousands {
    //     match big_units.char_indices().rev().nth(2) {
    //         Some((i, _)) => &big_units[..i],
    //         None => "",
    //     }
    // } else {
    //     big_units
    // };
    // let small_units_display: String = if is_thousands {
    //     big_units.chars().rev().take(3).collect()
    // } else {
    //     small_units.chars().take(2).collect()
    // };

    rsx! {
        Row {
            class: "sm:gap-3 h-10 w-min {class}",
            gap: 2,
            OreIcon {
                class: "h-6 w-6 sm:h-8 sm:w-8 my-auto"
            }
            Row {
                class: "my-auto",
                span {
                    class: "mt-auto font-semibold text-2xl sm:text-3xl",
                    "{big_units}"
                }
                span {
                    class: "mt-auto font-semibold text-xl sm:text-2xl text-elements-lowEmphasis",
                    ".{small_units}"
                }
            }
        }
    }
}

#[component]
pub fn OreValueGold(ui_amount_string: String, class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    let units: Vec<_> = ui_amount_string.split('.').collect();
    let big_units = units[0];
    let small_units = units[1];
    rsx! {
        Row {
            class: "sm:gap-3 h-10 w-min {class}",
            gap: 2,
            OreIcon {
                class: "h-6 w-6 sm:h-8 sm:w-8 my-auto text-elements-gold"
            }
            Row {
                class: "my-auto",
                span {
                    class: "mt-auto font-bold text-2xl sm:text-3xl text-elements-gold",
                    "{big_units}"
                }
                span {
                    class: "mt-auto font-bold text-xl sm:text-2xl text-elements-gold opacity-50",
                    ".{small_units}"
                }
            }
        }
    }
}

#[component]
pub fn OreValueWhole(ui_amount_string: String, class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    let units: Vec<_> = ui_amount_string.split('.').collect();
    let big_units = units[0];
    let big_units = format_with_commas(big_units);
    rsx! {
        Row {
            class: "sm:gap-3 h-10 w-min {class}",
            gap: 2,
            OreIcon {
                class: "h-5 w-5 sm:h-8 sm:w-8 my-auto"
            }
            Row {
                class: "my-auto",
                span {
                    class: "mt-auto font-semibold text-2xl sm:text-3xl",
                    "{big_units}"
                }
            }
        }
    }
}

#[component]
pub fn OrePrice(ui_amount_string: String, change: Option<f64>) -> Element {
    let units: Vec<_> = ui_amount_string.split('.').collect();
    let big_units = units[0];
    let small_units = units[1];
    rsx! {
        Row {
            class: "gap-2 w-min",
            OreIcon {
                class: "h-4 w-4 sm:h-6 sm:w-6 my-auto"
            }
            Row {
                class: "my-auto",
                span {
                    class: "mt-auto font-semibold text-lg sm:text-xl",
                    "{big_units}.{small_units}"
                }
            }
            if let Some(change) = change {
                span {
                    class: "font-medium text-green-500 text-sm mt-auto mb-2 sm:mb-[7px]",
                    "{change:.2}%"
                }
            }
        }
    }
}

#[component]
pub fn OreValueSmallWhole(class: Option<String>, ui_amount_string: String) -> Element {
    let class: String = class.unwrap_or("".to_string());
    let units: Vec<_> = ui_amount_string.split('.').collect();
    let big_units = units[0];
    let big_units = format_with_commas(big_units);
    rsx! {
        Row {
            class: "gap-1.5 w-min {class}",
            OreIcon {
                class: "h-4 w-4 my-auto"
            }
            Row {
                class: "font-medium my-auto",
                span {
                    class: "mt-auto",
                    "{big_units}"
                }
            }
        }
    }
}

#[component]
pub fn OreValueSmallAbbreviated(class: Option<String>, ui_amount_string: String) -> Element {
    let class: String = class.unwrap_or("".to_string());
    let units: Vec<_> = ui_amount_string.split('.').collect();
    let big_units = units[0];
    let small_units = units[1];

    let is_thousands = big_units.len() > 3;
    let k = if is_thousands { "k" } else { "" };
    let big_units_display = if is_thousands {
        match big_units.char_indices().rev().nth(2) {
            Some((i, _)) => &big_units[..i],
            None => "",
        }
    } else {
        big_units
    };
    let small_units_display: String = if is_thousands {
        big_units.chars().rev().take(3).into_iter().collect()
    } else {
        small_units.chars().take(3).collect()
    };

    rsx! {
        Row {
            class: "gap-1.5 w-min {class}",
            OreIcon {
                class: "h-4 w-4 my-auto"
            }
            Row {
                class: "font-medium my-auto",
                span {
                    class: "mt-auto",
                    "{big_units_display}.{small_units_display}{k}"
                }
            }
        }
    }
}

#[component]
pub fn OreValueSmall(
    class: Option<String>, 
    ui_amount_string: String, 
    small_units: Option<bool>,
    abbreviated: Option<bool>
) -> Element {
    let class: String = class.unwrap_or("".to_string());
    let display_small_units = small_units.unwrap_or(false);
    let abbreviated = abbreviated.unwrap_or(false);
    let units: Vec<_> = ui_amount_string.split('.').collect();
    let big_units = units[0];
    let small_units = units[1];
    // let big_units_display = format_with_commas(big_units);
    // let small_units_display = small_units.trim_end_matches('0');

    // Abbreviate the value if the abbreviated flag is true
    let (big_units_display, small_units_display) = if abbreviated {
        if big_units == "0" {
            // For values < 1, show 3 significant digits total
            let mut significant_digits = String::new();
            let mut non_zero_digits = 0;
            for c in small_units.chars() {
                significant_digits.push(c);
                if c != '0' {
                    non_zero_digits += 1;
                    if non_zero_digits >= 3 {
                        break;
                    }
                }
            }
            (
                "0".to_string(),
                significant_digits
            )
        } else {
            // For values >= 1, show all big units and first 3 small units
            (
                format_with_commas(big_units),
                small_units.chars().take(3).collect()
            )
        }
    } else {
        (
            format_with_commas(big_units),
            small_units.trim_end_matches('0').to_string()
        )
    };

    rsx! {
        Row {
            class: "gap-1.5 w-min {class}",
            OreIcon {
                class: "h-4 w-4 my-auto"
            }
            Row {
                class: "font-medium my-auto",
                span {
                    class: "mt-auto",
                    "{big_units_display}"
                }
                if display_small_units {
                    span {
                        class: "mt-auto font-medium opacity-50",
                        ".{small_units_display}"
                    }
                }
            }
        }
    }
}

#[component]
pub fn TokenValueSmall(class: Option<String>, amount: String, ticker: String, small_units: Option<bool>) -> Element {
    let class = class.unwrap_or("".to_string());
    let display_small_units = small_units.unwrap_or(false);
    let image = if let Some(token) = LISTED_TOKENS_BY_TICKER.get(&ticker) {
        token.image.clone()
    } else {
        "".to_string()
    };

    let units: Vec<_> = amount.split('.').collect();
    let big_units = units[0];
    let big_units = format_with_commas(big_units);
    let small_units = if units.len() > 1 { units[1] } else { "00" };

    rsx! {
        Row {
            class: "gap-1.5 {class}",
            img {
                class: "w-6 h-6 my-auto bg-gray-900 rounded-full border border-gray-800",
                src: "{image}"
            }
            span {
                class: "my-auto font-medium", 
                "{big_units}"
                if display_small_units {
                    span {
                        class: "mt-auto font-medium opacity-50",
                        ".{small_units}"
                    }
                }
            }
        }
    }
}

#[component]
pub fn UsdValueSmall(class: Option<String>, amount: String, small_units: Option<bool>) -> Element {
    let class = class.unwrap_or("".to_string());
    let display_small_units = small_units.unwrap_or(false);
    let units: Vec<_> = amount.split('.').collect();
    let big_units = units[0];
    let big_units = format_with_commas(big_units);
    let small_units = if units.len() > 1 { units[1] } else { "00" };
    rsx! {
        Row {
            class: "gap-1.5 {class}",
            span {
                class: "my-auto font-medium", 
                if display_small_units {
                    "${big_units}.{small_units[..2].to_string()}"
                } else {
                    "${big_units}"
                }
            }
        }
    }
}

#[component]
pub fn LiquidityPairValue(class: Option<String>, liquidity_pair: LiquidityPair, small_units: Option<bool>) -> Element {
    let class = class.unwrap_or("".to_string());
    let (ore_amount_f64, token_amount_f64, token_ticker) = if liquidity_pair.token_a.ticker == "ORE" {
        (liquidity_pair.balance_a_f64, liquidity_pair.balance_b_f64, liquidity_pair.token_b.ticker.clone())
    } else {
        (liquidity_pair.balance_b_f64, liquidity_pair.balance_a_f64, liquidity_pair.token_a.ticker.clone())
    };
    rsx! {
        Col {
            class: "gap-2 {class}",
            OreValueSmall {
                class: "ml-auto",
                ui_amount_string: ore_amount_f64.to_string(),
                small_units: small_units,
            }
            TokenValueSmall {
                class: "ml-auto",
                amount: token_amount_f64.to_string(),
                ticker: token_ticker.clone(),
                small_units: small_units,
            }
        }
    }
}


#[component]
pub fn LiquidityPairStakeValue(
    class: Option<String>, 
    shares: u64, 
    liquidity_pair: LiquidityPair, 
    small_units: Option<bool>
) -> Element {
    let class = class.unwrap_or("".to_string());

    let lp_share = shares as f64 / liquidity_pair.shares as f64;
    let stake_amount_a = liquidity_pair.balance_a_f64 * lp_share;
    let stake_amount_b = liquidity_pair.balance_b_f64 * lp_share;
    
    let (ore_amount_f64, token_amount_f64, token_ticker, token_decimals) = if liquidity_pair.token_a.ticker == "ORE" {
        (stake_amount_a, stake_amount_b, liquidity_pair.token_b.ticker.clone(), liquidity_pair.token_b.decimals)
    } else {
        (stake_amount_b, stake_amount_a, liquidity_pair.token_a.ticker.clone(), liquidity_pair.token_a.decimals)
    };

    rsx! {
        Col {
            class: "gap-2 {class}",
            OreValueSmall {
                class: "ml-auto",
                ui_amount_string: format!("{:.1$}", ore_amount_f64, TOKEN_DECIMALS as usize),
                small_units: small_units,
            }
            TokenValueSmall {
                class: "ml-auto",
                amount: format!("{:.1$}", token_amount_f64, token_decimals as usize),
                ticker: token_ticker.clone(),
                small_units: small_units,
            }
        }
    }
}


pub fn NullValue() -> Element {
    rsx! {
        span {
            class: "text-elements-midEmphasis font-medium",
            "–"
        }
    }
}


pub fn LoadingValue() -> Element {
    rsx! {
        span {
            class: "w-10 h-4 rounded my-auto loading",
            ""
        }
    }
}

fn format_with_commas(number: &str) -> String {
    if number.len() <= 3 {
        return number.to_string();
    }

    let mut result = String::new();
    let mut count = 0;
    for c in number.chars().rev() {
        if count > 0 && count % 3 == 0 {
            result.push(',');
        }
        result.push(c);
        count += 1;
    }
    result.chars().rev().collect::<String>()
}