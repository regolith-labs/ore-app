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
            class: "flex flex-col gap-4",
            h2 {
                "Ore economics"
            }
            p {
                "Coming soon..."
            }
            p {
                "(In short, 1 new ORE token is mined every 60 seconds on average no matter how many miners are active on the network)"
            }
        }
    }
}
