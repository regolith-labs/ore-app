use b64::ToBase64;
use dioxus::document::eval;
use dioxus::prelude::*;
use ore_types::{request::LinkTwitterAccountRequest, response::AccessTokenResponse};
use steel::Pubkey;

use crate::{
    components::*,
    gateway::ore::OreGateway,
    hooks::{use_gateway, use_wallet, Wallet},
};

#[component]
pub fn Callback(oauth_token: String, oauth_verifier: String) -> Element {
    let access_token = use_resource(move || {
        let oauth_token = oauth_token.clone();
        let oauth_verifier = oauth_verifier.clone();
        async move {
            use_gateway()
                .get_access_token(oauth_token, oauth_verifier)
                .await
                .unwrap()
        }
    });

    rsx! {

        Col {
            class: "w-full h-full pb-20 sm:pb-16",
            gap: 8,
            Heading {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                title: "Link Account",
                subtitle: "Link your X account to receive creator rewards."
            }
            Col {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                gap: 8,
                if let Some(access_token) = access_token.cloned() {
                    LinkAccount { access_token }
                } else {
                    "Loading..."
                }
            }
        }
    }
}

#[component]
pub fn LinkAccount(access_token: AccessTokenResponse) -> Element {
    let wallet = use_wallet();
    rsx! {
        Col {
            gap: 8,
            p {
                "User ID: {access_token.user_id}"
            }
            p {
                "Screen Name: {access_token.screen_name}"
            }
            if let Wallet::Connected(pubkey) = *wallet.read() {
                LinkAccountButton { access_token, pubkey }
            } else {
                "Connect Wallet"
            }
        }
    }
}

#[component]
pub fn LinkAccountButton(access_token: AccessTokenResponse, pubkey: Pubkey) -> Element {
    rsx! {
        button {
            class: "controls-primary h-12 rounded-full justify-center items-center",
            onclick: move |_| {
                spawn(async move {
                    // Build eval command for wallet signing
                    let mut eval = eval(
                        r#"
                let msg = await dioxus.recv();
                let signed = await window.OreMsgSigner({b64: msg});
                dioxus.send(signed);
                "#,
                    );

                    // Sign request with wallet
                    let msg = "Testing".as_bytes().to_base64(b64::STANDARD);
                    let _send = eval
                        .send(serde_json::Value::String(msg))
                        .map_err(|err| anyhow::anyhow!(err))
                        .unwrap();

                    // wait on eval
                    let res = eval.recv().await;

                    // Process eval result
                    match res {
                        // Process valid signing result
                        Ok(serde_json::Value::String(string)) => {
                            log::info!("signed message: {}", string);
                        }

                        _ => {
                            log::error!("error signing message");
                        }
                    }
                });
            },
            "Link Account"
        }
    }
}
