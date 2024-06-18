use dioxus::prelude::*;

use crate::components::{BusIcon, TreasuryIcon, UserIcon};

#[component]
pub fn UserBubble(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        div {
            class: "flex rounded-full text-gray-300 bg-gray-200 dark:bg-gray-700 dark:text-gray-900 {class}",
            UserIcon {
                class: "h-1/2 m-auto"
            }
        }
    }
}

#[component]
pub fn TreasuryBubble(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        div {
            class: "flex rounded-full text-gray-300 bg-gray-200 dark:bg-gray-700 dark:text-gray-900 {class}",
            TreasuryIcon {
                class: "h-1/2 m-auto"
            }
        }
    }
}

#[component]
pub fn BusBubble(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        div {
            class: "flex rounded-full text-gray-300 bg-gray-200 dark:bg-gray-700 dark:text-gray-900 {class}",
            BusIcon {
                class: "h-1/2 m-auto"
            }
        }
    }
}
