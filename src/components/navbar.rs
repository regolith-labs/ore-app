use dioxus::prelude::*;

use crate::{
    components::{
        Appearance, Banner, BannerStyle, CogIcon, Footer, MountWalletAdapter, OreLogoIcon,
        OreWordmarkIcon,
    },
    hooks::use_appearance,
    route::Route,
};

pub fn Navbar() -> Element {
    let appearance = use_appearance();
    let dark = match *appearance.read() {
        Appearance::Dark => "dark",
        Appearance::Light => "",
    };
    rsx! {
        div {
            class: "relative min-h-screen flex flex-col text-black dark:bg-black dark:text-white {dark}",
            // Banner {
            //     style: BannerStyle::Info,
            //     "This is a devnet preview of ORE v2."
            // }
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
                        class: "flex flex-row gap-4",
                        SettingsButton {}
                        MountWalletAdapter {}
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

pub fn SettingsButton() -> Element {
    let appearance = use_appearance();
    let button_color = match *appearance.read() {
        Appearance::Light => "text-gray-300 hover:text-black",
        Appearance::Dark => "text-gray-300 hover:text-white",
    };
    rsx! {
        Link {
            to: Route::Settings {},
            class: "flex my-auto h-8 w-8 sm:h-10 sm:w-10 transition-colors rounded-full transition-colors {button_color} hover-100 active-200",
            CogIcon {
                class: "h-5 w-5 sm:h-6 sm:w-6 m-auto"
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
