use dioxus::prelude::*;

use crate::components::{Col, PageTitle};

// TODO Display QR code
// TODO Use camera to capture QR code

pub fn Pay() -> Element {
    rsx! {
        Col {
            class: "w-full",
            gap: 8,
            PageTitle {
                title: "Pay"
            }
        }
    }
}
