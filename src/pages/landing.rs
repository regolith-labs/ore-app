use dioxus::prelude::*;

use crate::{components::*, hooks::{use_ore_holders, use_ore_market_cap}, route::Route, utils::format_abbreviated_number};

pub fn Landing() -> Element {
    rsx! {
        Hero {}
        // Marqee {}
        Mining {}
        Liquidity {}
        Stats {}
        Community {}
        Footer {}
    }
}

fn Hero() -> Element {
    rsx! {
        Col {
            class: "relative w-full h-full max-w-screen min-h-screen",
            LandingNavbar {}
            Col {
                class: "absolute w-full h-full mx-auto max-w-7xl top-0 bottom-0 left-0 right-0 z-50",
                HeroTitle {}
                HeroOrb {}
            }
        }
    }
}

fn LandingNavbar() -> Element {
    rsx! {
        Row {
            class: "w-screen shrink-0 h-16 sm:h-24 px-2 sm:px-6 z-100",
            Row {
                class: "w-full my-auto justify-between",
                Logo {}
                LaunchButton {}
            }
        }
    }
}

fn LaunchButton() -> Element {
    rsx! {
        Link {
            class: "flex px-8 h-12 my-auto rounded controls-primary rounded-full z-100",
            to: Route::Mine {},
            span {
                class: "my-auto",
                "Launch app →"
            }
        }
    }
}

fn HeroTitle() -> Element {
    rsx! {
        Col {
            class: "absolute left-0 right-0 bottom-32 sm:left-8 sm:right-8 md:bottom-0 md:top-0 font-extended font-bold text-7xl md:text-8xl lg:text-9xl text-center md:text-left text-elements-highEmphasis selection:bg-elements-highEmphasis selection:text-black",
            gap: 2,
            span {
                class: "md:mt-auto z-30",
                "Liquid"
            }
            span { 
                class: "z-20",
                "Digital" 
            }
            span {
                class: "md:mb-auto z-10",
                "Gold" 
            }
        }
    }
}

fn Mining() -> Element {
    rsx! {
        Col {
            class: "relative w-screen h-full min-h-screen px-4",
            Col {
                class: "w-full h-min mx-auto max-w-7xl justify-start",
                SectionCopy {
                    title: "Proof of work.",
                    subtitle: "On Solana.",
                    detail: "Start mining crypto in just one click."
                }
            }
            LandingMiner {}
        }
    }
}

fn Liquidity() -> Element {
    rsx! {
        Col {
            class: "relative w-screen h-full min-h-screen px-4",
            Col {
                class: "w-full h-min mx-auto max-w-7xl justify-start",
                SectionCopy {
                    title: "Deep liquidity.",
                    subtitle: "Native yield.",
                    detail: "Stake your crypto and earn yield."
                }
                LandingWave {}
            }
        }
    }
}

fn Community() -> Element {
    rsx! {
        Col {
            class: "relative w-full h-full mx-auto max-w-7xl min-h-screen pt-16 px-4",
            SectionCopy {
                title: "Join the community.",
                detail: "Discover why people around the world love ORE."
            }
            Testimonials {}
        }
    }
}
fn Testimonials() -> Element {
    rsx! {
        Col {
            class: "w-full h-min mx-auto max-w-7xl justify-start",
            Row {
                class: "w-full gap-8",
                Col {
                    class: "flex-1 mt-16",
                    Testimonial {
                        image: "https://api.dicebear.com/7.x/avataaars/svg?seed=Felix",
                        name: "John Doe",
                        quote: "This is an amazing project! The technology behind it is truly revolutionary and I'm excited to see where it goes."
                    }
                    Testimonial {
                        image: "https://api.dicebear.com/7.x/avataaars/svg?seed=Felix",
                        name: "John Doe",
                        quote: "This is an amazing project! The technology behind it is truly revolutionary and I'm excited to see where it goes."
                    }
                    Testimonial {
                        image: "https://api.dicebear.com/7.x/avataaars/svg?seed=Felix",
                        name: "John Doe",
                        quote: "This is an amazing project! The technology behind it is truly revolutionary and I'm excited to see where it goes."
                    }
                    Testimonial {
                        image: "https://api.dicebear.com/7.x/avataaars/svg?seed=Felix",
                        name: "John Doe",
                        quote: "This is an amazing project! The technology behind it is truly revolutionary and I'm excited to see where it goes."
                    }
                }
                Col {
                    class: "flex-1",
                    Testimonial {
                        image: "https://api.dicebear.com/7.x/avataaars/svg?seed=Felix",
                        name: "John Doe",
                        quote: "This is an amazing project! The technology behind it is truly revolutionary and I'm excited to see where it goes."
                    }
                    Testimonial {
                        image: "https://api.dicebear.com/7.x/avataaars/svg?seed=Felix",
                        name: "John Doe",
                        quote: "This is an amazing project! The technology behind it is truly revolutionary and I'm excited to see where it goes."
                    }
                    Testimonial {
                        image: "https://api.dicebear.com/7.x/avataaars/svg?seed=Felix",
                        name: "John Doe",
                        quote: "This is an amazing project! The technology behind it is truly revolutionary and I'm excited to see where it goes."
                    }
                    Testimonial {
                        image: "https://api.dicebear.com/7.x/avataaars/svg?seed=Felix",
                        name: "John Doe",
                        quote: "This is an amazing project! The technology behind it is truly revolutionary and I'm excited to see where it goes."
                    }
                    Testimonial {
                        image: "https://api.dicebear.com/7.x/avataaars/svg?seed=Felix",
                        name: "John Doe",
                        quote: "This is an amazing project! The technology behind it is truly revolutionary and I'm excited to see where it goes."
                    }
                }
                Col {
                    class: "flex-1 mt-16",
                    Testimonial {
                        image: "https://api.dicebear.com/7.x/avataaars/svg?seed=Felix",
                        name: "John Doe",
                        quote: "This is an amazing project! The technology behind it is truly revolutionary and I'm excited to see where it goes."
                    }
                    Testimonial {
                        image: "https://api.dicebear.com/7.x/avataaars/svg?seed=Felix",
                        name: "John Doe",
                        quote: "This is an amazing project! The technology behind it is truly revolutionary and I'm excited to see where it goes."
                    }
                    Testimonial {
                        image: "https://api.dicebear.com/7.x/avataaars/svg?seed=Felix",
                        name: "John Doe",
                        quote: "This is an amazing project! The technology behind it is truly revolutionary and I'm excited to see where it goes."
                    }
                    Testimonial {
                        image: "https://api.dicebear.com/7.x/avataaars/svg?seed=Felix",
                        name: "John Doe",
                        quote: "This is an amazing project! The technology behind it is truly revolutionary and I'm excited to see where it goes."
                    }
                }
            }
        }
    }
}

#[component]
fn Testimonial(image: String, name: String, quote: String) -> Element {
    rsx! {
        Col {
            class: "bg-elements-midEmphasis/10 rounded-lg p-6 mb-8",
            Row {
                class: "gap-3",
                img {
                    class: "w-10 h-10 rounded-full",
                    src: "{image}" // Placeholder avatar
                }
                Col {
                    class: "gap-1",
                    span {
                        class: "font-semibold",
                        "{name}"
                    }
                    p {
                        class: "text-elements-lowEmphasis",
                        "{quote}"
                    }
                }
            }
        }
    }
}

fn Stats() -> Element {
    // TODO 
    let holders = use_ore_holders();
    let market_cap = use_ore_market_cap();
    rsx! {
        Col {
            class: "w-full h-min text-elements-highEmphasis px-4",
            Col {
                class: "md:flex-row md:gap-24 relative w-full py-32 px-4 mx-auto max-w-7xl border-t border-b border-elements-midEmphasis",
                gap: 16,

                Col {
                    class: "md:ml-auto",
                    span {
                        class: "text-3xl md:text-4xl lg:text-5xl font-extended font-bold text-center text-elements-highEmphasis",
                        if let Some(Ok(holders)) = holders.cloned() {
                            "{format_abbreviated_number(holders as f64)}"
                        } else {
                            "–"
                        }
                    }
                    span {
                        class: "text-lg md:text-xl lg:text-2xl font-wide font-semibold text-center text-elements-lowEmphasis",
                        "Holders"
                    }
                }

                Col {
                    class: "",
                    span {
                        class: "text-3xl md:text-4xl lg:text-5xl font-extended font-bold text-center text-elements-highEmphasis",
                        "5,000,000"
                    }
                    span {
                        class: "text-lg md:text-xl lg:text-2xl font-wide font-semibold text-center text-elements-lowEmphasis",
                        "Max Supply"
                    }
                }

                Col {
                    class: "md:mr-auto",
                    span {
                        class: "text-3xl md:text-4xl lg:text-5xl font-extended font-bold text-center text-elements-highEmphasis",
                        if let Some(Ok(market_cap)) = market_cap.cloned() {
                            "${format_abbreviated_number(market_cap)}"
                        } else {
                            "–"
                        }
                    }
                    span {
                        class: "text-lg md:text-xl lg:text-2xl font-wide font-semibold text-center text-elements-lowEmphasis",
                        "Market Cap"
                    }
                }
            }
        }
    }
}

#[component]
fn SectionCopy(title: String, subtitle: Option<String>, detail: Option<String>) -> Element {
    rsx! {
        Col {
            class: "py-16 font-wide font-bold text-4xl md:text-5xl lg:text-6xl text-center text-elements-highEmphasis selection:bg-elements-highEmphasis selection:text-black px-4",
            gap: 2,
            span {
                class: "z-30",
                "{title}"
            }
            if let Some(subtitle) = subtitle {
                span { 
                    class: "z-20 text-elements-lowEmphasis",
                    "{subtitle}" 
                }
            }
            if let Some(detail) = detail {
                span {
                    class: "md:mb-auto mt-4 z-10 text-elements-midEmphasis font-wide font-medium text-lg sm:text-xl md:text-2xl mx-auto",
                    "{detail}"
                }
            }
        }
    }
}

fn Footer() -> Element {
    rsx! {
        Col {
            class: "w-screen h-full text-elements-highEmphasis pt-8 px-4",
            Row {
                class: "w-full h-min mx-auto max-w-7xl justify-end pb-4",
                SocialLinks {}
            }
            Col {
                class: "w-full h-min mx-auto max-w-7xl justify-between border-t border-elements-midEmphasis pt-4",
                gap: 16,
                span {
                    class: "text-elements-lowEmphasis text-sm font-medium ml-auto mr-4",
                    "© Regolith Labs 2025"
                }
                OreWordmarkIcon {
                    class: "w-full"
                }
            }
        }
    }
}