use dioxus::prelude::*;

use crate::components::*;

// TODO Price chart component
// TODO Activity component

pub fn Trade() -> Element {
    rsx! {
        Col {
            class: "w-full h-full pb-20 sm:pb-16",
            gap: 4,
            Heading {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                title: "Trade",
                subtitle: "Swap tokens at the best available price."
            }
            SwapForm {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
            }
        }
    }
}
