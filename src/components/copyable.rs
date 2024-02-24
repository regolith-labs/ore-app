use dioxus::prelude::*;

use crate::{components::CopyIcon, hooks::use_clipboard};

#[component]
pub fn Copyable<'a>(cx: Scope, value: String, children: Element<'a>) -> Element {
    let clipboard = use_clipboard(cx);
    let solid = use_state(cx, || false);
    let _ = use_future(cx, solid, |_| {
        let solid = solid.clone();
        async move {
            if *solid.get() {
                async_std::task::sleep(std::time::Duration::from_secs(3)).await;
                solid.set(false);
            }
        }
    });
    render! {
        div {
            class: "flex flex-row",
            button {
                class: "flex px-2 py-1 rounded hover-100 active-200 transition-colors",
                onclick: move |_e| {
                    if let Some(cb) = clipboard.clone() {
                        let _ = cb.write_text(value);
                    }
                    solid.set(true);
                },
                CopyIcon {
                    class: "w-4 h-4 my-auto",
                    solid: *solid.get(),
                }
            }
            &children
        }
    }
}
