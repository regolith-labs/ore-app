use std::str::FromStr;

use b64::{FromBase64, ToBase64};
use dioxus::document::eval;
use dioxus::prelude::*;
use ore_types::response::AccessTokenResponse;
use solana_sdk::signature::Signature;
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
                .get_x_access_token(oauth_token, oauth_verifier)
                .await
        }
    });

    rsx! {

        Col {
            class: "w-full h-full pb-20 sm:pb-16",
            gap: 8,
            Heading {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                title: "Link account",
                subtitle: "Link your account to begin earning creator rewards."
            }
            Col {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                gap: 8,
                if let Some(Ok(access_token)) = access_token.cloned() {
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
                "Connect wallet"
            }
        }
    }
}

#[component]
pub fn LinkAccountButton(access_token: AccessTokenResponse, pubkey: Pubkey) -> Element {
    log::info!("LinkAccountButton: {:?}", access_token.oauth_token);
    rsx! {
        button {
            class: "controls-primary h-12 rounded-full justify-center items-center",
            onclick: move |_| {
                let access_token = access_token.clone();
                let pubkey = pubkey.clone();
                spawn(async move {
                    // Build eval command for wallet signing
                    let mut eval = eval(
                        r#"
                let msg = await dioxus.recv();
                let signed = await window.OreMsgSigner({b64: msg});
                console.log("signed", signed);
                dioxus.send(signed);
                "#,
                    );

                    // Sign request with wallet
                    let msg = format!(
                        "I authorize Regolith Labs to use content published on my X account for the creator rewards program.\n\nAccount: {}\nAddress: {}\nAuth: {}",
                        access_token.screen_name.clone(),
                        pubkey,
                        access_token.oauth_token.clone()
                    )
                    .as_bytes()
                    .to_base64(b64::STANDARD);

                    // Send message to eval
                    let _send = eval
                        .send(serde_json::Value::String(msg.clone()))
                        .map_err(|err| anyhow::anyhow!(err))
                        .unwrap();

                    // wait on eval
                    let res = eval.recv().await;

                    // Process eval result
                    match res {
                        // Process valid signing result
                        Ok(serde_json::Value::String(sig)) => {
                            if let Ok(sig) = sig.from_base64() {
                                log::info!("sig: {:?}", sig);
                                if let Ok(sig) = Signature::try_from(sig) {
                                    log::info!("signed message: {}", sig);
                                    let response = use_gateway()
                                        .link_x_account(
                                            access_token.user_id.clone(),
                                            msg,
                                            sig,
                                            pubkey,
                                            access_token.oauth_token.clone(),
                                        )
                                        .await;
                                    log::info!("response: {:?}", response);
                                } else {
                                    log::error!("error parsing signature");
                                }
                            } else {
                                log::error!("error decoding signature");
                            }
                        }

                        Err(err) => {
                            log::error!("error signing message: {}", err);
                        }

                        x => {
                            log::error!("error signing message: {:?}", x);
                        }
                    }
                });
            },
            "Link account"
        }
    }
}
