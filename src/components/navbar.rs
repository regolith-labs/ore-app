use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::{
    components::{Banner, BannerType, UserGroupIcon},
    gateway::AsyncResult,
    hooks::use_ping,
    route::Route,
};

#[component]
pub fn Navbar(cx: Scope) -> Element {
    let ping = use_ping(cx);
    render! {
        div {
            class: "relative min-h-screen flex flex-col text-black dark:bg-black dark:text-white",
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
                        class: "text-xl font-black my-auto w-min",
                        "ORE"
                    }
                }
                div {
                    class: "flex flex-row gap-12",
                    Link {
                        class: "transition transition-colors flex w-10 h-10 justify-center rounded-full hover-100 active-200",
                        to: Route::Leaderboard {},
                        UserGroupIcon {
                            class: "w-6 h-6 my-auto"
                        }
                    }
                    Profile {}
                }
            }
            div {
                class: "flex flex-col h-full py-4 px-4 sm:px-8 grow w-full max-w-[96rem] mx-auto",
                Outlet::<Route> {}
            }
        }
    }
}

#[component]
pub fn Profile(cx: Scope) -> Element {
    render! {
        Link {
            to: Route::Settings {},
            class: "bg-gray-100 dark:bg-gray-900 w-10 h-10 rounded-full"
        }
    }
}
