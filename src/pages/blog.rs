use dioxus::prelude::*;

use crate::components::Col;

// Mining
// - How ORE broke solana.
// - How ORE mining works.

pub fn Blog() -> Element {
    rsx! {
        Col {
            class: "w-full max-w-7xl mx-auto px-5 sm:px-8",
            gap: 8,
            BlogHeading {
                class: "w-full",
                image: asset!("/public/social.webp"),
                tip: "Whitepaper",
                title: "ORE: Liquid Digital Gold",
                // subtitle: "ORE is a liquid digital gold that is minted on the Solana blockchain."
            }
        }
    }
}

#[component]
pub fn BlogHeading(
    class: Option<String>,
    tip: Option<String>,
    image: Option<String>,
    title: String,
    subtitle: Option<String>,
) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        Col {
            gap: 16,
            class: "{class}",
            if let Some(image) = image {
                img {
                    src: image,
                    class: "w-full h-full object-cover rounded-2xl max-h-96",
                }
            }
            // if let Some(tip) = tip {
            //     span {
            //         class: "z-30 border-l-2 border-elements-gold px-2 text-elements-gold w-min text-sm mb-2 font-semibold text-nowrap mr-auto",
            //         "{tip}"
            //     }
            // }
            Col {
                gap: 2,
                span {
                    class: "font-wide text-3xl sm:text-4xl font-bold text-center",
                    "{title}"
                }
                if let Some(subtitle) = subtitle {
                    span {
                        class: "text-elements-lowEmphasis font-medium text-center",
                        "{subtitle}"
                    }
                }
            }
        }
    }
}
