use crate::components::*;
use dioxus::prelude::*;

#[component]
pub fn Fee(// base fee, priority fee, ore fee
) -> Element {
    let mut is_open = use_signal(|| false);
    let base_fee = 0.00001; // Example values - replace with actual fee calculations
    let priority_fee = 0.00002;
    let ore_fee = 0.00002;
    let total_fee = base_fee + priority_fee + ore_fee;

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
                            span { class: "font-medium text-xs text-elements-lowEmphasis", "{ore_fee:.5} SOL" }
                        }
                    }
                }
            }
        }
    }
}
