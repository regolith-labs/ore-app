use dioxus::prelude::*;

#[component]
pub fn WhatIsMining(cx: Scope) -> Element {
    render! {
        div {
            class: "flex flex-col gap-4 font-hero",
            h2 {
                "What is mining?"
            }
            p {
                "(Draft in progress...)"
            }
            p {
                "Most people have heard of crypto mining, but few people truly underestand it and only a handful have ever done it. "
                "So what is mining and how does it work?"
            }
            p {
                span {
                    class: "font-bold",
                    "Mining is the act of producing tokens by burning energy. "
                }
                // "When creating a mining algorithm, protocol designers have to decide what types of behaviors they are going to incentivize."
                // "This can take many forms such or running an internet hotspot, giving a passenger a ride, or even counting your steps."
            }
            p {
                // "The goal is to distribute tokens as fairly as possible. "
                // "This requires us to define 'fairness'. "
                // "There are many different ways to approach this question, but we shall limit ourselves only to those definitions which can computational be proved have happened."
                "There's a bit to unpack here so we'll break it down one concept at a time."
            }
            p {
                class: "text-xl font-bold mt-4",
                "Producing tokens"
            }
            p {
                "Let's start with the tokens. "
                "Tokens are just digital assets. "
                "Like the coins you might carry in your pocket or the trading cards you might collect, tokens are digital objects you can own, trade, and transfer over the internet. "
            }
            p {
                "Tokens come in many different shapes and sizes. "
                "Some of the most familiar tokens are associated with popular blockchains such as Bitcoin (BTC), Ethereum (ETH), or Solana (SOL). "
                "However tokens can represent anything such as an ownership stake in a company or a product like a concert ticket. "
                "Many tokens are associated with cultural symbols such as memes, celebrities, or even breeds of dog. "
            }
            p {
                "When it comes to mining, the thing we care about is not what the tokens represent, but rather how they are produced. "
                "Blockchains allow anyone with a computer to mint and distribute their own tokens. "
                "Mining is a special kind of distribution process, managed by computer code, that mints and issues new tokens according to how much energy the user has provably burned. "
            }
            p {
                class: "text-xl font-bold mt-4",
                "Burning energy"
            }
            p {
                "At first glance, burning energy might seem a little counterintuitive. "
                "Energy is scarce after all; we shouldn't waste it. "
                "On deeper inspection, burning energy turns out to be an incredibly powerful mechanism for token distribution. "
            }
            p {
                "TODO"
            }
            p {
                "For one, Burning energy "
                "This grounds the tokens in the physical economy. "
                "It creates a material opportunity cost for each one produced. "
                "To logic goes like this: for a miner, the mined token must be worth at least the energy required to produce it, otherwise they wouldn't have spent that energy to produce the token."
            }
            p {
                "So how do we burn energy? "
                "And more importantly, how do we prove we've burned energy? "
                "Well this is where computation comes to the rescue."
            }
            p {
                class: "text-xl font-bold mt-4",
                "Proof of work"
            }
            p {
                "TODO"
            }
            p {
                "Imagine you are given a puzzle. "
                "It's a very difficult puzzle for you to do in your head or on paper, but your computer can finish it in a few seconds. "
                "Importantly, it is very easy for another computer to verify if your computer's solution is correct."
            }
            p {
                "This is where the phrase 'proof-of-work' comes from. "
                "It refers simply the proof your computer generates to show it has accomplished some verifiable amount of work."
            }
            p {
                "Ore uses a special hash function called SHA-3 that GPUs are really good at."
            }
            p {
                "When you're mining, you have to prove your computer can perform a calculation of a known difficulty. To do this, it computes a bunch of hash values until it finds one that satisfies the difficulty requirement."
            }
            p {
                "When it finds a valid hash, it submits it to the blockchain – a public, permissionless computer network – for verification. "
                "If your solution is valid, a new token is produced."
            }
            p {
                "The information produced by all these hashes is meaningless and unimportant. The output of the mining process is not the information; it's the token."
            }
            p {
                class: "text-xl font-bold mt-4",
                "Putting it all together"
            }
        }
    }
}
