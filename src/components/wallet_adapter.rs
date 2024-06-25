use dioxus::prelude::*;

#[component]
pub fn WalletAdapter() -> Element {
    rsx! {
        Render {}
    }
}

#[component]
fn Render() -> Element {
    let _ = use_resource(move || async move {
        let init_wallets = eval(
            r#"
                const walletAdapter = window.OreWalletAdapter;
                console.log(walletAdapter);
                walletAdapter();
                return
            "#,
        );
        let _ = init_wallets.await;
    });
    rsx! {
        nav {
            id: "ore-wallet-adapter-id"
        }
    }
}
