use dioxus::prelude::*;

use crate::{components::Col, route::Route};

pub fn Landing() -> Element {
    rsx! {
        div {
            class: "flex h-full w-full mx-auto p-8 pb-20 sm:pb-16 ",
            Col {
                class: "mx-auto my-auto gap-12",
                gap: 12,
                span {
                    class: "mx-auto font-wide text-3xl sm:text-4xl font-medium",
                    "Welcome to crypto."
                }
                span {
                    class: "mx-auto text-elements-lowEmphasis sm:text-lg font-medium",
                    "Pick an activity..."
                }
                div {
                    class: "flex flex-col sm:flex-row sm:gap-4",
                    ActionTab {
                        title: "MINE",
                        to: Route::Mine {},
                    }
                    ActionTab {
                        title: "STAKE",
                        to: Route::Stake {},
                    }
                    ActionTab {
                        title: "TRADE",
                        to: Route::Trade {},
                    }
                }
            }
        }
    }
}

#[component]
fn ActionTab(title: String, to: Route) -> Element {
    rsx! {
        Link {
            class: "flex h-24 w-full sm:w-64 rounded-full text-elements-midEmphasis hover:text-elements-highEmphasis hover:bg-controls-secondaryHover",
            to: to,
            span {
                class: "font-wide font-semibold text-2xl mx-auto my-auto",
                "{title}"
            }
        }
    }
}
