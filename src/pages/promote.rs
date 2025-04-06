use dioxus::prelude::*;

use crate::{
    components::*,
    gateway::ore::OreGateway,
    hooks::{use_gateway, use_wallet},
};

// 1. Check for existing account.
// 2. If no account, then advertise program and display onboarding.
// 3. If account, then display dashboard.

pub fn Promote() -> Element {
    let wallet = use_wallet();

    rsx! {
        Onboarding {}
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
    let request_token = use_resource(|| async move { use_gateway().get_x_request_token().await });
    rsx! {
        if let Some(Ok(request_token)) = request_token.cloned() {
            a {
                class: "controls-primary w-full flex flex-row justify-center items-center gap-1.5 rounded-full h-12",
                href: "https://api.x.com/oauth/authenticate?oauth_token={request_token}",
                target: "_blank",
                rel: "noopener noreferrer",
                "Log in with"
                XIcon {
                    class: "w-4 h-4"
                }
            }
        } else {
            "Loading..."
        }

    }
}

fn Dashboard() -> Element {
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
