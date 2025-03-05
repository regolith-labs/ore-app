use std::str::FromStr;

use num_format::{Locale, ToFormattedString};

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq)]
pub enum TokenValueSize {
    XSmall,
    Small,
    Large,
}

pub fn format_token_amount(
    ui_amount_string: String,
    with_decimal_units: Option<bool>,
    abbreviated: Option<bool>,
) -> String {
    // Split the amount into big and small units
    let units: Vec<_> = ui_amount_string.split('.').collect();
    let whole_units = units[0];
    let decimal_units = if units.len() > 1 { units[1] } else { "000" };

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
            decimal_units.to_string()
            // decimal_units.trim_end_matches('0').to_string()
        }
    } else {
        "".to_string()
    };

    // Return formatted string
    format!("{}.{}", whole_units, decimal_units)
        .trim_end_matches(".")
        .to_string()
}
