use dioxus::prelude::*;

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

fn BackButton() -> Element {
    let navigator = use_navigator();
    rsx! {
        button {
            class: "w-10 h-10 -ml-2.5 rounded-full text-bold text-elements-midEmphasis hover:bg-controls-handle",
            onclick: move |_| {
                navigator.go_back();
            },
            "←"
        }
    }
} 