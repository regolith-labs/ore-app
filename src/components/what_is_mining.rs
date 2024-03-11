use dioxus::prelude::*;

#[component]
pub fn WhatIsMining(cx: Scope) -> Element {
    render! {
        div {
            class: "flex flex-col gap-4 font-hero max-w-3xl w-full mx-auto pb-32",
            h2 {
                "What is mining?"
            }
            p {
                "Many people have heard of crypto mining, but few truly understand how it works and only a handful of people have ever actually done it. "
                "So what is mining and why should you care about it? "
            }
            p {
                span {
                    class: "font-bold",
                    "Mining is the process of producing digital tokens by consuming energy. "
                }
                "This is a really big idea with the potential to reshape the world economy. In this post, we'll drill down into crypto mining and unpack what it means for you one concept at a time."
            }
            p {
                class: "text-xl font-bold mt-8",
                "Producing tokens"
            }
            p {
                "Let's start with understanding tokens. "
                "What are they? "
            }
            p {

                span {
                    class: "font-bold",
                    "Tokens are digital assets. "
                }
                "Like the coins you might carry in your pocket or trading cards you might collect, tokens are just digital objects you can own, trade, and transfer over the internet. "
            }
            p {
                "Tokens can come in many different shapes and sizes. "
                "Many of the tokens people are most familiar with are those associated with popular blockchains such as Bitcoin (BTC), Ethereum (ETH), and Solana (SOL). "
                "However tokens can represent anything of value such as an ownership stake in a company or a product like a concert ticket. "
                "Many tokens are associated with cultural symbols such as memes, celebrities, and breeds of dog. "
            }
            p {
                "Blockchains allow anyone with a computer to mint their own tokens and share them with the world in any way they like. "
                "When it comes to mining, the thing we care about is not so much what the tokens represent per se, but rather how they are produced. "
                "Mining is simply a special process for producing tokens where the issuance of new supply is managed by computer code and based on how much energy a user has consumed."
            }
            p {
                class: "text-xl font-bold mt-8",
                "Consuming energy"
            }
            p {
                "At a glance, consuming energy to create digital tokens might seem a little alarming. "
                "Energy after all can be scarce and difficult to acquire; we shouldn't waste it. "
                "However on deeper inspection, we will see how energy consumption can be useful for reasoning about the economic value of a given token and guaranteeing it is distributed fairly."
            }
            p {
                "Let's first take a moment to recall some basic facts about energy. "
                "For starters, our universe is made of energy. "
                "Physics tells us that all light, all matter, and all motion are simply different forms of energy. "
                "It is universally conserved resource – meaning, it can neither be created nor destroyed. "
                "All energy existing in the universe today has been here since the very origin of the universe. "
            }
            p {
                "As humans, we harness energy from the universe to do useful work. "
                "We have a portfolio of technologies we use to tap into our planet's energy sources. "
                "We burn oil, deploy solar cells, construct hydroelectric dams, stand up wind turbines, and operate nuclear reactors. "
                "We tend to utilize every energy source we can find, and by all meaningful metrics, our demand for energy is infinite. "
                "Energy is the foundational resource upon which all life and human economy depends. "
                "It is with this perspective that we can begin to see why energy is so important for mining. "
            }
            p {
                span {
                    class: "font-bold",
                    "Energy consumption has the power to ground tokens in the physical world. "
                }
                "When a token has a material production cost measured in energy, it is easier to estimate its value relative to other goods. "
                "For the miner who spends energy on acquiring new tokens, a token must be worth at least the energy required to produce it, otherwise the miner simply wouldn't be mining. "
                "This is the basic principle of exchange that underlies all human economic theory. "
            }
            p {
                "We also should note that the energy cost of producing a token does not directly determine its market price. "
                "Prices are free floating values, subject to speculation, and set by the dynamic forces of supply and demand. "
                "Mining simply helps market particpants reason about how much energy other people are willing to sacrafice in order to acquire a particular token. "
            }
            p {
                "But this now presents us with a problem. "
                "If a token has a real economic value – in part, because it has a real energy cost – how do we guarnatee the energy cost is actually real? "
                "That is, how do prove some energy has been consumed without relying on someone's word for it? "
                "How do we detect a liar? "
                "This is where computers and computation come to help. "
            }
            p {
                class: "text-xl font-bold mt-8",
                "Proof of work"
            }
            p {
                "Imagine you are given a puzzle. "
                "It's a very difficult puzzle to solve in your head or on paper, but your computer can solve it in a few seconds. "
                "Mathematically, the only known way to solve the puzzle is to generate thousands of potential solutions and check each one to see if it's correct. "
                "Based on how difficulty the puzzle is configured to be, we can estimate how many solutions on average your computer will need to generate before it finds one that is correct. "
            }
            p {
                span {
                    class: "font-bold",
                    "This is how we use computation to approximate energy consumption. "
                }
                "Every solution your computer generates costs a small, known quantity of energy. "
                "Simply by presenting a valid solution to the puzzle, you are proving to the world you must have generated some average number of invalid solutions; thereby doing computational work that consumed energy. "
                "This is where the phrase \"proof-of-work\" comes from. "
            }
            p {
                "Importantly, it is very easy for anyone to verify if a solution is correct without having to re-solve the puzzle from scratch. "
                "This allows users to easily share their solutions with the world and have them publicly verified on a blockchain network. "
                "A mining protocol is a special kind of smart-contract on a blockchain that mints new tokens when a miner presents it with a valid solution. "
            }
            p {
                "Always keep in mind that other than being correct, the solutions produced by a miner are meaningless and unimportant. The output of the mining process is the token, not the calculation. "
            }
            p {
                class: "text-xl font-bold mt-8",
                "Fair distribution"
            }
            p {
                "Now that we have a decent understanding of mining and how it works, we come to a very different kind of challenge: defining fairness. "
                "There are many ways to setup a mining protocol, and some lead to more bias than others. "
            }
            p {
                "For Ore, the important principle is that everyone can participate in mining. "
                "The primary innovation of Ore is to offer a fair mining protocol which guarantees non-exclusive mining rewards. "
                "This means if one miner wins, it doesn't prevent another miner from winning also. "
                span {
                    class: "font-bold",
                    "This is what makes Ore unique and unlike any other digital token in the world. "
                }
            }
            p {
                "For traditional mineable cryptocurrencies such as Bitcoin, mining rewards are primarily exclusive. "
                "That is, only one Bitcoin miner can win at a time, and there can only be one winner approximately every ~10 minutes. "
                "This has had the longterm effect of starving out casual miners who have been unable to keep up in the arms race against much larger professional mining firms. "
                "Ore takes a different approach."
            }
            p {
                "Rather than setting up every miner in a winner-take-all competition against one another, Ore gives each miner their own individual computational challenge. "
                "As long as a miner provides a valid solution to their own personal puzzle, Ore guarantees that miner will earn a piece of the supply. "
                "Since no miner can be censored from the network and all valid solutions are non-exclusive, starvation is prevented. "
            }
            p {
                "This has never been done before in the history of cryptocurrency, and is only possible due to the technical breakthroughs of high performance blockchains such as "
                a {
                    class: "font-medium hover:text-green-500 hover:underline",
                    href: "https://solana.com",
                    "Solana."
                }
            }
            p {
                class: "text-xl font-bold mt-8",
                "Conclusion"
            }
            p {
                "We now have everything we need to understand why mining is such a revolutionary new idea. "
                span {
                    class: "font-bold",
                    "Mining presents an algorithmic alternative to discretionary monetary policy. "
                }
            }
            p {
                "Monetary policy is the process by which central bankers control a nation's money supply. "
                "These officials tend to be unelected, unpredictable, and subject to policitcal pressures. "
                "Mining, on the other hand, is managed by computer code. "
                "It is predictable, tested, impartial, an open-source. "
            }
            p {
                "Mining offers a chance to level the playing field where it has historically been biased in favor of large banks. "
                "For example, when a central banks issues new currency, it tends to distribute it first to the accounts of the country's largest banks. "
                "These entities are the first to benefit from the injection of new money and kickoff a new spending spree. "
                "This money only trickles down to rest of the economy after the effects of inflation has already begun to kick in. "
            }
            p {
                "Mining offers everyday people the chance to earn new tokens directly from the source, without intermediaries. "
                "It's programmatic nature promises to bring stability and predictability to a process that has historically been characterized by its inherent uncertainty. "
            }
            p {
                "Where earlier cryptocurrency mining has traditionally been dominated by large well-resourced players, Ore promises a new world where mining is available to everyone. "
            }
        }
    }
}
