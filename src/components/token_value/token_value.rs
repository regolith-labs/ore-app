use dioxus::prelude::*;

use crate::{components::{format_token_amount, Row}, config::LISTED_TOKENS_BY_TICKER};


#[component]
pub fn TokenValueSmall(class: Option<String>, amount: String, ticker: String, with_decimal_units: Option<bool>) -> Element {
    let class = class.unwrap_or("".to_string());
    let image = if let Some(token) = LISTED_TOKENS_BY_TICKER.get(&ticker) {
        token.image.clone()
    } else {
        "".to_string()
    };

    let formatted_amount = format_token_amount(amount, with_decimal_units, Some(false));
    let units: Vec<_> = formatted_amount.split('.').collect();

    rsx! {
        Row {
            class: "gap-1.5 {class}",
            img {
                class: "w-6 h-6 my-auto bg-gray-900 rounded-full border border-gray-800",
                src: "{image}"
            }
            span {
                class: "my-auto font-medium", 
                "{units[0]}"
                if with_decimal_units.unwrap_or(false) {
                    span {
                        class: "mt-auto font-medium text-elements-lowEmphasis",
                        ".{units[1]}"
                    }
                }
            }
        }
    }
}
