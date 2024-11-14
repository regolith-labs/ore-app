use dioxus::prelude::*;

use crate::{
    components::{Col, OreIcon},
    route::Route,
};

pub fn Landing() -> Element {
    rsx! {
        Link {
            to: Route::Mine {},
            class: "flex h-full w-full mx-auto p-8 ",
            Col {
                class: "mx-auto my-auto",
                gap: 8,
                span {
                    class: "p-10 mx-auto rounded-full hover:loading",
                    OreIcon {
                        class: "h-16 w-16"
                    }
                }
                span {
                    class: "mx-auto font-wide font-medium px-4 py-3 hover:underline hover:font-semibold",
                    "Welcome to crypto →"
                }
            }
        }
    }
}
