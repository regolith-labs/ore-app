use dioxus::prelude::*;

use crate::{components::{LoadingValue, NullValue, Row}, gateway::GatewayResult};

#[component]
pub fn TitledRow(title: String, value: Element) -> Element {
    rsx! {
        Row {
            class: "w-full justify-between px-4",
            span {
                class: "text-elements-lowEmphasis font-medium",
                "{title}"
            }
            {value}
        }
    }
}

#[component]
pub fn TitledResourceRow<T: Clone + PartialEq + 'static>(
    title: String,
    resource: Resource<GatewayResult<T>>,
    com: Component<T>
) -> Element {
    rsx! {
        TitledRow {
            title: title,
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
