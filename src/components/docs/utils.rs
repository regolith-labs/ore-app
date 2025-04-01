use crate::components::*;
use dioxus::prelude::*;

#[component]
pub fn ContentSection(children: Element) -> Element {
    rsx! {
        Col {
            class: "w-full px-8 pb-8 overflow-y-auto scrollbar-hide text-lg text-elements-highEmphasis",
            Col {
                {children}
            }
        }
    }
}

#[component]
pub fn FaqItem(question: String, children: Element) -> Element {
    let mut is_open = use_signal(|| false);
    let rotation = if is_open.cloned() {
        "rotate-45"
    } else {
        "rotate-0"
    };
    let answer_class = if is_open.cloned() {
        "max-h-96 opacity-100"
    } else {
        "max-h-0 opacity-0"
    };

    rsx! {
        button {
            class: "flex flex-col w-full py-4 px-2 sm:px-4 cursor-pointer transition-all duration-300 ease-in-out rounded-md hover:bg-elements-midEmphasis/10",
            onclick: move |_| is_open.set(!is_open.cloned()),
            Row {
                class: "justify-between text-left text-lg w-full text-elements-highEmphasis font-semibold",
                gap: 8,
                "{question}"
                PlusIcon {
                    class: "w-4 h-4 my-auto shrink-0 transition-transform duration-300 ease-in-out text-elements-lowEmphasis {rotation}"
                }
            }
            div {
                class: "overflow-hidden transition-all duration-300 ease-in-out {answer_class}",
                p {
                    class: "text-elements-midEmphasis mt-4 text-left",
                    {children}
                }
            }
        }
    }
}

#[component]
pub fn BulletPointList(children: Element) -> Element {
    rsx! {
        Col {
            class: "w-full",
            gap: 2,
            {children}
        }
    }
}

#[component]
pub fn BulletPoint(number: Option<String>, children: Element) -> Element {
    rsx! {
        Row {
            class: "items-start pl-2",
            span {
                class: "text-elements-highEmphasis mr-2 select-none font-semibold",
                if let Some(number) = number {
                    "{number}."
                } else {
                    "•"
                }
            }
            span {
                class: "text-lg text-elements-midEmphasis flex-1",
                {children}
            }
        }
    }
}
