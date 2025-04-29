use dioxus::prelude::*;

use crate::{
    components::*,
    gateway::ore::{OreGateway, WaitlistStatus},
    hooks::{use_gateway, use_wallet, Wallet},
    route::Route,
};

pub fn Post() -> Element {
    let wallet = use_wallet();

    // Check waitlist status if wallet is connected
    let waitlist_status = use_resource(move || async move {
        if let Wallet::Connected(pubkey) = *wallet.read() {
            use_gateway().validate_waitlist_status(pubkey).await
        } else {
            Ok(WaitlistStatus {
                is_registered: false,
                screen_name: None,
                waitlist_number: None,
                profile_image_url: None,
            })
        }
    });

    rsx! {
        match *wallet.read() {
            Wallet::Connected(_) => {
                match waitlist_status.read().as_ref() {
                    Some(Ok(status)) if status.is_registered => rsx! { Waitlist { status: status.clone() } },
                    Some(Ok(_)) => rsx! { Onboarding {} },
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
                title: "Post",
                subtitle: "Get paid to create and share content."
            }
            Col {
                gap: 8,
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                span {
                    class: "text-lg",
                    "Eligible creators will soon be able to earn ORE by creating and sharing engaging content on X dot com. To join the waitlist, log in with your X account below."
                }
                Col {
                    gap: 4,
                    SignInWithX {}
                    span {
                        class: "text-xs text-elements-lowEmphasis text-center",
                        "By logging in with X, you agree to share your data with Regolith Labs and accept the "
                        Link {
                            class: "underline",
                            to: Route::PostTerms {},
                            "Terms and Conditions"
                        }
                        " of the ORE Creator Program. Regolith Labs may modify or disable the ORE Creator Program at any time in its sole discretion, including for business, financial, or legal reasons."
                    }
                }
            }
        }
    }
}

fn SignInWithX() -> Element {
    let wallet = use_wallet();
    let request_token = use_resource(|| async move { use_gateway().get_x_request_token().await });

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

#[component]
fn Waitlist(status: WaitlistStatus) -> Element {
    rsx! {
        Col {
            class: "w-full h-full pb-20 sm:pb-16",
            gap: 8,
            Heading {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                title: "Post",
                subtitle: "Get paid to create and share content."
            }
            Col {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                gap: 4,
                {status.profile_image_url.as_ref().map(|url| rsx! {
                    div {
                        class: "flex justify-center mb-3",
                        img {
                            src: "{url}",
                            class: "w-16 h-16 rounded-full",
                            alt: "Profile image"
                        }
                    }
                })}
                {
                    if let (Some(_name), Some(number)) = (&status.screen_name, status.waitlist_number) {
                        rsx! {
                            span {
                                class: "text-elements-highEmphasis font-semibold text-2xl mx-auto",
                                "You're #{number} on the waitlist!"
                            }
                        }
                    } else {
                        rsx! {
                            span {
                                class: "text-elements-highEmphasis font-semibold text-2xl mx-auto",
                                "You're on the waitlist!"
                            }
                        }
                    }
                }
                span {
                    class: "text-elements-midEmphasis font-medium mx-auto text-center",
                    "The ORE Creator Program will be launching soon. Follow @OREsupply on X and check back soon for updates."
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
