use dioxus::prelude::*;

#[derive(Debug, Clone)]
pub enum Toast {
    Success(String),
    Error(String),
}

pub fn use_toast() -> Signal<Option<Toast>> {
    use_context::<Signal<Option<Toast>>>()
}

pub fn use_toast_provider() {
    use_context_provider::<Signal<Option<Toast>>>(|| Signal::new(None));
}

pub fn toast() -> Element {
    let toast = use_toast();
    rsx! {
        div {
            class: "absolute left-0 right-0",
            if let Some(toast) = toast.cloned() {
                div {
                    class: "absolute right-8 bottom-8",
                }
            }
        }
    }
}
