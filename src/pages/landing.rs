use dioxus::prelude::*;

use crate::components::*;

pub fn Landing() -> Element {
    rsx! {
        Hero {}
        Section1 {}
        Section2 {}
        Section3 {}
        Section4 {}
        Footer {}
    }
}

fn Hero() -> Element {
    rsx! {
        Col {
            class: "relative w-full h-full mx-auto max-w-7xl min-h-screen",
            HeroTitle {}
            HeroOrb {}
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

fn Section1() -> Element {
    rsx! {
        Col {
            class: "relative w-screen h-full min-h-screen",
            Col {
                class: "w-full h-min mx-auto max-w-7xl justify-start",
                SectionCopy {
                    title: "Proof of work.",
                    subtitle: "On Solana.",
                    detail: "Start mining in just one click."
                }
            }
            LandingMiner {}
        }
    }
}

fn Section2() -> Element {
    rsx! {
        Col {
            class: "relative w-screen h-full min-h-screen",
            Col {
                class: "w-full h-min mx-auto max-w-7xl justify-start",
                SectionCopy {
                    title: "Hard coded.",
                    subtitle: "Fixed supply.",
                    detail: "Protect your wealth from inflation."
                }
            }
            
        }
    }
}

fn Section3() -> Element {
    rsx! {
        Col {
            class: "relative w-screen h-full min-h-screen",
            Col {
                class: "w-full h-min mx-auto max-w-7xl justify-start",
                SectionCopy {
                    title: "Deep liquidity.",
                    subtitle: "Native yield.",
                    detail: "Stake and earn productive yield on your crypto."
                }
                // LandingOrbit {}
                LandingWave {}
            }
        }
    }
}

fn Section4() -> Element {
    rsx! {
        Col {
            class: "relative w-full h-full mx-auto max-w-7xl min-h-screen",
            SectionCopy {
                title: "Join the community.",
                detail: "Discover why people around the world love ORE."
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
                    class: "md:mb-auto mt-4 z-10 text-elements-midEmphasis font-wide font-medium text-lg sm:text-xl md:text-2xl max-w-lg md:max-w-xl lg:max-w-2xl mx-auto",
                    "{detail}"
                }
            }
        }
    }
}

fn Footer() -> Element {
    rsx! {
        Col {
            class: "w-screen h-full bg-base-canvas py-32 px-4 sm:px-8 text-elements-highEmphasis",
            Logo {}
            "Socials"
        }
    }
}