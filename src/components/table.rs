use dioxus::prelude::*;

use crate::{
    components::{Col, Row},
    route::Route,
};

#[component]
pub fn Table(children: Element) -> Element {
    rsx! {
        Col {
            class: "sm:mx-5 overflow-x-scroll",
            {children}
        }
    }
}

#[component]
pub fn TableHeader(left: String, right: Vec<String>) -> Element {
    rsx! {
        Row {
            class: "h-8 sm:h-10 min-w-[32rem] w-full px-5 sm:px-3 justify-between font-medium text-xs sm:text-sm text-gray-700",
            span {
                class: "my-auto w-48 grow shrink-0",
                "{left}"
            }
            Row {
                class: "text-right shrink-0",
                for title in right {
                    span {
                        class: "my-auto w-28 sm:w-40",
                        "{title}"
                    }
                }
            }
        }
    }
}

#[component]
pub fn TableRowLink(to: Route, left: Element, right: Vec<Element>) -> Element {
    rsx! {
        Link {
            to: to,
            class: "flex flex-row justify-between min-w-[32rem] w-full px-5 sm:px-3 py-4 transition sm:rounded-md hover:bg-controls-tertiary active:bg-controls-tertiaryHover hover:cursor-pointer",
            Row {
                class: "grow w-48 shrink-0",
                gap: 4,
                {left}
            }
            Row {
                class: "text-right my-auto",
                for r in right {
                    span {
                        class: "flex w-28 sm:w-40 justify-end",
                        {r}
                    }
                }
            }
        }
    }
}
