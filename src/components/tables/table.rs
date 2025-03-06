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
    help_left: Option<String>,
    help_right_1: Option<String>,
    help_right_2: Option<String>,
    help_right_3: Option<String>,
    help_right_4: Option<String>,
    display_help: Signal<bool>,
) -> Element {
    let class = class.unwrap_or("".to_string());
    let help_class = if display_help.cloned() {
        "max-h-96 opacity-100"
    } else {
        "max-h-0 opacity-0"
    };
    rsx! {
        Row {
            class: "min-h-8 sm:min-h-10 h-min w-full min-w-max px-5 sm:px-3 justify-between font-medium text-elements-lowEmphasis text-nowrap {class}",
            span {
                class: "flex w-screen sm:w-full sm:min-w-96 -ml-5 sm:ml-0 px-5 sm:px-0",
                Row {
                    class: "my-auto w-full sm:min-w-96 grow-0 shrink-0 sm:grow justify-between",
                    Col {
                        class: "w-full mb-auto",
                        span {
                            {left}
                        }
                        if let Some(help_left) = help_left {
                            span {
                                class: "overflow-hidden my-2 transition-all duration-300 ease-in-out text-xs text-wrap text-elements-midEmphasis {help_class}",
                                "{help_left}"
                            }
                        }
                    }
                    Col {
                        class: "text-right w-56 mb-auto",
                        span {
                            {right_1}
                        }
                        if let Some(help_right_1) = help_right_1 {
                            span {
                                class: "overflow-hidden my-2 transition-all duration-300 ease-in-out text-xs text-wrap pl-2 text-elements-midEmphasis {help_class}",
                                "{help_right_1}"
                            }
                        }
                    }
                }
            }
            Row {
                if let Some(right_2) = right_2 {
                    Col {
                        class: "text-right w-56 mb-auto",
                        span {
                            {right_2}
                        }
                        if let Some(help_right_2) = help_right_2 {
                            span {
                                class: "overflow-hidden my-2 transition-all duration-300 ease-in-out text-xs text-wrap pl-2 text-elements-midEmphasis {help_class}",
                                "{help_right_2}"
                            }
                        }
                    }
                }
                if let Some(right_3) = right_3 {
                    Col {
                        class: "text-right w-56 mb-auto",
                        span {
                            {right_3}
                        }
                        if let Some(help_right_3) = help_right_3 {
                            span {
                                class: "overflow-hidden my-2 transition-all duration-300 ease-in-out text-xs text-wrap pl-2 text-elements-midEmphasis {help_class}",
                                "{help_right_3}"
                            }
                        }
                    }
                }
                if let Some(right_4) = right_4 {
                    Col {
                        class: "text-right w-56 mb-auto",
                        span {
                            {right_4}
                        }
                        if let Some(help_right_4) = help_right_4 {
                            span {
                                class: "overflow-hidden my-2 transition-all duration-300 ease-in-out text-xs text-wrap pl-2 text-elements-midEmphasis {help_class}",
                                "{help_right_4}"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn TableRowExternalLink(
    to: String,
    left: Element,
    right_1: Element,
    right_2: Option<Element>,
    right_3: Option<Element>,
    right_4: Option<Element>,
) -> Element {
    rsx! {
        Link {
            to: to,
            new_tab: true,
            class: "flex flex-row w-full min-w-max px-5 sm:px-3 h-20 sm:rounded-md transition duration-300 ease-in-out hover:bg-controls-tertiary active:bg-controls-tertiaryHover hover:cursor-pointer",
            span {
                class: "w-screen sm:w-full sm:min-w-96 my-auto -ml-5 sm:ml-0 px-5 sm:px-0",
                Row {
                    class: "w-full sm:min-w-96 my-auto grow-0 shrink-0 sm:grow justify-between",
                    span {
                        class: "flex w-min sm:w-56 text-nowrap",
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
            class: "flex flex-row w-full min-w-max px-5 sm:px-3 h-20 sm:rounded-md transition duration-300 ease-in-out hover:bg-controls-tertiary active:bg-controls-tertiaryHover hover:cursor-pointer",
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
