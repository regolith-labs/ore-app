use dioxus::prelude::*;
#[cfg(feature = "desktop")]
use dioxus_std::clipboard::use_clipboard;

use crate::components::CopyIcon;
#[cfg(feature = "web")]
use crate::hooks::use_clipboard;

#[component]
pub fn CodeBlock<'a>(cx: Scope, text: &'a str) -> Element {
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
    let strs = text.split('\n');
    render! {
        div {
            class: "flex flex-row justify-between py-2 pl-4 pr-2 bg-gray-100 text-black dark:bg-gray-900 dark:text-white font-mono rounded",
            div {
                class: "flex flex-col",
                for s in strs {
                    render! {
                        p {
                            "{s}"
                        }
                    }
                }
            }
            button {
                class: "flex shrink-0 px-2 py-1 mb-auto rounded hover-100 active-200 transition-colors",
                onclick: move |_e| {
                    #[cfg(feature = "web")]
                    if let Some(cb) = clipboard.clone() {
                        let _ = cb.write_text(text);
                    }

                    #[cfg(feature = "desktop")]
                    clipboard.set(text.to_string()).ok();

                    solid.set(true);
                },
                CopyIcon {
                    class: "w-4 h-4 my-auto",
                    solid: *solid.get(),
                }
            }
        }
    }
}
