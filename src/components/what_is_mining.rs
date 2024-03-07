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
                "Most people have heard of crypto mining, but not as many underestand what it is and only a handful have ever done it. "
                "So what is mining and how does it work?"
            }
            p {
                span {
                    class: "font-bold",
                    "Mining is the process of producing tokens by consuming energy. "
                }
            }
            p {
                "There's a bit to unpack here so we'll break it down one concept at a time."
            }
            p {
                class: "text-xl font-bold mt-4",
                "Producing tokens"
            }
            p {
                "Let's start with tokens. "
                "Tokens are just digital assets. "
                "Like coins you might carry in your pocket or trading cards you might collect, tokens are digital objects you can own, trade, and transfer over the internet. "
            }
            p {
                "Tokens come in many different shapes and sizes. "
                "Some of the most familiar tokens are those associated with popular blockchains such as Bitcoin (BTC), Ethereum (ETH), or Solana (SOL). "
                "However tokens can represent anything such as an ownership stake in a company or a product like a concert ticket. "
                "Many tokens are associated with cultural symbols such as memes, celebrities, or even breeds of dog. "
            }
            p {
                "When it comes to understanding mining, the thing we care about is not what the tokens represent per se, but rather how they are produced. "
                "Blockchains allow anyone with a computer to mint and distribute tokens in whatever manner they like. "
                "Mining is simply a special kind of token distribution process – managed by computer code – that mints and issues new tokens based on consuming energy. "
            }
            p {
                class: "text-xl font-bold mt-4",
                "Consuming energy"
            }
            p {
                "At first glance, spending energy to create tokens might seem a little alarming. "
                "Energy is scarce after all; it can be expensive to generate and we shouldn't waste it. "
                "However on deeper inspection, we will see that energy consumption can be helpful in determining the value of a token and guaranteeing it is distributed fairly."
            }
            p {
                "Energy is the fundamental resource of the universe upon which all life and human economy depends. "
                "Therefore, it is the consumption of energy that grounds tokens produced by mining to the physical world. "
                "A tokens energetic opportunity cost of production helps define its value. "
                // "All light, matter, and motion are energy. "
                // "It is the fundamental re the physical universe. "
                // "All mass, light, and motion are energy. "
                // ""
            }
            p {
                // "When there is a material opportunity cost. "
                // "This is basic economics 101. "
                // "The logic goes something like this: for a miner, a token must be worth at least the energy required to produce it, otherwise the miner wouldn't have spent the energy required to produce it."
                "For a miner, a token must be worth at least the energy required to produce it, otherwise the miner wouldn't have spent the energy required to produce it. "
                "This comes from the basic market theory which says simply the value of a good is equal to that which a buyer is willing and able to exchange for it. "

                // "The energetic opportuntity cost of mining is, in part, what gives token some material value."
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
                "Equal opportunity"
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
                "Putting it all together"
            }
        }
    }
}
