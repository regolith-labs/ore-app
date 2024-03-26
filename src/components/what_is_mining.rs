use dioxus::prelude::*;

#[component]
pub fn WhatIsMining(cx: Scope) -> Element {
    render! {
        div {
            class: "flex flex-col gap-4 h-full font-hero max-w-3xl w-full mx-auto pb-20 leading-7",
            p {
                class: "text-3xl sm:text-4xl md:text-5xl lg:text-6xl font-bold mb-8 font-hero",
                "What is mining?"
            }
            p {
                span {
                    class: "font-bold",
                    "Crypto mining is the process of producing digital tokens by consuming energy. "
                }
                "This is a really big idea with major potential implications for the world economy. "
                "In this post, we'll drill down into details of mining and unpack what it means for you, one concept at a time."
            }
            p {
                class: "text-lg sm:text-xl md:text-2xl lg:text-3xl font-bold mt-8 font-hero",
                "Producing tokens"
            }
            p {
                span {
                    class: "font-bold",
                    "Tokens are digital assets. "
                }
                "Like the coins you might carry in your pocket or trading cards you might collect, tokens are digital objects you can own, trade, and transfer over the internet. "
                "Blockchains allow anyone with a computer to create their own tokens and share them with the world. "
                "Tokens can represent anything – from concert tickets, to ownership in a company, to fun jokes or memes. "
                "When it comes to mining, the thing we care about is not so much what the tokens represent per se, but rather how they are produced. "
                "Mining is a special process for producing tokens where issuance is managed by computer code and based on how much energy a user has provably consumed."
            }
            p {
                class: "text-lg sm:text-xl md:text-2xl lg:text-3xl font-bold mt-8 font-hero",
                "Consuming energy"
            }
            p {
                span {
                    class: "font-bold",
                    "Energy consumption grounds tokens in the physical world. "
                }
                "When a token has a material cost of production measured in energy, it means a miner has to sacrafice something of real economic value to acquire it. "
                "To miners, a token must be worth at least the energy required to produce it – otherwise, they simply wouldn't be mining. "
                "This is the basic principle of exchange that underlies all market theory. "
            }
            p {
                "A token's cost of production can be a useful data point for estimating its value relative to other goods. "
                "It is important to note that production costs are not the only factor which can influence an asset's market price. "
                "Prices are free floating values, subject to speculation, and set by the dynamic forces of supply and demand. "
                "Mining simply helps market particpants reason about how much energy other people are willing to sacrifice in order to acquire a particular token. "
            }
            p {
                class: "text-lg sm:text-xl md:text-2xl lg:text-3xl font-bold mt-8 font-hero",
                "Proof of work"
            }
            p {
                span {
                    class: "font-bold",
                    "Computation is a proxy measure for energy consumption. "
                }
                "If a token has real economic value – in part, because it has a real energy cost – it's important we can prove the energy cost has actually been paid. "
                "To do this, your computer is given a computational puzzle it can only solve by generating thousands of potential solutions and checking each one for correctness. "
                "Each solution costs a small, known quantity of energy to generate. "
                "Based on the difficulty of the puzzle, we can estimate how many potential solutions your computer will need to generate before it finds one that is valid. "
                "Simply by presenting a valid solution, you can prove your computer must have done the computational work to find it and therefore consumed energy. "
                "This is where the phrase \"proof-of-work\" comes from. "
            }
            p {
                class: "text-lg sm:text-xl md:text-2xl lg:text-3xl font-bold mt-8 font-hero",
                "Fair distribution"
            }
            p {
                span {
                    class: "font-bold",
                    "Ore guarantees everyone can participate in mining and win. "
                }
                "It does this by using a novel fair mining protocol with non-exclusive rewards. "
                "This means if one miner wins, it doesn't prevent another miner from winning also. "
                "Rather than setting up every miner in a winner-take-all competition against one another, Ore gives each miner their own personalized computational challenge. "
                "As long as you provide a valid solution to your own individual puzzle, Ore guarantees your will earn a piece of the supply. "
                "Since no miner can be censored from the network and all valid solutions are non-exclusive, starvation is avoided. "
            }
            p {
                "This algorithm is what makes Ore unique and unlike any other digital token in the world. "
                "It has never been tried before in the history of cryptocurrency and is only possible due to the recent breakthroughs of high performance blockchains like "
                a {
                    class: "font-semibold hover:text-green-500 underline transition-colors",
                    href: "https://solana.com",
                    "Solana"
                }
                ". "
            }
            p {
                class: "text-lg sm:text-xl md:text-2xl lg:text-3xl font-bold mt-8 font-hero",
                "Conclusion"
            }
            p {
                span {
                    class: "font-bold",
                    "Mining is an algorithmic alternative to central banking. "
                }
                "Where traditional fiat currencies are managed by unelected officials and opaque boards of central bankers, mining offers an alternative form of currency control that is codified, predictable, testable, and open-source. "
                "It promises to level the playing field in a system that has historically been biased in favor of the world's largest banks and corporations. "
                "Ore opens the door to everyone and offers a digital currency that regular, everyday people can mine and win. "
            }
        }
    }
}
