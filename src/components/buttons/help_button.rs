use dioxus::prelude::*;

#[component]
pub fn HelpButton(class: Option<String>, display_help: Signal<bool>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        button {
            class: "text-elements-midEmphasis font-medium border border-elements-lowEmphasis/40 rounded-full h-10 w-10 flex items-center justify-center text-center text-xl hover:cursor-pointer hover:bg-elements-lowEmphasis/10 hover:text-elements-highEmphasis transition-all duration-300 ease-in-out {class}",
            onclick: move |_| {
                let current = display_help.cloned();
                display_help.set(!current);
            },
            "?"
        }
    }
}
