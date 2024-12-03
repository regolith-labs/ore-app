use dioxus::prelude::*;

use crate::{components::*, route::Route};

pub fn Landing() -> Element {
    rsx! {
        div {
            class: "flex h-full w-full mx-auto p-8 pb-20 sm:pb-16 ",
            Col {
                class: "mx-auto my-auto gap-16",
                gap: 16,
                Col {
                    class: "mx-auto",
                    gap: 8,
                    Orb {
                        class: "mx-auto", 
                        is_gold: true,
                        is_large: true
                    }
                    Col {
                        class: "mx-auto",
                        gap: 2,
                        span {
                            class: "mx-auto font-wide text-4xl sm:text-5xl font-bold",
                            "Digital gold."
                        }
                        span {
                            class: "mx-auto font-wide text-4xl sm:text-5xl font-bold opacity-50",
                            "On Solana."
                        }
                    }
                }
                Link {
                    class: "flex mx-auto h-16 w-full sm:w-64 rounded-full controls-primary",
                    to: Route::Mine {},
                    span {
                        class: "font-wide font-bold text-2xl mx-auto my-auto",
                        "Start mining →"
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
            class: "flex h-24 w-full sm:w-64 rounded-full text-elements-lowEmphasis hover:text-elements-highEmphasis hover:bg-controls-secondaryHover",
            to: to,
            span {
                class: "font-wide font-bold text-2xl mx-auto my-auto",
                "{title}"
            }
        }
    }
}
