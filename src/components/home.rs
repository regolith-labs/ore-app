use dioxus::prelude::*;

use crate::components::{Activity, Balance, StopButton};

pub fn Home() -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-16 overflow-visible",
            // MinerPreview {}
            Balance {}
            Activity {}
        }
    }
}

// pub fn MinerPreview() -> Element {
//     rsx! {
//         div {
//             class: "flex flex-row w-full bg-green-500 text-white justify-between rounded p-2",
//             p {
//                 class: "my-auto font-bold text-lg sm:text-xl md:text-2xl",
//                 "Mining"
//             }
//             StopButton {}
//         }
//     }
// }
