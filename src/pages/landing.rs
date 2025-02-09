use dioxus::prelude::*;

use crate::components::*;

pub fn Landing() -> Element {
    rsx! {
        Col {
            class: "flex h-full w-full",
            Hero {}
        }
    }
}

fn Hero() -> Element {
    rsx! {
        Col {
            class: "relative w-full h-full mx-auto max-w-6xl",
            LiquidDigitalGold {}
            OrbHero {}
        }
    }
}


fn LiquidDigitalGold() -> Element {
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