use dioxus::prelude::*;

use crate::{
    components::*,
    gateway::ore::OreGateway,
    hooks::{use_gateway, use_persistent, use_persistent_override, use_wallet},
};

// 1. Check for existing account.
// 2. If no account, then advertise program and display onboarding.
// 3. If account, then display dashboard.

pub fn Creator() -> Element {
    let wallet = use_wallet();

    rsx! {
        CreatorOnboarding {}
    }
}

fn CreatorOnboarding() -> Element {
    rsx! {
        Col {
            class: "w-full h-full pb-20 sm:pb-16",
            gap: 8,
            Heading {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                title: "Creator Rewards",
                subtitle: "Get paid to create content."
            }
            Col {
                gap: 8,
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                span {
                    class: "text-lg",
                    "With the Creator Rewards Program, eligible creators can now earn money by creating and posting content about ORE on X. To get started, log in with your X account below."
                }
                Col {
                    gap: 4,
                    SignInWithX {}
                    span {
                        class: "text-xs text-elements-lowEmphasis text-center",
                        "By linking your account, you agree to the Terms and Conditions and acknowledge that Regolith Labs operates the oracle which powers the Creator Rewards Program. Regolith Labs may modify or disable the oracle at any time in its sole discretion, including for business, financial, or legal reasons."
                    }
                }
            }
        }
    }
}

fn SignInWithX() -> Element {
    let request_token =
        use_resource(|| async move { use_gateway().get_twitter_request_token().await.unwrap() });
    rsx! {
        if let Some(request_token) = request_token.cloned() {
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

fn CreatorDashboard() -> Element {
    rsx! {
        Col {
            class: "w-full h-full pb-20 sm:pb-16",
            gap: 8,
            Heading {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                title: "Creator",
                subtitle: "Get paid to create content."
            }
        }
    }
}
