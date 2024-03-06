use dioxus::prelude::*;

#[component]
pub fn HowItWorks(cx: Scope) -> Element {
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
                "Most people have heard of crypto mining, but few truly underestand it and only a handful have ever done it. So what is it?"
            }
            p {
                span {
                    class: "font-bold",
                    "Mining is simply the act of producing tokens by burning energy. "
                 // algorithm a protocol uses to distribute tokens. "
                }
                // "When creating a mining algorithm, protocol designers have to decide what types of behaviors they are going to incentivize."
                // "This can take many forms such or running an internet hotspot, giving a passenger a ride, or even counting your steps."
            }
            p {
                // "The goal is to distribute tokens as fairly as possible. "
                // "This requires us to define 'fairness'. "
                // "There are many different ways to approach this question, but we shall limit ourselves only to those definitions which can computational be proved have happened."
                "There's a little bit to unpack there so we'll break it down one concept at a time."
            }
            p {
                class: "text-xl font-bold mt-4",
                "Producing tokens"
            }
            p {
                "First, let's start with tokens. "
                "Tokens are just countable digital assets. "
                "Like a coin you might carry in your pocket or a trading card you might collect, tokens are a digital asset you can own and send over the internet. "
            }
            p {
                "There are many kinds of tokens you might be familiar with. "
                "Many of the most familiar tokens are associated with popular blockchains such as Bitcoin (BTC), Ethereum (ETH), or Solana (SOL)."
                // "This is where the term 'proof-of-work' comes from. "
            }
            p {
                "However tokens can come in all sorts of shapes and sizes. "
                "Some tokens might be associated with a specific company while others with an asset like a car or a concert ticket. Even others tokens still might be associated with cultural symbols such as memes, celebrities, or dogs. "
            }
            p {
                "When it comes to mining, the important thing is not so much what the token is associated with, so much as how it's produced. "
            }
            p {
                class: "text-xl font-bold mt-4",
                "Burning energy"
            }
            p {
                "Let's first establish why burning energy is important. "
                "Energy is scarce! We can never get enough of it. "
            }
            p {
                "Burning energy grounds the token in physical economy. "
                "It creates a material opportunity cost for each one produced. "
                "To the miner, the token must be worth at least the energy consumed, otherwise they wouldn't have spent it producing the token."
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
        }
    }
}
