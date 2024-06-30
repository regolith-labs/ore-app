use dioxus::prelude::*;

#[component]
pub fn MountWalletAdapter() -> Element {
    let _ = use_future(move || async move {
        let eval = eval(
            r#"
                window.MountWalletAdapter();
                return
            "#,
        );
        let _ = eval.await;
    });
    rsx!(nav {
        id: "ore-wallet-adapter"
    })
}
