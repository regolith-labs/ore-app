use dioxus::prelude::*;

#[component]
pub fn WhatIsMining(cx: Scope) -> Element {
    render! {
        div {
            class: "flex flex-col gap-4 font-hero max-w-3xl w-full",
            h2 {
                "What is mining?"
            }
            p {
                "(Draft in progress...)"
            }
            p {
                "Many people have heard of crypto mining, but few truly understand what it is and only a handful have ever actually done it. "
                "So what is mining and how does it work?"
            }
            p {
                span {
                    class: "font-bold",
                    "Mining is the process of producing tokens by consuming energy. "
                }
            }
            p {
                "This is a really big idea and there's a lot to unpack here, so let's drill into one concept at a time."
            }
            p {
                class: "text-xl font-bold mt-4",
                "Producing tokens"
            }
            p {
                "We'll start with the tokens. "
                "Tokens are just digital assets. "
                "Like coins you might carry in your pocket or trading cards you might have collected, tokens are digital objects you can own, trade, and transfer over the internet. "
            }
            p {
                "Tokens come in many different shapes and sizes. "
                "Some of the tokens people are most familiar with are those associated with popular blockchains such as Bitcoin (BTC), Ethereum (ETH), and Solana (SOL). "
                "However tokens can represent anything such as an ownership stake in a company or a product like a concert ticket. "
                "Many tokens are associated with cultural symbols such as memes, celebrities, and dogs. "
            }
            p {
                "When it comes to mining, the thing we care about is not so much what the tokens represent per se, but rather how they are produced. "
                "Blockchains allow anyone with a computer to mint their own tokens and distribute them around the world in any manner they like. "
                "Mining is simply a special process for creating tokens that issues new tokens to users based on how much energy they have provably consumed."
            }
            p {
                class: "text-xl font-bold mt-4",
                "Consuming energy"
            }
            p {
                "At first glance, consuming energy to create digital tokens might seem a little alarming. "
                "Energy after all can be scarce and expensive to acquire; we shouldn't waste it. "
                "However on deeper inspection, it will be clearer that energy consumption can be helpful for estimating the value of a token and guaranteeing it is distributed more fairly."
            }
            p {
                "We first need to take a moment to recognize a few fundamental physical properties of energy. "
                "Energy is the fundamental resource of the universe. "
                "All light, all matter, and all motion are simply different forms of energy. "
                "Energy can neither be created nor destroyed."
                "It is the one resource upon which all life and human economy depends. "
                // "All light, matter, and motion are energy. "
                // "It is the fundamental re the physical universe. "
                // "All mass, light, and motion are energy. "
                // ""
            }
            p {
                "It is in this sense that we begin to see how consuming energy can ground a token produced by mining in the physical world. "
                "Every good produced has an opportunity cost. "
                "For a miner who spends energy to acquire new tokens, a token must be worth at least the energy required to produce it, otherwise the miner wouldn't have done it. "
                // "A tokens energetic opportunity cost of production helps define its value. "
                // "When there is a material opportunity cost. "
                // "This is basic economics 101. "
                // "The logic goes something like this: for a miner, a token must be worth at least the energy required to produce it, otherwise the miner wouldn't have spent the energy required to produce it."
                // "For a miner, a token must be worth at least the energy required to produce it, otherwise the miner wouldn't have spent the energy required to produce it. "
                // "This comes from the basic market theory which says simply the value of a good is equal to that which a buyer is willing and able to exchange for it. "
                // "The energetic opportuntity cost of mining is, in part, what gives token some material value."
            }
            p {
                // "So how do we burn energy? "
                // "And more importantly, how do we prove we've burned energy? "
                "How do we know how much energy has been spent?"
                "Well this is where computation comes to the rescue."
            }
            p {
                "TODO"
            }
            p {
                class: "text-xl font-bold mt-4",
                "Proof of work"
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
                "Tokens for everyone"
            }
            p {
                "Everyone with access to a computer can mine. "
                "This is about as fair as we can reasonably get. "
            }
            p {
                "For sure, some people have access to more resources than others, but the important thing is that everyone can participate."
            }
            p {
                "This is what makes Ore special. "
                "For other mineable tokens such as Bitcoin, all miners are in a competition with one another. "
            }
            p {
                class: "text-xl font-bold mt-4",
                "Conclusion"
            }
        }
    }
}
