use dioxus::prelude::*;

use crate::components::Balance;

pub fn Home() -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-16 overflow-visible",
            Balance {}
            div {
                class: "flex w-full p-4 bg-blue-500 text-white",
                "Mining on this page has been disabled. It will return with the rollout of the web app coming soon."
            }
        }
    }
}
