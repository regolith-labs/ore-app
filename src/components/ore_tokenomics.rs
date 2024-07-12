use dioxus::prelude::*;

// TODO Live supply
// TODO Live circulating supply

// TODO 1 ORE/min
// TODO Emphasis on simple predictability and fairness.
// TODO Linear supply inflation provides a baseline incentive for lending and spending
// TODO Protection from exponential inflation
// TODO Longterm sustainability in a way that deflationary currencies (such as BTC and ETH) do not.
// TODO This supply rate will be the same 100 years from now as it is today
// TODO Each generation will see approximately the same number of tokens mined (XYZ over the average 80 year human lifetime).
// TODO 21 million new supply every ~40 years.
// TODO Claims

pub fn OreTokenomics() -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-4 h-full font-hero max-w-3xl w-full mx-auto pb-20 leading-7",
            p {
                class: "text-4xl font-bold",
                "ORE tokenomics"
            }
            p {
                "(Coming soon...)"
            }
            p {
                "Here are the key points:"
                li {
                    class: "ml-2",
                    "ORE has a total maximum supply of 21 million tokens."
                }
                li {
                    class: "ml-2",
                    "On average, one new ORE token will be mined every minute."
                }
                li {
                    class: "ml-2",
                    "It will take ~40 years until the total ORE supply is fully mined."
                }
                li {
                    class: "ml-2",
                    "You can stake ORE to receive a multiplier on your mining rewards."
                }
            }
        }
    }
}
