use dioxus::prelude::*;

use crate::{
    components::{Col, Row},
    route::Route,
};

#[component]
pub fn TableSimple(rows: Element) -> Element {
    rsx! {
        Col {
            {rows}
        }
    }
}

#[component]
pub fn TableSimpleRowLink(
    to: Route,
    left: Element,
    right: Element,
    onclick: Option<EventHandler<MouseEvent>>,
) -> Element {
    rsx! {
        Link {
            to: to,
            class: "flex flex-row w-full px-4 h-20 transition hover:bg-controls-tertiary active:bg-controls-tertiaryHover hover:cursor-pointer",
            onclick: move |e| {
                if let Some(handler) = &onclick {
                    handler.call(e);
                }
            },
            Row {
                class: "w-full my-auto grow-0 shrink-0 justify-between", 
                span {
                    class: "w-min",
                    {left}
                }
                span {
                    class: "flex text-right my-auto justify-end",
                    {right}
                }
            }
        }
    }
}
