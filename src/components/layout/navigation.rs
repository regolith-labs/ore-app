use dioxus::prelude::*;

use crate::{
    components::{
        BoltIcon, CircleStackIcon, Col, DexscreenIcon, DiscordIcon, GithubIcon, GlobeIcon,
        MegaphoneIcon, OreWordmarkIcon, Row, WalletAdapter, XIcon,
    },
    hooks::use_wallet_drawer_state,
    route::Route,
};

#[component]
pub(crate) fn AppNavBar(tabs: bool) -> Element {
    rsx! {
        Row {
            class: "w-screen h-20 sm:h-24 px-2 sm:px-6 sm:sticky sm:top-0 bg-black z-50 shrink-0",
            Row {
                class: "relative justify-center sm:justify-between w-full my-auto",
                span {
                    class: "hidden sm:flex my-auto",
                    Logo {}
                }
                if tabs {
                    span {
                        class: "absolute left-1/2 -translate-x-1/2 my-auto",
                        TabBar {}
                    }
                }
                WalletAdapter {}
            }
        }
    }
}

pub(crate) fn VisitorNavBar() -> Element {
    rsx! {
        Row {
            class: "w-screen shrink-0 h-20 sm:h-24 px-2 sm:px-6",
            Row {
                class: "w-full my-auto justify-between",
                Logo {}
                // SocialLinks {}
                // LaunchButton {}
            }
        }
    }
}

pub fn Logo() -> Element {
    rsx! {
        Link {
            class: "p-1 my-auto w-min h-min rounded hover:bg-controls-secondaryHover",
            to: if cfg!(feature = "web") { Route::Landing {} } else { Route::Mine {} },
            OreWordmarkIcon {
                class: "h-5"
            }
        }
    }
}

fn TabBar() -> Element {
    let current_route: Route = use_route();
    let hidden = if is_navbar_hidden(&current_route) {
        "hidden"
    } else {
        "hidden lg:flex"
    };
    rsx! {
        Row {
            // class: "{hidden} h-full rounded-full elevated elevated-border overflow-hidden",
            class: "{hidden}",
            Tab {
                title: "Mine",
                route: Route::Mine {}
            }
            Tab {
                title: "Stake",
                route: Route::Stake {}
            }
            Tab {
                title: "Trade",
                route: Route::Trade {}
            }
            Tab {
                title: "Post",
                route: Route::Post {}
            }
        }
    }
}

#[component]
fn Tab(title: String, route: Route) -> Element {
    let current_route = use_route();
    let selected = is_tab_selected(&route, &current_route);
    let color = if selected {
        "text-elements-highEmphasis border-b-2 border-white"
    } else {
        "text-elements-lowEmphasis hover:text-elements-midEmphasis"
    };
    rsx! {
        Link {
            class: "flex w-20 h-10 transition-colors {color} hover:elevated",
            to: route,
            span {
                class: "text-sm font-medium font-semibold my-auto mx-auto",
                "{title}"
            }
        }
    }
}

pub(crate) fn MobileTabBar() -> Element {
    let current_route: Route = use_route();
    let drawer_state = use_wallet_drawer_state();

    let hidden = if is_navbar_hidden(&current_route) || *drawer_state.read() {
        "hidden"
    } else {
        ""
    };

    rsx! {
        Row {
            class: "{hidden} lg:hidden fixed bottom-0 w-full elevated z-50",
            MobileTab {
                title: "Mine",
                route: Route::Mine {}
            }
            MobileTab {
                title: "Stake",
                route: Route::Stake {}
            }
            MobileTab {
                title: "Trade",
                route: Route::Trade {}
            }
            MobileTab {
                title: "Post",
                route: Route::Post {}
            }
        }
    }
}

#[component]
fn MobileTab(title: String, route: Route) -> Element {
    let current_route: Route = use_route();
    let selected = is_tab_selected(&route, &current_route);
    let color = if !selected {
        "text-elements-lowEmphasis hover:text-elements-midEmphasis"
    } else {
        ""
    };
    rsx! {
        Link {
            class: "flex h-16 w-full",
            to: route.clone(),
            Col {
                class: "mx-auto my-auto {color}",
                gap: 1,
                match route {
                    Route::Mine {  } => rsx!{
                        BoltIcon {
                            class: "h-5 w-5 mx-auto"
                        }
                    },
                    Route::Stake {  } => rsx!{
                        CircleStackIcon {
                            class: "h-5 w-5 mx-auto"
                        }
                    },
                    Route::Trade {} | Route::TradeWithPair { token_pair: _ } => rsx!{
                        GlobeIcon {
                            class: "h-5 w-5 mx-auto"
                        }
                    },
                    Route::Post {} | Route::PostTerms {} => rsx!{
                        MegaphoneIcon {
                            class: "h-5 w-5 mx-auto"
                        }
                    },
                    _ => rsx! {}
                }
                span {
                    class: "mx-auto font-medium text-xs",
                    "{title}"
                }
            }
        }
    }
}

fn is_navbar_hidden(current_route: &Route) -> bool {
    match current_route {
        Route::Callback {
            oauth_token: _,
            oauth_verifier: _,
        } => true,
        _ => false,
    }
}

fn is_tab_selected(route: &Route, current_route: &Route) -> bool {
    match route {
        Route::Mine {} => match current_route {
            Route::Mine {} => true,
            _ => false,
        },
        Route::Stake {} => match current_route {
            Route::Stake {}
            | Route::Idle {}
            | Route::Pair { lp_mint: _ }
            | Route::Unlisted { mint: _ } => true,
            _ => false,
        },
        Route::Trade {} => match current_route {
            Route::Trade {} | Route::TradeWithPair { token_pair: _ } => true,
            _ => false,
        },
        Route::Post {} => match current_route {
            Route::Post {} | Route::PostTerms {} => true,
            _ => false,
        },
        _ => false,
    }
}

pub fn SocialLinks() -> Element {
    let button_color = "text-white";
    rsx! {
        div {
            class: "flex flex-row sm:text-sm md:text-base lg:text-lg my-auto gap-4 md:gap-8",
            Link {
                to: "https://dexscreener.com/solana/ggadtfbqdgjozz3fp7zrtofgwnrs4e6mczmmd5ni1mxj",
                class: "flex h-10 sm:h-12 w-10 sm:w-12 rounded-full {button_color} hover:bg-controls-secondaryHover my-auto",
                new_tab: true,
                DexscreenIcon {
                    class: "w-6 sm:w-8 h-6 sm:h-8 m-auto"
                }
            }
            Link {
                to: "https://discord.gg/4TQfshAAsT",
                class: "flex h-10 sm:h-12 w-10 sm:w-12 transition-colors rounded-full transition-colors {button_color} hover:bg-controls-secondaryHover my-auto",
                new_tab: true,
                DiscordIcon {
                    class: "w-6 sm:w-8 h-6 sm:h-8 m-auto"
                }
            }
            Link {
                to: "https://github.com/regolith-labs/ore",
                class: "flex h-10 sm:h-12 w-10 sm:w-12 transition-colors rounded-full transition-colors {button_color} hover:bg-controls-secondaryHover my-auto",
                new_tab: true,
                GithubIcon {
                    class: "w-6 sm:w-8 h-6 sm:h-8 m-auto"
                }
            }
            Link {
                to: "https://x.com/oresupply",
                class: "flex h-10 sm:h-12 w-10 sm:w-12 transition-colors rounded-full transition-colors {button_color} hover:bg-controls-secondaryHover my-auto",
                new_tab: true,
                XIcon {
                    class: "w-6 sm:w-8 h-6 sm:h-8 m-auto"
                }
            }
        }
    }
}
