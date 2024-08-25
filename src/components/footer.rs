use dioxus::prelude::*;

use crate::{
    components::{DexscreenIcon, DiscordIcon, GithubIcon, OreLogoIcon, XIcon},
    route::Route,
};

// TODO About
// TODO   What is mining?
// TODO   Tokenomics
// TODO   Glossary

// TODO Product
// TODO   CLI
// TODO   Web

// TODO Media
// TODO   In the news
// TODO   Media kit
// TODO   Contact

// TODO Copyright Regolith Labs

#[component]
pub fn Footer(transparent_bg: bool, show_site_map: bool) -> Element {
    let bg_color = if transparent_bg { "" } else { "bg-black" };
    rsx! {
        div {
            class: "flex flex-col gap-8 w-full px-4 sm:px-8 py-8 md:py-10 snap-end text-white {bg_color}",
            div {
                class: "flex flex-row text-white w-full justify-between",
                Link {
                    class: "my-auto",
                    to: Route::Landing {},
                    OreLogoIcon {
                        class: "h-6 md:h-8"
                    }
                }
                SocialLinks {}
            }
            if show_site_map {
                SiteMap {}
                Copyright {}
            }
        }
    }
}

fn SiteMap() -> Element {
    let container_class = "flex flex-col w-full gap-2";
    let title_class = "font-bold";
    let link_class = "transition-opacity active:opacity-70 hover:underline";
    rsx! {
        div {
            class: "flex flex-row gap-2 w-full",
            div {
                class: "{container_class}",
                p {
                    class: "{title_class}",
                    "About"
                }
                Link {
                    class: "{link_class}",
                    to: Route::WhatIsMining {},
                    "What is mining?"
                }
                Link {
                    class: "{link_class}",
                    to: Route::OreTokenomics {},
                    "Tokenomics"
                }
            }
            div {
                class: "{container_class}",
                p {
                    class: "{title_class}",
                    "Clients"
                }
                Link {
                    class: "{link_class}",
                    to: Route::Download {},
                    "CLI"
                }
                Link {
                    class: "{link_class}",
                    to: Route::Home {},
                    "Web"
                }
            }
        }
    }
}

fn SocialLinks() -> Element {
    rsx! {
        div {
            class: "flex flex-row sm:text-sm md:text-base lg:text-lg my-auto gap-4 md:gap-8",
            Link {
                to: "https://dexscreener.com/solana/ggadtfbqdgjozz3fp7zrtofgwnrs4e6mczmmd5ni1mxj",
                class: "flex h-10 w-10 hover:bg-gray-100 hover:bg-opacity-20 active:bg-opacity-30 transition-colors rounded-full text-white",
                new_tab: true,
                DexscreenIcon {
                    class: "w-6 h-6 m-auto"
                }
            }
            Link {
                to: "https://discord.gg/4TQfshAAsT",
                class: "flex h-10 w-10 hover:bg-gray-100 hover:bg-opacity-20 active:bg-opacity-30 transition-colors rounded-full text-white",
                new_tab: true,
                DiscordIcon {
                    class: "w-6 h-6 m-auto"
                }
            }
            Link {
                to: "https://github.com/regolith-labs/ore",
                class: "flex h-10 w-10 hover:bg-gray-100 hover:bg-opacity-20 active:bg-opacity-30 transition-colors rounded-full text-white",
                new_tab: true,
                GithubIcon {
                    class: "w-6 h-6 m-auto"
                }
            }
            Link {
                to: "https://x.com/oresupply",
                class: "flex h-10 w-10 hover:bg-gray-100 hover:bg-opacity-20 active:bg-opacity-30 transition-colors rounded-full text-white",
                new_tab: true,
                XIcon {
                    class: "w-6 h-6 m-auto"
                }
            }
        }
    }
}

fn Copyright() -> Element {
    rsx! {
        p {
            class: "text-sm opacity-80",
            "© 2024 Regolith Labs Inc."
        }
    }
}
