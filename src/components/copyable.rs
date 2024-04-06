use dioxus::prelude::*;
#[cfg(feature = "desktop")]
use dioxus_std::clipboard::use_clipboard;

use crate::components::CopyIcon;
#[cfg(feature = "web")]
use crate::hooks::use_clipboard;

#[component]
pub fn Copyable<'a>(
    cx: Scope,
    class: Option<&'a str>,
    large_button: Option<bool>,
    value: String,
    children: Element<'a>,
) -> Element {
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
    let class = class.unwrap_or("");
    if large_button.unwrap_or(false) {
        render! {
            div {
                class: "flex flex-col gap-2 truncate max-w-full {class}",
                button {
                    class: "flex flex-row gap-2 shrink-0 p-2 mx-auto rounded transition-colors hover-100 active-200 font-semibold",
                    onclick: move |_e| {
                        #[cfg(feature = "web")]
                        if let Some(cb) = clipboard.clone() {
                            let _ = cb.write_text(value);
                        }

                        #[cfg(feature = "desktop")]
                        clipboard.set(value.clone()).ok();

                        solid.set(true);
                    },
                    CopyIcon {
                        class: "w-4 h-4 my-auto",
                        solid: *solid.get(),
                    }
                    "Copy to clipboard"
                }
                &children
            }
        }
    } else {
        render! {
            div {
                class: "flex flex-row justify-end truncate max-w-full {class}",
                button {
                    class: "flex shrink-0 p-2 rounded transition-colors hover-100 active-200",
                    onclick: move |_e| {
                        #[cfg(feature = "web")]
                        if let Some(cb) = clipboard.clone() {
                            let _ = cb.write_text(value);
                        }

                        #[cfg(feature = "desktop")]
                        clipboard.set(value.clone()).ok();

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
}
