use dioxus::prelude::*;

pub fn Holders() -> Element {
    let _ = use_future(move || async move {
        let eval = eval(
            r#"
            window.getTokenHolders();
            return
        "#,
        );
        let _ = eval.await;
    });

    rsx! {
        div {
            class: "rounded-full transition-colors my-auto h-8 sm:h-10",
            nav {
                id: "ore-holders-list"
            }
        }
    }
}
