use dioxus::prelude::*;
use solana_sdk::pubkey::Pubkey;

use crate::{
    components::{Col, PairWithdrawForm, Row}, config::{BoostMeta, LISTED_TOKENS}, gateway::{kamino::KaminoGateway, GatewayError}, hooks::{use_gateway, use_wallet, Wallet}
};
use super::common::*;

#[component]
pub fn PairStakeForm(class: Option<String>, boost_meta: BoostMeta) -> Element {
    let class = class.unwrap_or_default();
    let tab = use_signal(|| StakeTab::Deposit);
    rsx! {
        Col {
            class: "{class}",
            StakeTabs {
                tab: tab
            }
            match *tab.read() {
                StakeTab::Deposit => rsx! {
                    PairDepositForm {
                        boost_meta: boost_meta
                    }
                },
                StakeTab::Withdraw => rsx! {
                    PairWithdrawForm {
                        boost_meta: boost_meta
                    }
                }
            }
        }
    }
}

#[component]
fn PairDepositForm(class: Option<String>, boost_meta: BoostMeta) -> Element {
    let class = class.unwrap_or_default();
    let wallet = use_wallet();
    let stake_amount_a = use_signal::<String>(|| "".to_owned());
    let stake_amount_b = use_signal::<String>(|| "".to_owned());
 
    // Build the deposit instruction
    let deposit_ix = use_resource(move || async move {
        // Check if wallet is connected
        let Wallet::Connected(authority) = *wallet.read() else {
            return Err(GatewayError::WalletDisconnected);
        };

        // Parse amounts
        let Ok(amount_a) = stake_amount_a.cloned().parse::<f64>() else {
            return Err(GatewayError::Unknown);
        };
        let Ok(amount_b) = stake_amount_b.cloned().parse::<f64>() else {
            return Err(GatewayError::Unknown);
        };
        if amount_a == 0f64 || amount_b == 0f64 {
            return Err(GatewayError::Unknown);
        }

        // Build the instruction
        use_gateway().build_deposit_instruction(
            boost_meta.lp_id,
            amount_a,
            amount_b,
            authority,
        ).await
    });

    log::info!("{:?}", deposit_ix);

    rsx! {
        Col {
            class: "w-full {class}",
            gap: 4,
            Col {
                class: "lg:flex elevated elevated-border shrink-0 h-min rounded-xl z-0",
                StakeInputs {
                    mint: boost_meta.pair_mint,
                    amount_a: stake_amount_a,
                    amount_b: stake_amount_b,
                }
            }
            // StakeDetails {}
            SubmitButton {
                enabled: if let Some(Ok(_ix)) = deposit_ix.cloned() {
                    true
                } else {
                    false
                },
                onclick: move |_| {
                    // TODO: Implement staking logic
                }
            }
        }
    }
}

#[component]
fn StakeInputs(
    mint: Pubkey,
    amount_a: Signal<String>,
    amount_b: Signal<String>,
) -> Element {
    let token = LISTED_TOKENS.get(&mint).unwrap();
    rsx! {
        Col {
            class: "w-full p-4",
            gap: 2,
            Row {
                class: "justify-between",
                span {
                    class: "text-elements-midEmphasis my-auto pl-1",
                    "Deposit"
                }
                MaxButton {}
            }
            Col {
                gap: 4,
                Row {
                    gap: 4,
                    Row {
                        class: "my-auto",
                        gap: 2,
                        img {
                            class: "w-8 h-8 rounded-full",
                            src: "{token.image}",
                        }
                        span {
                            class: "font-semibold my-auto",
                            "{token.ticker}"
                        }
                    }
                    input {
                        class: "text-3xl placeholder:text-gray-700 font-semibold bg-transparent h-10 pr-1 w-full outline-none text-right [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none",
                        placeholder: "0",
                        r#type: "number",
                        inputmode: "decimal",
                        value: amount_a.cloned(),
                        oninput: move |e| {
                            let s = e.value();
                            if s.len().eq(&0) || s.parse::<f64>().is_ok() {
                                amount_a.set(s);
                            } else {
                                amount_a.set(s[..s.len()-1].to_string());
                            }
                        }
                    }
                }
                Row {
                    gap: 4,
                    Row {
                        class: "my-auto",
                        gap: 2,
                        img {
                            class: "w-8 h-8 rounded-full",
                            src: asset!("/public/icon.png"),
                        }
                        span {
                            class: "font-semibold my-auto",
                            "ORE"
                        }
                    }
                    input {
                        class: "text-3xl placeholder:text-gray-700 font-semibold bg-transparent h-10 pr-1 w-full outline-none text-right [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none",
                        placeholder: "0",
                        r#type: "number",
                        inputmode: "decimal",
                        value: amount_b.cloned(),
                        oninput: move |e| {
                            let s = e.value();
                            if s.len().eq(&0) || s.parse::<f64>().is_ok() {
                                amount_b.set(s);
                            } else {
                                amount_b.set(s[..s.len()-1].to_string());
                            }
                        }
                    }
                }
            }
        }
    }
}


fn MaxButton() -> Element {
    rsx! {
        button {
            class: "text-xs my-auto py-1 px-3 rounded-full bg-gray-800",
            onclick: move |_| {
                // TODO: Implement max amount logic
            },
            "Max"
        }
    }
}