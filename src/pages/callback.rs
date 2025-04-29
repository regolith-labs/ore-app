use b64::{FromBase64, ToBase64};
use dioxus::document::eval;
use dioxus::prelude::*;
use ore_types::response::AccessTokenResponse;
use solana_sdk::signature::Signature;
use steel::Pubkey;

use crate::{
    components::{CheckCircleIcon, *},
    gateway::{ore::OreGateway, GatewayError},
    hooks::{use_gateway, use_wallet, Wallet},
    route::Route,
};

#[component]
pub fn Callback(oauth_token: String, oauth_verifier: String) -> Element {
    // Track whether account linking was successful
    let linking_successful = use_signal(|| false);
    let waitlist_number = use_signal(|| 0);

    let access_token: Resource<Result<AccessTokenResponse, GatewayError>> =
        use_resource(move || {
            let oauth_token_value = oauth_token.clone();
            let oauth_verifier_value = oauth_verifier.clone();

            async move {
                use_gateway()
                    .get_x_access_token(oauth_token_value, oauth_verifier_value)
                    .await
            }
        });

    let navigator = use_navigator();
    use_effect(move || {
        if *linking_successful.read() {
            navigator.replace(Route::Post {});
        }
    });

    rsx! {
        Col {
            class: "w-full h-full pb-20 sm:pb-16",
            gap: 8,
            Heading {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                title: "Join waitlist",
                subtitle: "Claim your account to join the waitlist."
            }

            Col {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                gap: 8,
                match &*access_token.read() {
                    Some(Ok(token)) => rsx! { LinkAccount { access_token: token.clone(), linking_successful: linking_successful.clone(), waitlist_number: waitlist_number.clone() } },
                    Some(Err(err)) => {
                        match err {
                            GatewayError::XAccountExists {screen_name, solana_address } => {
                                let abbreviated_address = format!("{}...{}",
                                    &solana_address.to_string()[0..4],
                                    &solana_address.to_string()[solana_address.to_string().len()-4..]);
                                let navigator = use_navigator();
                                rsx! {
                                    div {
                                        class: "p-4 border border-yellow-500 rounded",
                                        h3 {
                                            class: "font-bold",
                                            "Account already claimed"
                                        }
                                        p {
                                            class: "text-elements-midEmphasis",
                                            "The X account @{screen_name} is already registered with another wallet address: {abbreviated_address}. Please try again with a different X account."
                                        }
                                    }
                                    button {
                                        class: "controls-primary h-12 rounded-full justify-center items-center",
                                        onclick: move |_| {
                                            navigator.replace(crate::route::Route::Post {});
                                        },
                                        "Try again"
                                    }
                                }
                            },
                            _ => {
                                // For all other errors
                                let navigator = use_navigator();
                                rsx! {
                                    div {
                                        class: "p-4 border border-red-500 rounded",
                                        h3 { class: "font-bold", "Something went wrong" }
                                        p { "We couldn't fetch your account details. Please try logging in with X again." }
                                    }
                                    button {
                                        class: "controls-primary h-12 rounded-full justify-center items-center",
                                        onclick: move |_| {
                                            navigator.replace(crate::route::Route::Post {});
                                        },
                                        "Try again"
                                    }
                                }
                            }
                        }
                    },
                    None => rsx! { div { "Loading..." } },
                }
            }
        }
    }
}

#[component]
pub fn LinkAccount(
    access_token: AccessTokenResponse,
    linking_successful: Signal<bool>,
    waitlist_number: Signal<i64>,
) -> Element {
    let wallet = use_wallet();

    // Extract the profile image URL
    let profile_image_url = match &access_token.profile_image_url {
        Some(url) => url.clone(),
        None => String::new(),
    };

    rsx! {
        Col {
            class: "bg-bg-secondary rounded-xl",
            gap: 4,

            div {
                class: "flex justify-center mb-3",
                img {
                    src: "{profile_image_url}",
                    class: "w-16 h-16 rounded-full p-2 border-2 border-elements-mediumEmphasis",
                    alt: "Profile image"
                }
            }

            // span {
            //     class: "text-elements-highEmphasis font-medium text-2xl",
            //     "X Account"
            // }

            div {
                class: "flex flex-col gap-2",
                div {
                    class: "flex justify-between",
                    span { class: "text-elements-lowEmphasis text-sm font-medium", "Account" }
                    span { class: "text-elements-highEmphasis font-medium", "@{access_token.screen_name}" }
                }
                div {
                    class: "flex justify-between",
                    span { class: "text-elements-lowEmphasis text-sm font-medium", "ID" }
                    span { class: "text-elements-highEmphasis font-medium", "{access_token.user_id}" }
                }
                if let Wallet::Connected(pubkey) = *wallet.read() {
                    div {
                        class: "flex justify-between",
                        span { class: "text-elements-lowEmphasis text-sm font-medium", "Wallet" }
                        span {
                            class: "text-elements-highEmphasis font-medium truncate",
                            "{pubkey}"
                        }
                    }
                }
            }

            if let Wallet::Connected(pubkey) = *wallet.read() {
                LinkAccountButton { access_token, pubkey, linking_successful, waitlist_number }
            } else {
                button {
                    class: "controls-primary h-12 rounded-full justify-center items-center opacity-70 cursor-not-allowed",
                    disabled: true,
                    "Connect Wallet to Link"
                }
            }
        }
    }
}

#[component]
pub fn LinkAccountButton(
    access_token: AccessTokenResponse,
    pubkey: Pubkey,
    linking_successful: Signal<bool>,
    waitlist_number: Signal<i64>,
) -> Element {
    rsx! {
        button {
            class: "controls-primary mt-4 h-12 rounded-full justify-center items-center",
            onclick: move |_| {
                let access_token = access_token.clone();
                let pubkey = pubkey.clone();
                let mut linking_successful = linking_successful.clone();

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
                        "I authorize Regolith Labs to use content published from my X account for the ORE Creator Program.\n\nAccount: {}\nAddress: {}\nAuth: {}",
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
                                if let Ok(sig) = Signature::try_from(sig) {
                                    // Call API to link account
                                    let response = use_gateway()
                                        .link_x_account(
                                            access_token.user_id.clone(),
                                            msg,
                                            sig,
                                            pubkey,
                                            access_token.oauth_token.clone(),
                                        )
                                        .await;

                                    match response {
                                        Ok(user_waitlist_number) => {
                                            // Set linking successful to true on success
                                            linking_successful.set(true);
                                            waitlist_number.set(user_waitlist_number);
                                        },
                                        Err(err) => {
                                            log::error!("Error linking X account: {:?}", err);
                                        }
                                    }
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
            "Claim account"
        }
    }
}
