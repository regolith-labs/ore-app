use dioxus::prelude::*;

use crate::components::Balance;

pub fn Home() -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-16 overflow-visible",
            Balance {}
            div {
                class: "flex w-full px-4 py-2 bg-blue-500 text-white rounded",
                "Web mining has been temporarily disabled. It will return soon with the rollout of the upcoming web app redesign."
            }
        }
    }
}
