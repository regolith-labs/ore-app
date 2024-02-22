use dioxus::prelude::*;

#[derive(Props)]
pub struct BackButtonProps<'a> {
    pub onclick: EventHandler<'a>,
}

#[component]
pub fn BackButton<'a>(cx: Scope<'a, BackButtonProps<'a>>) -> Element {
    render! {
        button {
            class: "transition-colors text-2xl -ml-2 w-10 h-10 bg-transparent hover-100 active-200 rounded-full mr-auto",
            onclick: move |_| cx.props.onclick.call(()),
            "←"
        }
    }
}
