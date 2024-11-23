use dioxus::prelude::*;

use crate::components::{BackButton, Col, InfoIcon, Row};

#[derive(Clone, PartialEq, Eq)]
struct ComparisonItem {
    icon: &'static str,
    title: &'static str,
    text: &'static str,
}

// in each pair, first item is for solo mining, second item is for pool mining
const COMPARISON_ROWS: [[ComparisonItem;2]; 2] = [
    [
        ComparisonItem {
            icon: "🔒",
            title: "High Hardware Requirements",
            text: "Need significant hash power to be profitable.",
        },
        ComparisonItem {
            icon: "🔓",
            title: "Flexible Entry Point",
            text: "Start mining with any hash rate and still earn rewards.",
        },
    ],
    [
        ComparisonItem {
            icon: "🔒",
            title: "100% Rewards",
            text: "Keep entire reward when you find a hash, but must pay transaction fees to submit to blockchain.",
        },
        ComparisonItem {
            icon: "🔓",
            title: "Shared Rewards",
            text: "Receive a percentage of pool rewards based on your contribution, with pool covering transaction fees.",
        },
    ]
];

pub fn MineComparison() -> Element {
    rsx! {
        Col {
            class: "pb-20 sm:pb-16 gap-8",
            gap: 8,
            Header {}
            ComparisonTable {}
        }
    }
}

fn Header() -> Element {
    rsx! {
        BackButton {}
        Row {
            class: "justify-center px-5 sm:px-8",
            gap: 4,
            h1 {
                class: "font-wide text-3xl sm:text-3xl font-semibold align-text-bottom my-auto",
                "Solo Mining VS Pool mining"
            }
        }
    }
}

#[component]
fn ComparisonTable() -> Element {
    rsx! {
        Col {
            gap: 2,
            div {
                class: "comparison-table",
                TableHeader {
                    left: "Solo Mining",
                    right: "Pool Mining",
                }
                for comparison in COMPARISON_ROWS.iter() {
                    TableRow { items: comparison.clone() }
                }
            }
        }
    }
}

#[component]
pub fn TableHeader(left: String, right: String) -> Element {
    rsx! {
        Row {
            class: "comparison-table__header",
            div {
                h2 {
                    class: "text-2xl text-center",
                    {left}
                }
            }
            div {
                h2 {
                    class: "text-2xl text-center",
                    {right}
                }
            }
        }
    }
}

#[component]
fn TableRow(items: [ComparisonItem; 2]) -> Element {
    rsx! {
        div {
            class: "comparison-table__row",
            for item in items.iter() {
                div {
                    // InfoIcon {
                    //     class: "h-5 w-5 mx-auto my-auto"
                    // }
                    h3 {
                        class: "text-xl text-center",
                        {item.title}
                    }
                    p {
                        {item.text}
                    }
                }
            }
        }
    }
}
