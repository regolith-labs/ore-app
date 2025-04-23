use dioxus::prelude::*;

use crate::{
    components::*,
    components::CheckCircleIcon,
    gateway::ore::OreGateway,
    hooks::{use_gateway, use_wallet, Wallet},
};

pub fn Promote() -> Element {
    let wallet = use_wallet();
    
    // Check waitlist status if wallet is connected
    let waitlist_status = use_resource(move || async move {
        if let Wallet::Connected(pubkey) = *wallet.read() {
            use_gateway().validate_waitlist_status(pubkey).await
        } else {
            Ok(false)
        }
    });

    rsx! {
        match *wallet.read() {
            Wallet::Connected(_) => {
                match *waitlist_status.read() {
                    Some(Ok(true)) => rsx! { Waitlist {} },
                    Some(Ok(false)) => rsx! { Onboarding {} },
                    Some(Err(_)) => rsx! { div { class: "mx-auto w-full max-w-2xl px-5 sm:px-8", "Loading..." } },
                    None => rsx! { div { class: "mx-auto w-full max-w-2xl px-5 sm:px-8", "Loading..." } }
                }
            },
            _ => rsx! { Onboarding {} }
        }
    }
}

fn Onboarding() -> Element {
    rsx! {
        Col {
            class: "w-full h-full pb-20 sm:pb-16",
            gap: 8,
            Heading {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                title: "Promote",
                subtitle: "Create and share content online."
            }
            Col {
                gap: 8,
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                span {
                    class: "text-lg",
                    "Eligible creators can now earn rewards by creating and sharing content about ORE on X dot com. To get started, log in with your X account below."
                }
                Col {
                    gap: 4,
                    SignInWithX {}
                    span {
                        class: "text-xs text-elements-lowEmphasis text-center",
                        "By linking your account, you agree to share your data with Regolith Labs and accept the Terms and Conditions of the creator rewards program. Regolith Labs may modify or disable the creator rewards program at any time in its sole discretion, including for business, financial, or legal reasons."
                    }
                }
            }
        }
    }
}

fn SignInWithX() -> Element {
    let wallet = use_wallet();
    let request_token = use_resource(|| async move { use_gateway().get_x_request_token().await });
    
    // Check if we have a wallet connected
    let is_wallet_connected = matches!(*wallet.read(), Wallet::Connected(_));
    
    rsx! {
        if let Some(Ok(token)) = request_token.cloned() {
            div {
                if is_wallet_connected {
                    a {
                        class: "controls-primary w-full flex flex-row justify-center items-center gap-1.5 rounded-full h-12",
                        href: format!("https://api.x.com/oauth/authenticate?oauth_token={}", token),
                        target: "_blank",
                        rel: "noopener noreferrer",
                        span { "Log in with " },
                        XIcon { class: "w-4 h-4" }
                    }
                } else {
                    button {
                        class: "controls-primary w-full flex flex-row justify-center items-center gap-1.5 rounded-full h-12 opacity-70 cursor-not-allowed",
                        disabled: true,
                        span { "Log in with " },
                        XIcon { class: "w-4 h-4" }
                    }
                }
            }
        } else {
            div { "Loading..." }
        }
    }
}

fn Waitlist() -> Element {
    let wallet = use_wallet();
    rsx! {
        Col {
            class: "w-full h-full pb-20 sm:pb-16",
            gap: 8,
            Heading {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                title: "Waitlist",
                subtitle: "You're on the waitlist for creator rewards."
            }
            Col {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                gap: 8,
                CheckCircleIcon {
                    class: "mx-auto w-24 h-24 text-elements-green mt-8"
                }
                Col {
                    gap: 2,
                    span {
                        class: "text-elements-highEmphasis font-semibold text-2xl mx-auto",
                        "Congratulations!"
                    }                                    
                    span {
                        class: "text-elements-highEmphasis font-medium mx-auto text-center",
                        {
                            if let Wallet::Connected(pubkey) = *wallet.read() {
                                let address = pubkey.to_string();
                                let short_address = format!("{}...{}", 
                                    &address[..6], 
                                    &address[address.len()-6..]);
                                format!("You're on the waitlist for creator rewards with wallet {}", short_address)
                            } else {
                                "You're on the waitlist for creator rewards.".to_string()
                            }
                        }
                    }
                }                
            }
        }
    }
}

fn _Dashboard() -> Element {
    rsx! {
        Col {
            class: "w-full h-full pb-20 sm:pb-16",
            gap: 8,
            Heading {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                title: "Promote",
                subtitle: "Create and share content online."
            }
        }
    }
}


