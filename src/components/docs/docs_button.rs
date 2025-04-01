use dioxus::prelude::*;

use crate::components::*;
use crate::hooks::use_help_drawer_state;

pub fn DocsButton() -> Element {
    let mut drawer_state = use_help_drawer_state();

    rsx! {
        button {
            onclick: move |_| {
                let mut current = drawer_state.read().clone();
                current.is_open = true;
                drawer_state.set(current);
            },
            Row {
                class: "elevated-control elevated-border rounded-full text-sm font-semibold h-12 px-5 hover:cursor-pointer",
                gap: 2,
                BookIcon {
                    class: "w-4 text-elements-lowEmphasis"
                }
                span {
                    class: "mx-auto my-auto",
                    "Docs"
                }
            }
        }
    }
}
