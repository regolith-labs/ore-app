use dioxus::prelude::*;

use crate::components::{OreIcon, Row};

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
pub fn OreValueSmall(class: Option<String>, ui_amount_string: String) -> Element {
    let class: String = class.unwrap_or("".to_string());
    let units: Vec<_> = ui_amount_string.split('.').collect();
    let big_units = units[0];
    let small_units = units[1];
    let big_units_display = format_with_commas(big_units);
    let small_units_display = small_units.trim_end_matches('0');
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
                    if small_units_display.is_empty() {
                        "{big_units_display}"
                    } else {
                        "{big_units_display}.{small_units_display}"
                    }
                }
            }
        }
    }
}

#[component]
pub fn TokenValueSmall(class: Option<String>, amount: String, image: String) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        Row {
            class: "gap-1.5 {class}",
            img {
                class: "w-6 h-6 my-auto bg-gray-900 rounded-full border border-gray-800",
                src: "{image}"
            }
            span {
                class: "my-auto font-medium", 
                "{amount}"
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