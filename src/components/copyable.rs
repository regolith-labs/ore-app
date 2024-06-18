use dioxus::prelude::*;
use dioxus_std::clipboard::use_clipboard;

use crate::components::CopyIcon;

#[component]
pub fn Copyable(
    class: Option<String>,
    large_button: Option<bool>,
    value: String,
    children: Element,
) -> Element {
    let mut clipboard = use_clipboard();
    let mut solid = use_signal(|| false);
    use_future(move || async move {
        if *solid.read() {
            async_std::task::sleep(std::time::Duration::from_secs(3)).await;
            solid.set(false);
        }
    });
    let class = class.unwrap_or("".to_string());
    if large_button.unwrap_or(false) {
        rsx! {
            div {
                class: "flex flex-col gap-2 truncate max-w-full {class}",
                button {
                    class: "flex flex-row gap-2 shrink-0 p-2 mx-auto rounded transition-colors hover-100 active-200 font-semibold",
                    onclick: move |_e| {
                        clipboard.set(value.clone()).ok();
                        solid.set(true);
                    },
                    CopyIcon {
                        class: "w-4 h-4 my-auto",
                        solid: *solid.read(),
                    }
                    "Copy to clipboard"
                }
                {children}
            }
        }
    } else {
        rsx! {
            div {
                class: "flex flex-row justify-end truncate max-w-full {class}",
                button {
                    class: "flex shrink-0 p-2 rounded transition-colors hover-100 active-200",
                    onclick: move |_e| {
                        clipboard.set(value.clone()).ok();
                        solid.set(true);
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
