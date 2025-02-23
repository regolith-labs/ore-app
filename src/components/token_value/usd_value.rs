use dioxus::prelude::*;

use crate::components::{format_token_amount, Row};

#[component]
pub fn UsdValue(
    class: Option<String>,
    ui_amount_string: String,
    with_decimal_units: Option<bool>,
    abbreviated: Option<bool>,
) -> Element {
    let class = class.unwrap_or("".to_string());
    let formatted_amount = format_token_amount(ui_amount_string, with_decimal_units, abbreviated);
    let units: Vec<_> = formatted_amount.split('.').collect();

    rsx! {
        Row {
            class: "gap-1.5 {class}",
            span {
                class: "my-auto font-medium",
                "${units[0]}"
                if with_decimal_units.unwrap_or(false) {
                    span {
                        class: "mt-auto font-medium",
                        ".{units[1]}"
                    }
                }
            }
        }
    }
}
