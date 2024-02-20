use dioxus::prelude::*;

use crate::components::IsModalOpen;

#[derive(Props)]
pub struct ClaimDoneProps<'a> {
    pub amount_input: &'a UseState<String>,
}

#[component]
pub fn ClaimDone<'a>(cx: Scope<'a, ClaimDoneProps<'a>>) -> Element {
    let amount_input = cx.props.amount_input;
    let is_modal_open = use_shared_state::<IsModalOpen>(cx).unwrap();
    render! {
        div {
            class: "flex flex-col gap-24 p-6",
            div {
                class: "flex flex-col gap-3",
                h1 {
                    "Success!"
                }
                p {
                    class: "text-black text-lg",
                    "You have claimed your mining rewards."
                }
                p {
                    class: "text-gray-300 text-sm",
                    "You can now spend and transfer your Ore from the dashboard."
                }
            }
            div {
                class: "flex flex-col gap-3",
                div {
                    class: "h-full"
                }
                button {
                    class: "w-full py-3 rounded font-semibold transition-colors text-white bg-green-500 hover:bg-green-600 active:bg-green-700",
                    onclick: move |_| {
                        *is_modal_open.write() = IsModalOpen(false);
                        amount_input.set("".into());
                    },
                    "Done"
                }
            }
        }
    }
}
