use dioxus::prelude::*;

use crate::{gateway::ore::OreGateway, hooks::use_gateway};

pub fn Social() -> Element {
    let token =
        use_resource(|| async move { use_gateway().get_twitter_request_token().await.unwrap() });

    rsx! {
        if let Some(token) = token.cloned() {
            a {
                href: "https://api.x.com/oauth/authenticate?oauth_token={token}",
                "Login with X"
            }
        } else {
            "Loading..."
        }
    }
}
