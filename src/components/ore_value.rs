use std::str::FromStr;

use dioxus::prelude::*;
use num_format::{Locale, ToFormattedString};
use ore_api::consts::TOKEN_DECIMALS;

use crate::{components::{Col, OreIcon, Row}, config::LISTED_TOKENS_BY_TICKER, hooks::LiquidityPair};

#[derive(Clone, Copy, PartialEq)]
pub enum TokenValueSize {
    Small,
    Large,
}

#[component]
pub fn OreValue(
    class: Option<String>, 
    ui_amount_string: String,
    with_decimal_units: Option<bool>,
    abbreviated: Option<bool>,
    gold: Option<bool>,
    size: Option<TokenValueSize>
) -> Element {
    let class = class.unwrap_or("".to_string());
    let formatted_amount = format_token_amount(ui_amount_string, with_decimal_units, abbreviated);
    let units: Vec<_> = formatted_amount.split('.').collect();

    let (whole_units_color, decimal_units_color) = if gold.unwrap_or(false) {
        if abbreviated.unwrap_or(false) {
            ("text-elements-gold", "text-elements-gold")
        } else {
            ("text-elements-gold", "text-elements-gold opacity-50")
        }
    } else {
        if abbreviated.unwrap_or(false) {
            ("text-elements-highEmphasis", "text-elements-highEmphasis")
        } else {
            ("text-elements-highEmphasis", "text-elements-lowEmphasis")
        }
    };

    let (icon_gap, icon_size, whole_units_size, decimal_units_size, font_weight) = match size.unwrap_or(TokenValueSize::Small) {
        TokenValueSize::Small => ("gap-1.5", "h-4 w-4", "text-base", "text-base", "font-medium"),
        TokenValueSize::Large => ("gap-3 h-10", "h-6 w-6 sm:h-8 sm:w-8", "text-2xl sm:text-3xl", "text-xl sm:text-2xl", "font-semibold"),
    };

    rsx! {
        Row {
            class: "w-min {class} {icon_gap}",
            OreIcon {
                class: "my-auto {icon_size} {whole_units_color}"
            }
            Row {
                class: "my-auto",
                span {
                    class: "mt-auto {whole_units_size} {whole_units_color} {font_weight}",
                    "{units[0]}"
                }
                if units.len() > 1 {
                    span {
                        class: "mt-auto {decimal_units_size} {decimal_units_color} {font_weight}",
                        ".{units[1]}"
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
pub fn UsdValue(class: Option<String>, amount: String, small_units: Option<bool>) -> Element {
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
                class: "my-auto font-semibold text-2xl sm:text-3xl", 
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
            OreValue {
                class: "ml-auto",
                ui_amount_string: ore_amount_f64.to_string(),
                with_decimal_units: true,
                size: TokenValueSize::Small,
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
    stake_balance: u64, 
    liquidity_pair: LiquidityPair, 
    small_units: Option<bool>
) -> Element {
    let class = class.unwrap_or("".to_string());
    let (ore_amount_f64, token_amount_f64, token_ticker, token_decimals) = liquidity_pair.get_stake_amounts(stake_balance);
    rsx! {
        Col {
            class: "gap-2 {class}",
            OreValue {
                class: "ml-auto",
                ui_amount_string: format!("{:.1$}", ore_amount_f64, TOKEN_DECIMALS as usize),
                with_decimal_units: true,
                size: TokenValueSize::Small,
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

fn format_token_amount(
    ui_amount_string: String,
    with_decimal_units: Option<bool>,
    abbreviated: Option<bool>,
) -> String {
    // Split the amount into big and small units
    let units: Vec<_> = ui_amount_string.split('.').collect();
    let whole_units = units[0];
    let decimal_units = if units.len() > 1 { units[1] } else { "00" };

    // Format big units
    let whole_units_u64 = u64::from_str(whole_units).unwrap();
    let whole_units = whole_units_u64.to_formatted_string(&Locale::en);

    // Format small units
    let decimal_units = if with_decimal_units.unwrap_or(false) {
        if abbreviated.unwrap_or(false) {
            let mut decimal_units_significant = String::new();
            let mut non_zero_digits = 0;
            for c in decimal_units.chars() {
                decimal_units_significant.push(c);
                if c != '0' || non_zero_digits > 0 {
                    non_zero_digits += 1;
                    if non_zero_digits >= 3 || decimal_units_significant.len() >= 3 {
                        break;
                    }
                }
            }
            decimal_units_significant
        } else {
            decimal_units.trim_end_matches('0').to_string()
        }
    } else {
        "".to_string()
    };
    
    // Return formatted string
    format!("{}.{}", whole_units, decimal_units)
        .trim_end_matches(".")
        .to_string()
}