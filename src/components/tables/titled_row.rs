use dioxus::prelude::*;

use crate::{
    components::{Col, InfoIcon, InfoText, LoadingValue, NullValue, Row},
    gateway::GatewayResult,
};

#[component]
pub fn TitledRow(title: String, description: String, value: Element) -> Element {
    let mut info_hidden = use_signal(|| true);
    // let max_height = if *open.read() { "max-h-96" } else { "max-h-0" };
    // let opacity = if *open.read() {
    //     "opacity-100"
    // } else {
    //     "opacity-0"
    // };
    rsx! {
        button {
            class: "flex flex-col py-4 px-0 sm:px-3 transition-all duration-300 ease-in-out group hover:cursor-pointer",
            onclick: move |_| info_hidden.set(!info_hidden.cloned()),
            Row {
                class: "w-full justify-between gap-2 sm:gap-16",
                Col {
                    Row {
                        class: "text-elements-lowEmphasis h-min",
                        gap: 2,
                        div {
                            class: "font-medium text-left",
                            "{title}"
                        }
                        InfoIcon {
                            class: "h-4 w-4 shrink-0 my-auto group-hover:text-elements-highEmphasis transition-all duration-300 ease-in-out",
                        }
                    }
                    InfoText {
                        class: "hidden sm:block text-sm text-left",
                        text: description.clone(),
                        hidden: info_hidden,
                    }
                }
                span {
                    class: "mb-auto",
                    {value}
                }
            }
            InfoText {
                class: "sm:hidden text-sm text-left",
                text: description,
                hidden: info_hidden,
            }
        }
    }
}

#[component]
pub fn TitledResourceRow<T: Clone + PartialEq + 'static>(
    title: String,
    description: String,
    resource: Resource<GatewayResult<T>>,
    com: Component<T>,
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
pub fn TitledSignalRow<T: Clone + PartialEq + 'static>(
    title: String,
    description: String,
    signal: Signal<GatewayResult<T>>,
    com: Component<T>,
) -> Element {
    rsx! {
        TitledRow {
            title: title,
            description: description,
            value: rsx! {
                LoadableSignal {
                    signal: signal,
                    com: com,
                }
            }
        }
    }
}

#[component]
pub fn LoadableResource<T: Clone + PartialEq + 'static>(
    resource: Resource<GatewayResult<T>>,
    com: Component<T>,
) -> Element {
    let Some(resource) = resource.cloned() else {
        return rsx! { LoadingValue {} };
    };
    let Ok(resource) = resource else {
        return rsx! { NullValue {} };
    };
    rsx! { { com(resource) } }
}

#[component]
pub fn LoadableSignal<T: Clone + PartialEq + 'static>(
    signal: Signal<GatewayResult<T>>,
    com: Component<T>,
) -> Element {
    let Ok(signal) = signal.cloned() else {
        return rsx! { NullValue {} };
    };
    rsx! { { com(signal) } }
}
