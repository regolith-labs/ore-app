use dioxus::prelude::*;

#[component]
pub fn HowItWorks(cx: Scope) -> Element {
    render! {
        div {
            class: "flex flex-col gap-4",
            h2 {
                "What is crypto mining?"
            }
            p {
                "Coming soon..."
            }
            p {
                "The goal is to distribute tokens as fairly as possible. This requires to define 'fairness'"
            }
            p {
                "There are many ways to define fairness. But we shall define it as tokens shall be distributed to those to prove they a certain level of computational ability."
            }
            p {
                "At it's core, it's simple. 'Proof-of-work' is simply proof that you've done some work."
            }
            p {
                "You have a computational challenge. It takes you a long time to solve the challenge, but is very easy to verify if your solution is correct."
            }
            p {
                "Ore uses a special hash function called SHA-3."
            }
            p {
                "Your computer calculates a bunch of different hashes, trying to find one that satisfies the difficulty requirement."
            }
            p {
                "When it finds a valid hash, it submits it to the blockchain for verification. If your solution is valid, you earn a reward."
            }
            p {
                "The hash solution is meaningless and unimportant. The output of the process is the token itself."
            }
        }
    }
}
