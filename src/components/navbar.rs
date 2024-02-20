use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::{
    components::{Banner, BannerType},
    gateway::AsyncResult,
    hooks::{use_is_onboarded, use_ping},
    route::Route,
};

#[component]
pub fn Navbar(cx: Scope) -> Element {
    let ping = use_ping(cx);
    render! {
        div {
            class: "relative min-h-screen flex flex-col",
            if let AsyncResult::Error(_) = ping {
                render! {
                    Banner {
                        text: "Please check your internet connection.".to_string(),
                        banner_type: BannerType::Error
                    }
                }
            }
            div {
                class: "flex flex-row justify-between px-4 sm:px-8 py-6 w-full",
                Link {
                    to: Route::Home {},
                    class: "flex h-10",
                    h1 {
                        class: "text-xl font-black my-auto w-min text-black",
                        "ORE"
                    }
                }
                div {
                    class: "flex flex-row gap-8",
                    Profile {}
                }
            }
            div {
                class: "flex flex-col h-full py-4 px-4 sm:px-8 grow",
                Outlet::<Route> {}
            }
        }
    }
}

#[component]
pub fn Profile(cx: Scope) -> Element {
    let is_onboarded = use_is_onboarded(cx);
    if is_onboarded.get() {
        render! {
            Link {
                to: Route::Settings {},
                class: "bg-gray-100 w-10 h-10 rounded-full"
            }
        }
    } else {
        None
    }
}
