use dioxus::prelude::*;
use dioxus_sdk::clipboard::use_clipboard;

use crate::components::CopyIcon;

#[component]
pub fn CodeBlock(text: String) -> Element {
    let mut clipboard = use_clipboard();
    let mut solid = use_signal(|| false);
    use_future(move || async move {
        if *solid.read() {
            async_std::task::sleep(std::time::Duration::from_secs(3)).await;
            solid.set(false);
        }
    });
    let strs = text.split('\n');
    rsx! {
        div {
            class: "flex flex-row justify-between overflow-x-auto py-2 pl-4 pr-2 bg-gray-100 text-black dark:bg-gray-900 dark:text-white font-mono rounded",
            div {
                class: "flex flex-col",
                for s in strs {
                    p {
                        "{s}"
                    }
                }
            }
            button {
                class: "flex shrink-0 px-2 py-1 mb-auto rounded hover-100 active-200 transition-colors",
                onclick: move |_e| {
                    clipboard.set(text.clone()).ok();
                    solid.set(true);
                },
                CopyIcon {
                    class: "w-4 h-4 my-auto",
                    solid: *solid.read(),
                }
            }
        }
    }
}
