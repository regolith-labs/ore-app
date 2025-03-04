use dioxus::prelude::*;

use crate::{components::*, config::Token, hooks::use_token_balance, route::Route};

// TODO Web top up form
// TODO Destop modal that links to web form

#[cfg(feature = "web")]
#[component]
pub fn Topup(address: String) -> Element {
    enum TopupStatus {
        Editing,
        Success,
    }

    let destination = use_memo(move || Pubkey::from_str(&address));
    let mut amount = use_signal(|| "0.2".to_string());
    let sol_balance = use_token_balance(Token::sol().mint);
    let priority_fee = use_signal(|| 0);
    let err = use_signal::<Option<TokenInputError>>(|| None);
    let tx = use_topup_transaction(destination, amount, sol_balance, err, priority_fee);
    let mut status = use_signal(|| TopupStatus::Editing);

    on_transaction_done(move |_| {
        status.set(TopupStatus::Success);
    });

    rsx! {
        Col {
            class: "w-full h-full max-w-2xl mx-auto px-5 sm:px-8 pb-20 sm:pb-16",
            gap: 8,
            Heading {
                class: "mx-auto w-full",
                title: "Top up",
                subtitle: "Transfer SOL to your desktop wallet."
            }

            match *status.read() {
                TopupStatus::Editing => {
                    rsx! {
                        // Destination
                        Col {
                            class: "mx-auto w-full",
                            gap: 2,
                            span {
                                class: "text-elements-lowEmphasis font-medium",
                                "Destination"
                            }
                            if let Ok(destination) = destination.cloned() {
                                span {
                                    class: "text-elements-highEmphasis font-mono",
                                    "{destination}"
                                }
                            } else {
                                span {
                                    "Pubkey is invalid"
                                }
                            }
                        }


                        // Amount
                        Col {
                            class: "mx-auto w-full",
                            gap: 2,
                            span {
                                class: "text-elements-lowEmphasis font-medium",
                                "Amount"
                            }
                            Row {
                                class: "w-full justify-between",
                                input {
                                    class: "text-3xl placeholder:text-gray-700 font-semibold bg-transparent h-12 my-auto w-32 text-left outline-none text-elements-highEmphasis [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none",
                                    placeholder: "0.1",
                                    r#type: "number",
                                    step: "any",
                                    inputmode: "decimal",
                                    value: amount.clone(),
                                    oninput: move |e: FormEvent| amount.set(e.value()),
                                }
                                Row {
                                    gap: 2,
                                    button {
                                        class: "flex items-center justify-center w-12 h-12 shrink-0 controls-secondary rounded-full text-3xl",
                                        onclick: move |_| {
                                            let new_amount = ((amount.read().parse::<f64>().unwrap() - 0.1) * 10.0).round() / 10.0;
                                            let new_amount = new_amount.max(0.0);
                                            amount.set(new_amount.to_string());
                                        },
                                        "–"
                                    }
                                    button {
                                        class: "flex items-center justify-center w-12 h-12 shrink-0 controls-secondary rounded-full text-3xl",
                                        onclick: move |_| {
                                            let new_amount = ((amount.read().parse::<f64>().unwrap() + 0.1) * 10.0).round() / 10.0;
                                            amount.set(new_amount.to_string());
                                        },
                                        "+"
                                    }
                                }
                            }
                        }

                        SubmitButton {
                            title: "Send SOL".to_string(),
                            transaction: tx,
                            err: err,
                            tx_type: TransactionType::TopUp
                        }
                    }
                }
                TopupStatus::Success => {
                    rsx! {
                        Col {
                            class: "mx-auto w-full",
                            gap: 8,
                            span {
                                class: "text-elements-highEmphasis font-semibold text-2xl mx-auto",
                                "Success!"
                            }
                            a {
                                class: "flex controls-primary w-full h-12 rounded-full hover:cursor-pointer",
                                href: "ore://",
                                span {
                                    class: "mx-auto my-auto",
                                    "Return to app"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(not(feature = "web"))]
#[component]
pub fn Topup(address: String) -> Element {
    use crate::hooks::{use_wallet, Wallet};

    let wallet = use_wallet();
    let navigator = use_navigator();
    let mut sol_balance = use_token_balance(Token::sol().mint);

    use_effect(move || {
        if let Some(Ok(balance)) = sol_balance.cloned() {
            if balance.amount.parse::<u64>().unwrap() > 0 {
                navigator.push(Route::Mine {});
            }
        }
    });

    use_effect(move || {
        spawn(async move {
            let mut t = 0;
            loop {
                sol_balance.restart();
                async_std::task::sleep(std::time::Duration::from_millis(2000)).await;
                t += 1;
                if t > 600 {
                    // Wait 20 minutes and then stop polling
                    break;
                }
            }
        });
    });

    rsx! {
        Col {
            class: "w-full h-full max-w-2xl mx-auto px-5 sm:px-8 pb-20 sm:pb-16",
            gap: 16,
            Heading {
                class: "mx-auto w-full",
                title: "Welcome!",
                subtitle: "To get started, top up your desktop wallet."
            }

            Col {
                class: "mx-auto my-4 w-full",
                gap: 2,
                span {
                    class: "text-elements-lowEmphasis font-medium mx-auto",
                    "Wallet balance"
                }
                span {
                    class: "text-elements-highEmphasis font-semibold text-3xl mx-auto",
                    "0 SOL"
                }
            }

            // TODO Generate QR code

            if let Wallet::Connected(address) = *wallet.read() {
                a {
                    class: "flex controls-primary w-full h-12 rounded-full hover:cursor-pointer",
                    href: "https://ore.supply/topup/{address}",
                    span {
                        class: "mx-auto my-auto",
                        "Top up →"
                    }
                }
            }
        }
    }
}
