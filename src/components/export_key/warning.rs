use dioxus::prelude::*;

use crate::components::{FlagIcon, LockOpenIcon};

use super::ExportKeyStep;

#[component]
pub fn ExportKeyWarning(step: Signal<ExportKeyStep>) -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-16 grow w-full h-full",
            div {
                class: "flex flex-col gap-3 justify-start",
                h2 {
                    class: "text-nowrap",
                    "Export key"
                }
            }
            div {
                class: "flex flex-col gap-8",
                div {
                    class: "flex flex-row gap-4",
                    FlagIcon {
                        class: "w-6 h-6"
                    }
                    p {
                        class: "text-lg",
                        "Never share your private key with anyone."
                    }
                }
                div {
                    class: "flex flex-row gap-4",
                    LockOpenIcon {
                        class: "w-6 h-6"
                    }
                    p {
                        class: "text-lg",
                        "Anyone with your private key will have complete control of your account."
                    }
                }
            }
            button {
                onclick: move |_| {
                    let mut step = step.clone();
                    step.set(ExportKeyStep::Secret)
                },
                class: "bg-green-500 hover:bg-green-600 active:bg-green-700 transition-colors text-white rounded text-center font-semibold py-3 mt-auto",
                "I understand. Continue"
            }
        }
    }
}
