use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::{
    components::{Banner, BannerType, OreWordmarkIcon, UserGroupIcon},
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
                    OreWordmarkIcon {
                        class: "h-3 md:h-4 my-auto"
                    }
                }
                div {
                    class: "flex flex-row gap-4 md:gap-6 lg:gap-8",
                    Link {
                        class: "transition-colors transition-opacity flex w-10 h-10 justify-center rounded-full opacity-50 hover:opacity-100",
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
            class: "bg-gray-300 dark:bg-gray-700 w-10 h-10 rounded-full"
        }
    }
}
