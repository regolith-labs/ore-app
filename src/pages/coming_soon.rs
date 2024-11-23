use dioxus::prelude::*;

use crate::components::BackButton;

pub fn ComingSoon() -> Element {
    rsx! {
        div {
            class: "flex w-full h-full",
            div {
                class: "w-full max-w-7xl mx-auto my-auto",
                div {
                    class: "flex flex-col gap-2",
                    BackButton {}
                    span {
                        class: "font-wide text-lg sm:text-xl text-center",
                        "Coming soon"
                    }
                }
            }
        }
    }
}
