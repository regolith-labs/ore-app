use dioxus::prelude::*;

pub fn OreTokenomics() -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-4 h-full font-hero max-w-3xl w-full mx-auto pb-20 leading-7",
            p {
                class: "text-4xl font-bold",
                "ORE Tokenomics"
            }
            p {
                li {
                    class: "ml-2",
                    "ORE has a total maximum supply cap of 5 million tokens."
                }
                li {
                    class: "ml-2",
                    "Currently, ORE has an emissions rate of 1 ORE/min."
                }
                li {
                    class: "ml-2",
                    "Emissions will reduce by 10% every ~12 months."
                }
            }
        }
    }
}
