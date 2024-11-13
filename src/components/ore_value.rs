use dioxus::prelude::*;

use crate::components::OreIcon;

#[component]
pub fn OreValue(ui_amount_string: String) -> Element {
    let units: Vec<_> = ui_amount_string.split('.').collect();
    let big_units = units[0];
    let small_units = units[1];
    rsx! {
        div {
            class: "flex flex-row gap-2 sm:gap-3 h-10 w-min",
            OreIcon {
                class: "h-6 w-6 sm:h-8 sm:w-8 my-auto"
            }
            div {
                class: "flex flex-row my-auto",
                span {
                    class: "mt-auto font-semibold text-2xl sm:text-3xl",
                    "{big_units}"
                }
                span {
                    class: "mt-auto font-semibold text-xl sm:text-2xl text-gray-700",
                    ".{small_units}"
                }
            }
        }
    }
}

#[component]
pub fn OreValueSmall(ui_amount_string: String) -> Element {
    rsx! {
        div {
            class: "flex flex-row gap-1.5 w-min",
            OreIcon {
                class: "h-4 w-4 my-auto"
            }
            div {
                class: "flex flex-row font-medium my-auto",
                span {
                    class: "mt-auto",
                    "{ui_amount_string}"
                }
            }
        }
    }
}
