use dioxus::prelude::*;

use crate::components::ClaimStep;

#[derive(Props)]
pub struct ClaimPreviewProps<'a> {
    pub claim_step: &'a UseState<ClaimStep>,
}

#[component]
pub fn ClaimPreview<'a>(cx: Scope<'a, ClaimPreviewProps<'a>>) -> Element {
    let claim_step = cx.props.claim_step;
    render! {
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
                onclick: |_| {
                    claim_step.set(ClaimStep::Edit);
                },
                "Continue"
            }
        }
    }
}
