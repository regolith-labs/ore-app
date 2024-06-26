use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;

use crate::hooks::build_transfer_tx;

#[component]
pub fn WalletAdapter() -> Element {
    rsx! {
        Render {}
        Dispatch {}
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
        nav { id: "ore-wallet-adapter-id" }
    }
}

#[component]
fn Dispatch() -> Element {
    rsx! {
        button {
            onclick: move |_| {
                let future = use_resource(move || async move {
                    let mut eval_1 = eval(
                        r#"
                        // get public key
                        const getPublicKey = window.OreGetPublicKey;
                        console.log(getPublicKey);
                        const publicKey = await getPublicKey(); 
                        console.log("public key from js");
                        console.log(publicKey); 
                        dioxus.send(publicKey.toBuffer());

                        //  // encode & send tx for sig
                        const transactionSigner = window.OreTxSigner;
                        console.log(transactionSigner);
                        // // rust sending json encoded tx 
                        // let msg = await dioxus.recv();
                        // console.log("msg from rs");
                        // console.log(msg);
                        // await transactionSigner({json: msg);
                        return
                    "#,
                    );
                    match eval_1.recv().await {
                        Ok(json_val) => {
                            let pubkey: Pubkey = serde_json::from_value(json_val).unwrap(); // todo
                            log::info!("decoded pubkey: {}", pubkey);
                            let tx =  build_transfer_tx(&pubkey, &pubkey, 1_000_000_000, "leo messi".to_string()).await.unwrap();
                            let tx_json_value = serde_json::to_value(&tx).unwrap();
                            eval_1.send(tx_json_value).unwrap();
                        }
                        Err(_err) => {
                            log::error!("err");
                        }
                    }
                    let _ = eval_1.await;
                });
                future.value().as_ref();
            },
            "click rsx"
        }
    }
}
