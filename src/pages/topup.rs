use dioxus::prelude::*;

use crate::components::*;

// TODO Display QR code
// TODO Use camera to capture QR code

#[component]
pub fn Topup(address: String) -> Element {
    rsx! {
        Col {
            class: "w-full h-full pb-20 sm:pb-16",
            gap: 8,
            Heading {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                title: "Top up",
                subtitle: "Fund your desktop wallet with SOL."
            }
        }
    }
}
