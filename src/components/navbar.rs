use dioxus::prelude::*;

use crate::{
    components::{
        Banner, BannerStyle, Footer, MountWalletAdapter, OreLogoIcon, OreWordmarkIcon, UserBubble,
    },
    hooks::use_appearance,
    route::Route,
};

use super::Appearance;

pub fn Navbar() -> Element {
    let appearance = use_appearance();
    let dark = match *appearance.read() {
        Appearance::Dark => "dark",
        Appearance::Light => "",
    };
    rsx! {
        div {
            class: "relative min-h-screen flex flex-col text-black dark:bg-black dark:text-white {dark}",
            Banner {
                style: BannerStyle::Info,
                link_to: "https://v1.ore.supply",
                "This is a devnet preview of ORE v2. To use the v1 app, click here →"
            }
            div {
                class: "flex w-full",
                div {
                    class: "max-w-[96rem] w-full flex flex-row justify-between mx-auto px-4 sm:px-8 py-6",
                    Link {
                        to: Route::Home {},
                        class: "flex h-10",
                        OreWordmarkIcon {
                            class: "h-3 md:h-4 my-auto"
                        }
                    }
                    div {
                        class: "flex flex-row gap-6 md:gap-8 lg:gap-10",
                        MountWalletAdapter {}
                        Profile {}
                    }
                }
            }
            div {
                class: "flex flex-col h-full py-4 px-4 sm:px-8 grow w-full max-w-[96rem] mx-auto",
                Outlet::<Route> {}
            }
        }
    }
}

pub fn Profile() -> Element {
    rsx! {
        Link {
            to: Route::Settings {},
            style: "align-self: center;",
            UserBubble {
                class: "w-10 h-10"
            }
        }
    }
}

pub fn SimpleNavbar() -> Element {
    rsx! {
        div {
            class: "flex flex-col min-h-screen h-full bg-white text-black",
            div {
                class: "flex flex-row justify-between px-4 sm:px-8 py-8 w-full z-50",
                Link {
                    to: Route::Landing {},
                    class: "flex flex-row h-10",
                    OreLogoIcon {
                        class: "h-6 md:h-8"
                    }
                }
            }
            div {
                class: "py-4 px-4 sm:px-8 grow h-full w-full max-w-[96rem] mx-auto",
                Outlet::<Route> {}
            }
            Footer {
                transparent_bg: false,
                show_site_map: false,
            }
        }
    }
}
