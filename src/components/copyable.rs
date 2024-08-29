use dioxus::prelude::*;

use crate::components::CopyIcon;

#[component]
pub fn Copyable(
    class: Option<String>,
    implicit: Option<bool>,
    value: String,
    children: Element,
) -> Element {
    let mut solid = use_signal(|| false);
    let window = web_sys::window().expect("window");
    let clipboard = window.navigator().clipboard();

    let _ = use_resource(move || async move {
        if *solid.read() {
            async_std::task::sleep(std::time::Duration::from_secs(2)).await;
            solid.set(false);
        }
    });

    let class = class.unwrap_or("".to_string());

    rsx! {
        if implicit.unwrap_or(false) {
            button {
                class: "flex max-w-full shrink-0 p-2 rounded transition-colors hover-100 active-200 {class}",
                onclick: move |_e| {
                    if let Some(clipboard) = clipboard.clone() {
                        let _ = clipboard.write_text(value.as_str());
                        solid.set(true);
                    }
                },
                {children}
            }
        } else {
            div {
                class: "flex flex-row gap-1 justify-end max-w-full {class}",
                button {
                    class: "flex shrink-0 p-2 rounded transition-colors hover-100 active-200",
                    onclick: move |_e| {
                        if let Some(clipboard) = clipboard.clone() {
                            let _ = clipboard.write_text(value.as_str());
                            solid.set(true);
                        }
                    },
                    CopyIcon {
                        class: "w-4 h-4 my-auto",
                        solid: *solid.read(),
                    }
                }
                {children}
            }
        }
    }
}
