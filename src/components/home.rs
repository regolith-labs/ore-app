use dioxus::prelude::*;

use crate::components::{Activity, ActivityIndicator, Balance};

#[component]
pub fn Home(cx: Scope) -> Element {
    render! {
        div {
            class: "flex flex-col gap-16 overflow-visible",
            Balance {}
            // div {
            //     class: "bg-green-500 text-white rounded py-1 px-2",
            //     p {
            //         class: "font-medium",
            //         "Start mining"
            //     }
            // }
            Activity {}
        }
    }
}

// div {
//     class: "flex flex-row gap-2 bg-green-500 text-white w-full rounded p-4",
//     ActivityIndicator {}
//     p {
//         class: "font-medium",
//         "Mining"
//     }
// }
