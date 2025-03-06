use crate::components::*;
use crate::hooks::{APP_FEE, SOLANA_BASE_FEE};
use dioxus::prelude::*;
use solana_sdk::native_token::lamports_to_sol;

fn format_fee(amount: f64) -> String {
    // Remove trailing zeros after decimal point
    let s = format!("{:.9}", amount);
    s.trim_end_matches('0').trim_end_matches('.').to_string()
}

#[component]
pub fn Fee(priority_fee: Signal<u64>) -> Element {
    let mut is_open = use_signal(|| false);
    let base_fee = lamports_to_sol(SOLANA_BASE_FEE);
    let app_fee = lamports_to_sol(APP_FEE);
    let priority_fee = lamports_to_sol(priority_fee.cloned());

    let total_fee = base_fee + priority_fee + app_fee;

    let max_height = if *is_open.read() {
        "max-h-32"
    } else {
        "max-h-0"
    };
    let opacity = if *is_open.read() {
        "opacity-100"
    } else {
        "opacity-0"
    };

    rsx! {
        button {
            class: "w-full flex flex-col transition-all duration-300 ease-in-out hover:cursor-pointer group".to_string(),
            onclick: move |_| is_open.set(!is_open.cloned()),
            Row {
                class: "w-full justify-between items-center",
                Row {
                    class: "text-elements-lowEmphasis gap-2 items-center",
                    span {
                        class: "font-medium text-left",
                        "Transaction fee"
                    }
                    InfoIcon {
                        class: "h-4 w-4 shrink-0 group-hover:text-elements-highEmphasis transition-all duration-300 ease-in-out",
                    }
                }
                span {
                    class: "text-elements-midEmphasis font-medium text-right",
                    {
                        format!("{} SOL", format_fee(total_fee))
                    }
                }
            }
            Col {
                class: "overflow-hidden transition-all duration-300 ease-in-out w-full px-2 {max_height}",
                Col {
                    class: "pt-2 gap-2 transition-opacity duration-300 ease-in-out w-full {opacity}",
                    Row {
                        class: "w-full justify-between",
                        span { class: "font-medium text-sm text-elements-lowEmphasis text-left", "App fee" }
                        span { class: "font-medium text-sm text-elements-lowEmphasis text-right", "{format_fee(app_fee)}" }
                    }
                    Row {
                        class: "w-full justify-between",
                        span { class: "font-medium text-sm text-elements-lowEmphasis text-left", "Solana base fee" }
                        span { class: "font-medium text-sm text-elements-lowEmphasis text-right", "{format_fee(base_fee)}" }
                    }
                    Row {
                        class: "w-full justify-between",
                        span { class: "font-medium text-sm text-elements-lowEmphasis text-left", "Solana priority fee" }
                        span { class: "font-medium text-sm text-elements-lowEmphasis text-right", "{format_fee(priority_fee)}" }
                    }
                }
            }
        }
    }
}
