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

#[component]
pub fn OreEconomics(cx: Scope) -> Element {
    render! {
        div {
            class: "flex flex-col gap-4 h-full font-hero max-w-3xl w-full mx-auto pb-20 leading-7",
            p {
                class: "text-4xl font-bold",
                "Ore economics"
            }
            p {
                "(Coming soon...)"
            }
            p {
                // "(In short, 1 new ORE token is mined every 60 seconds on average no matter how many miners are active on the network)"
                "In summary:"
                li {
                    class: "ml-2",
                    "Ore supply growth is linear with a target average of 1 ORE/min."
                }
                li {
                    class: "ml-2",
                    "Ore supply growth is strictly bounded to 0 ≤ R ≤ 2 ORE/min."
                }
                li {
                    class: "ml-2",
                    "Ore has no maximum supply limit."
                }
                li {
                    class: "ml-2",
                    "It will take ~40 years for the total Ore supply to reach 21 million tokens."
                }
                li {
                    class: "ml-2",
                    "Approximately ~42 million tokens will be mined in an average human lifetime."
                }
                li {
                    class: "ml-2",
                    "This will be as true for the next generation as it is today."
                }
            }
        }
    }
}
