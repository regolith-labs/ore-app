use dioxus::prelude::*;

use crate::{components::Col, route::Route};

pub fn Landing() -> Element {
    rsx! {
        div {
            class: "flex h-full w-full mx-auto p-8 pb-20 sm:pb-16 ",
            Col {
                class: "mx-auto my-auto gap-24",
                gap: 24,
                span {
                    class: "mx-auto font-wide text-3xl sm:text-4xl font-medium px-4 py-3",
                    "Welcome to crypto."
                }
                div {
                    class: "flex flex-col sm:flex-row",
                    ActionTab {
                        title: "Mine",
                        to: Route::Mine {},
                    }
                    ActionTab {
                        title: "Stake",
                        to: Route::Stake {},
                    }
                    ActionTab {
                        title: "Trade",
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
            class: "flex h-32 w-full sm:w-64 rounded-full text-elements-lowEmphasis hover:text-elements-highEmphasis",
            to: to,
            span {
                class: "text-5xl mx-auto my-auto",
                "{title}"
            }
        }
    }
}
