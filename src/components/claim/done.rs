use dioxus::prelude::*;

use crate::route::Route;

pub fn ClaimDone() -> Element {
    rsx! {
        div {
            class: "flex flex-col grow justify-between",
            div {
                class: "flex flex-col gap-2",
                h2 {
                    "Success!"
                }
                p {
                    class: "text-lg",
                    "You have claimed your mining rewards."
                }
                p {
                    class: "text-sm text-gray-300",
                    "You can now spend and transfer your Ore from the dashboard."
                }
            }
            div {
                class: "flex flex-col gap-3",
                div {
                    class: "h-full"
                }
                Link {
                    class: "w-full py-3 rounded font-semibold transition-colors text-center text-white bg-green-500 hover:bg-green-600 active:bg-green-700",
                    to: Route::Home{},
                    "Done"
                }
            }
        }
    }
}
