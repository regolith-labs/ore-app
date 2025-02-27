use crate::components::*;
use crate::hooks::{APP_FEE, SOLANA_BASE_FEE};
use dioxus::prelude::*;
use solana_sdk::native_token::lamports_to_sol;

#[component]
pub fn Fee(priority_fee: Signal<u64>) -> Element {
    let mut is_open = use_signal(|| false);
    let base_fee = lamports_to_sol(SOLANA_BASE_FEE);
    let app_fee = lamports_to_sol(APP_FEE);
    let priority_fee = lamports_to_sol(priority_fee.cloned() / 1_000_000);

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
            class: "w-full flex flex-col transition-all duration-300 ease-in-out hover:cursor-pointer".to_string(),
            onclick: move |_| is_open.set(!is_open.cloned()),
            Row {
                class: "w-full justify-between items-center",
                Row {
                    class: "text-elements-lowEmphasis gap-2 items-center",
                    span {
                        class: "font-medium",
                        "Total Fee"
                    }
                    InfoIcon {
                        class: "h-4 w-4 shrink-0",
                    }
                }
                Row {
                    class: "items-center gap-2",
                    span {
                        class: "text-elements-lowEmphasis font-medium",
                        { format!("{:.5} SOL", total_fee) }
                    }
                }
            }
            Col {
                gap: 3,
                Col {
                    class: "overflow-hidden transition-all duration-300 ease-in-out {max_height}",
                    Col {
                        class: "pt-4 gap-2 transition-opacity duration-300 {opacity}",
                        Row {
                            class: "w-full justify-between",
                            span { class: "font-medium text-xs text-elements-lowEmphasis", "App Fee" }
                            span { class: "font-medium text-xs text-elements-lowEmphasis", "{base_fee:.5} SOL" }
                        }
                        Row {
                            class: "w-full justify-between",
                            span { class: "font-medium text-xs text-elements-lowEmphasis", "Solana base Fee" }
                            span { class: "font-medium text-xs text-elements-lowEmphasis", "{priority_fee:.5} SOL" }
                        }
                        Row {
                            class: "w-full justify-between",
                            span { class: "font-medium text-xs text-elements-lowEmphasis", "Solana priority Fee" }
                            span { class: "font-medium text-xs text-elements-lowEmphasis", "{app_fee:.5} SOL" }
                        }
                    }
                }
            }
        }
    }
}
