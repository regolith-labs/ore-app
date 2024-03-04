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
    render! {
        div {
            // class: "flex flex-row-reverse sm:flex-row justify-end truncate max-w-full {class}",
            class: "flex flex-row justify-end truncate max-w-full {class}",
            button {
                class: "flex px-2 py-1 rounded hover-100 active-200 transition-colors",
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
