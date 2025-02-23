use dioxus::prelude::*;

use super::{format_token_amount, TokenValueSize};
use crate::components::{OreIcon, Row};

#[component]
pub fn OreValue(
    class: Option<String>,
    ui_amount_string: String,
    with_decimal_units: Option<bool>,
    abbreviated: Option<bool>,
    gold: Option<bool>,
    size: Option<TokenValueSize>,
    color_override: Option<String>,
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
    let whole_units_color = color_override
        .clone()
        .unwrap_or(whole_units_color.to_string());
    let decimal_units_color = color_override
        .clone()
        .unwrap_or(decimal_units_color.to_string());

    let (icon_gap, icon_size, whole_units_size, decimal_units_size, font_weight) =
        match size.unwrap_or(TokenValueSize::Small) {
            TokenValueSize::XSmall => ("gap-1", "h-3 w-3", "text-xs", "text-xs", "font-medium"),
            TokenValueSize::Small => (
                "gap-1.5",
                "h-4 w-4",
                "text-base",
                "text-base",
                "font-medium",
            ),
            TokenValueSize::Large => (
                "gap-3 h-10",
                "h-6 w-6 sm:h-8 sm:w-8",
                "text-3xl",
                "text-2xl",
                "font-semibold",
            ),
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
