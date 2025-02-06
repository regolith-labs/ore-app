use dioxus::prelude::*;

use crate::{
    components::{Col, Row},
    route::Route,
};

#[component]
pub fn Table(header: Element, rows: Element, class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        Col {
            class: "overflow-x-auto max-w-full {class}",
            {header}
            {rows}
        }
    }
}

#[component]
pub fn TableHeader(
    class: Option<String>,
    left: String,
    right_1: String,
    right_2: Option<String>,
    right_3: Option<String>,
    right_4: Option<String>,
) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        Row {
            class: "h-8 sm:h-10 w-full min-w-max px-5 sm:px-3 justify-between font-medium text-elements-lowEmphasis text-nowrap {class}",
            span {
                class: "flex w-screen sm:w-full sm:min-w-96 -ml-5 sm:ml-0 px-5 sm:px-0",
                Row {
                    class: "my-auto w-full sm:min-w-96 grow-0 shrink-0 sm:grow justify-between",
                    span {
                        class: "w-min sm:w-56",
                        {left}
                    }
                    span {
                        class: "text-right w-56 my-auto",
                        {right_1}
                    }
                }
            }
            Row {
                if let Some(right_2) = right_2 {
                    span {
                        class: "text-right w-56 my-auto",
                        {right_2}
                    }
                }
                if let Some(right_3) = right_3 {
                    span {
                        class: "text-right w-56 my-auto",
                        {right_3}
                    }
                }
                if let Some(right_4) = right_4 {
                    span {
                        class: "text-right w-56 my-auto",
                        {right_4}
                    }
                }
            }
        }
    }
}

#[component]
pub fn TableRowLink(
    to: Route,
    left: Element,
    right_1: Element,
    right_2: Option<Element>,
    right_3: Option<Element>,
    right_4: Option<Element>,
) -> Element {
    rsx! {
        Link {
            to: to,
            class: "flex flex-row w-full min-w-max px-5 sm:px-3 h-20 sm:rounded-md transition hover:bg-controls-tertiary active:bg-controls-tertiaryHover hover:cursor-pointer",
            span {
                class: "w-screen sm:w-full sm:min-w-96 my-auto -ml-5 sm:ml-0 px-5 sm:px-0",
                Row {
                    class: "w-full sm:min-w-96 my-auto grow-0 shrink-0 sm:grow justify-between",
                    span {
                        class: "w-min sm:w-56 text-nowrap",
                        {left}
                    }
                    span {
                        class: "flex text-right w-56 my-auto justify-end",
                        {right_1}
                    }
                }
            }
            Row {
                if let Some(right_2) = right_2 {
                    span {
                        class: "flex text-right w-56 my-auto justify-end",
                        {right_2}
                    }
                }
                if let Some(right_3) = right_3 {
                    span {
                        class: "flex text-right w-56 my-auto justify-end",
                        {right_3}
                    }
                }
                if let Some(right_4) = right_4 {
                    span {
                        class: "flex text-right w-56 my-auto justify-end",
                        {right_4}
                    }
                }
            }
        }
    }
}

pub fn TableCellLoading() -> Element {
    rsx! {
        span {
            class: "w-16 h-8 rounded my-auto loading",
            ""
        }
    }
}