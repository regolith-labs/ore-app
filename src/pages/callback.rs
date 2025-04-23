use b64::{FromBase64, ToBase64};
use dioxus::document::eval;
use dioxus::prelude::*;
use ore_types::response::AccessTokenResponse;
use solana_sdk::signature::Signature;
use steel::Pubkey;

use crate::{
    components::*,
    components::CheckCircleIcon,
    gateway::{GatewayError, ore::OreGateway},
    hooks::{use_gateway, use_wallet, Wallet},
};

#[component]
pub fn Callback(oauth_token: String, oauth_verifier: String) -> Element {
    // Track whether account linking was successful
    let linking_successful = use_signal(|| false);
    let waitlist_number = use_signal(|| 0);
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
                
                if *linking_successful.read() {
                    SuccessView { waitlist_number: waitlist_number.clone() }
                } else {
                    match access_token.cloned() {
                        Some(Ok(token)) => rsx! { LinkAccount { access_token: token.clone(), linking_successful: linking_successful.clone(), waitlist_number: waitlist_number.clone() } },
                        Some(Err(err)) => {
                            match err {
                                GatewayError::XAccountExists {screen_name, solana_address } => {
                                    rsx! { 
                                        div { 
                                            class: "p-4 border border-yellow-500 rounded", 
                                            h3 { class: "font-bold", "X Account Already Linked" }
                                            p { "The X account @{screen_name} is already registered with {solana_address} in the waitlist registration." }
                                            p { "Please try with a different X account." }
                                        }
                                    }
                                },
                                _ => {
                                    // For all other errors
                                    let err_string = format!("{:?}", err);
                                    log::error!("X account linking error: {}", err_string);
                                    
                                    rsx! { 
                                        div {
                                            class: "p-4 border border-red-500 rounded",
                                            h3 { class: "font-bold", "Error Connecting Account" }
                                            p { "We couldn't connect your X account. Please try again later." }
                                            p { class: "mt-2 text-sm text-red-700", "Details: {err_string}" }
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
}

#[component]
pub fn LinkAccount(access_token: AccessTokenResponse, linking_successful: Signal<bool>, waitlist_number: Signal<i64>) -> Element {
    let wallet = use_wallet();
    
    // Extract the profile image URL before using in RSX
    let profile_image_url = match &access_token.profile_image_url {
        Some(url) => url.clone(),
        None => String::new(),
    };
    
    rsx! {
        Col {
            class: "p-5 bg-bg-secondary rounded-xl",
            gap: 4,
            
            // Add profile image
            div {
                class: "flex justify-center mb-3",
                img {
                    src: "{profile_image_url}",
                    class: "w-16 h-16 rounded-full border-2 border-elements-mediumEmphasis",
                    alt: "Profile image"
                }
            }
            
            span {
                class: "text-elements-highEmphasis font-medium",
                "X Account Details"
            }
            div {
                class: "flex flex-col gap-2",
                div {
                    class: "flex justify-between",
                    span { class: "text-elements-mediumEmphasis", "User ID:" }
                    span { class: "text-elements-highEmphasis font-medium", "{access_token.user_id}" }
                }
                div {
                    class: "flex justify-between",
                    span { class: "text-elements-mediumEmphasis", "Screen Name:" }
                    span { class: "text-elements-highEmphasis font-medium", "@{access_token.screen_name}" }
                }
                if let Wallet::Connected(pubkey) = *wallet.read() {
                    div {
                        class: "flex justify-between",
                        span { class: "text-elements-mediumEmphasis", "Wallet Address:" }
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
pub fn LinkAccountButton(access_token: AccessTokenResponse, pubkey: Pubkey, linking_successful: Signal<bool>, waitlist_number: Signal<i64>) -> Element {
    log::info!("LinkAccountButton: {:?}", access_token.oauth_token);
    rsx! {
        button {
            class: "controls-primary h-12 rounded-full justify-center items-center",
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
            "Link account"
        }
    }
}

// Simple success view component for when account linking succeeds
#[component]
fn SuccessView(waitlist_number: Signal<i64>) -> Element {
    let waitlist_val = waitlist_number.read();
    rsx! {
        Col {
            class: "mx-auto w-full",
            gap: 8,
            CheckCircleIcon {
                class: "mx-auto w-24 h-24 text-elements-green mt-8"
            }
            Col {
                gap: 2,
                span {
                    class: "text-elements-highEmphasis font-semibold text-2xl mx-auto",
                    "You're on the waitlist!"
                }                
                span {
                    class: "text-elements-highEmphasis font-medium mx-auto",
                    "Creator rewards are coming soon. Your waitlist number is {waitlist_val}."
                }
            }
            a {
                class: "flex controls-primary w-full h-12 rounded-full hover:cursor-pointer mt-8",
                href: "/mine",
                span {
                    class: "mx-auto my-auto",
                    "Return to home"
                }
            }
        }
    }
}
