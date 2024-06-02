use dioxus::prelude::*;

use crate::components::ClaimStep;

#[component]
pub fn ClaimPreview(claim_step: Signal<ClaimStep>) -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-24 p-6",
            div {
                class: "flex flex-col gap-3",
                h1 {
                    "Claim rewards"
                }
                p {
                    class: "text-black text-lg",
                    "You are about to take custody of your mining rewards."
                }
                p {
                    class: "text-gray-300 text-sm",
                    "In some legal jurisdictions, this may be considered a taxable event."
                }
            }
            button {
                class: "w-full py-3 rounded font-semibold transition-colors text-white bg-green-500 hover:bg-green-600 active:bg-green-700",
                onclick: move |_| {
                    claim_step.set(ClaimStep::Edit);
                },
                "Continue"
            }
        }
    }
}
