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
            class: "rounded-full transition-colors my-auto",
            nav {
                id: "ore-holders-list"
            }
        }
    }
}
