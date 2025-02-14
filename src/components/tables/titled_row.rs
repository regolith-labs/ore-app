use dioxus::prelude::*;

use crate::{components::{Col, InfoIcon, LoadingValue, NullValue, Row}, gateway::GatewayResult};

#[component]
pub fn TitledRow(title: String, description: String, value: Element) -> Element {
    let mut open = use_signal(|| false);
    let color = if *open.read() { "" } else { "" };
    let max_height = if *open.read() { "max-h-96" } else { "max-h-0" };
    let opacity = if *open.read() { "opacity-100" } else { "opacity-0" };
    rsx! {
        button {
            class: "flex flex-col gap-4 px-0 sm:px-3 transition-all rounded duration-300 ease-in-out group hover:cursor-pointer {color}",
            onclick: move |_| open.set(!open.cloned()),
            Row {
                class: "w-full justify-between gap-4 sm:gap-16",
                Col {
                    gap: 4,
                    Row {
                        class: "text-elements-lowEmphasis h-min",
                        gap: 2,
                        div {
                            class: "font-medium text-left",
                            "{title}"
                        }
                        InfoIcon {
                            class: "h-4 w-4 shrink-0 my-auto transition-opacity duration-300 ease-in-out group-hover:opacity-100 opacity-0",
                        }
                    }
                    div {
                        class: "hidden sm:block overflow-hidden transition-all duration-300 ease-in-out h-min {max_height}",
                        span {
                            class: "block w-full text-left transition-opacity duration-300 ease-in-out {opacity} text-elements-midEmphasis",
                            "{description}"
                        }
                    }
                }
                {value}
            }
            div {
                class: "sm:hidden overflow-hidden transition-all duration-300 ease-in-out h-min {max_height}",
                span {
                    class: "block w-full text-left transition-opacity duration-300 ease-in-out {opacity} text-elements-midEmphasis",
                    "{description}"
                }
            }
        }
    }
}

#[component]
pub fn TitledResourceRow<T: Clone + PartialEq + 'static>(
    title: String,
    description: String,
    resource: Resource<GatewayResult<T>>,
    com: Component<T>
) -> Element {
    rsx! {
        TitledRow {
            title: title,
            description: description,
            value: rsx! {
                LoadableResource {
                    resource: resource,
                    com: com,
                }
            }
        }
    }
}

#[component]
pub fn LoadableResource<T: Clone + PartialEq + 'static>(
    resource: Resource<GatewayResult<T>>, 
    com: Component<T>
) -> Element {
    let Some(resource) = resource.cloned() else {
        return rsx! { LoadingValue {} };
    };
    let Ok(resource) = resource else {
        return rsx! { NullValue {} };
    };
    rsx! { { com(resource) } }
}
