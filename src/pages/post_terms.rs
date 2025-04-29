use dioxus::prelude::*;

use crate::components::{Col, Heading};

pub fn PostTerms() -> Element {
    rsx! {
        Col {
            class: "w-full h-full pb-20 sm:pb-16",
            gap: 8,
            Heading {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                title: "Terms and Conditions",
                subtitle: "ORE Creator Program"
            }
            Col {
                class: "mx-auto w-full max-w-2xl px-5 sm:px-8",
                gap: 4,
                // div {
                //     class: "text-elements-midEmphasis",
                //     "Effective Date: [Insert Date]"
                // }
                p {
                    class: "text-elements-midEmphasis",
                    "These Terms and Conditions (\"Terms\") govern your participation in the ORE Creator Program (the \"Program\") offered by Regolith Labs Inc. (\"we\", \"us\", or \"Company\"). By participating in the Program, you (\"you\" or \"User\") agree to be bound by these Terms."
                }
                div {
                    class: "mt-6",
                    h2 {
                        class: "text-xl font-semibold mb-2",
                        "1. Program overview"
                    }
                    p {
                        class: "text-elements-midEmphasis mb-4",
                        "The Creator Program enables Users to authenticate their X (formerly Twitter) account and submit or share content for consideration via our platform. Eligible users may receive a public creator score based on the Company's evaluation of submitted content. This published score may be used by decentralized and open-source smart contract systems to distribute token rewards. Rewards are not issued, controlled, or distributed by the Company. The Company operates solely as an oracle operator that assigns and publishes content scores on chain."
                    }
                }
                div {
                    class: "mt-6",
                    h2 {
                        class: "text-xl font-semibold mb-2",
                        "2. Oracle control"
                    }
                    p {
                        class: "text-elements-midEmphasis mb-4",
                        "You acknowledge and agree the Company retains sole and absolute control over the operation of the oracle, including how scores are assigned, updated, or revoked. We may modify, suspend, or disable the oracle service at any time, for any reason, without notice and at our sole discretion. Your participation and any rewards dependent on the oracle scoring mechanism are not guaranteed and may be changed or terminated without compensation."
                    }
                }
                div {
                    class: "mt-6",
                    h2 {
                        class: "text-xl font-semibold mb-2",
                        "3. X account and data usage"
                    }
                    p {
                        class: "text-elements-midEmphasis mb-4",
                        "By participating in the Program, you consent to allow Regolith Labs Inc. to access and use your authenticated X account data, including but not limited to: your posts, engagement metrics, follower data, profile metadata, and related content. You also consent to the use of your X account data for the evaluation of your submissions, as well as for product development, analytics, marketing, and other business purposes. "
                    }
                }
                div {
                    class: "mt-6",
                    h2 {
                        class: "text-xl font-semibold mb-2",
                        "4. No financial guarantees"
                    }
                    p {
                        class: "text-elements-midEmphasis mb-4",
                        "Regolith Labs does not issue or distribute any tokens. Any rewards received are minted and distributed by an independent, permissionless smart contract protocols. The value of rewards is determined by market conditions and the smart contract's logic, not by the Company. Participation in the Program does not constitute a financial investment and should not be construed as one."
                    }
                }
                div {
                    class: "mt-6",
                    h2 {
                        class: "text-xl font-semibold mb-2",
                        "5. Intellectual property"
                    }
                    p {
                        class: "text-elements-midEmphasis mb-4",
                        "By submitting content, you grant Regolith Labs Inc. a non-exclusive, worldwide, royalty-free, perpetual license to use, reproduce, modify, distribute, and display the content for promotional, business, or research purposes. You retain ownership of your content and any intellectual property rights therein."
                    }
                }
                div {
                    class: "mt-6",
                    h2 {
                        class: "text-xl font-semibold mb-2",
                        "6. User representations"
                    }
                    p {
                        class: "text-elements-midEmphasis mb-4",
                        "You represent and warrant that you are the lawful owner or authorized user of the X account you connect. Any content submitted is original and does not infringe upon the intellectual property or rights of any third party. You are at least 18 years old or the age of majority in your jurisdiction."
                    }
                }
                div {
                    class: "mt-6",
                    h2 {
                        class: "text-xl font-semibold mb-2",
                        "7. Termination"
                    }
                    p {
                        class: "text-elements-midEmphasis",
                        "We reserve the right to terminate or suspend your access to the Program at any time, without notice, if we believe you have violated these Terms or engaged in fraudulent, abusive, or unlawful behavior."
                    }
                }
                div {
                    class: "mt-6",
                    h2 {
                        class: "text-xl font-semibold mb-2",
                        "8. Disclaimers"
                    }
                    p {
                        class: "text-elements-midEmphasis mb-4",
                        "The Program is provided \"as is\" with no warranties, express or implied. We do not guarantee the availability, accuracy, or functionality of the oracle or any associated rewards. Participation is at your own risk."
                    }
                }
                div {
                    class: "mt-6",
                    h2 {
                        class: "text-xl font-semibold mb-2",
                        "9. Limitation of liability"
                    }
                    p {
                        class: "text-elements-midEmphasis",
                        "To the fullest extent permitted by law, Regolith Labs Inc. shall not be liable for any indirect, incidental, special, or consequential damages arising out of or in connection with your participation in the Program, even if advised of the possibility of such damages."
                    }
                }
                div {
                    class: "mt-6",
                    h2 {
                        class: "text-xl font-semibold mb-2",
                        "10. Indemnification"
                    }
                    p {
                        class: "text-elements-midEmphasis",
                        "You agree to indemnify and hold harmless Regolith Labs Inc. and its affiliates from any claims, losses, liabilities, damages, and expenses arising from your participation in the Program, your content, or your violation of these Terms."
                    }
                }
                div {
                    class: "mt-6",
                    h2 {
                        class: "text-xl font-semibold mb-2",
                        "11. Governing law"
                    }
                    p {
                        class: "text-elements-midEmphasis",
                        "These Terms shall be governed by and construed in accordance with the laws of the State of Deleware, without regard to its conflict of law principles."
                    }
                }
                div {
                    class: "mt-6 mb-8",
                    h2 {
                        class: "text-xl font-semibold mb-2",
                        "12. Changes to terms"
                    }
                    p {
                        class: "text-elements-midEmphasis",
                        "We reserve the right to update or modify these Terms at any time. Changes will be effective immediately upon posting. Continued participation in the Program after changes constitutes your acceptance of the revised Terms."
                    }
                }
            }
        }
    }
}
