use dioxus::prelude::*;
use ore_api::consts::TOKEN_DECIMALS;

use crate::{
    components::*,
    gateway::pool::PoolGateway,
    hooks::{
        on_transaction_done, use_gateway, use_member, use_member_record, use_member_record_balance,
        use_miner_claim_transaction, use_miner_cores, use_miner_is_active, use_miner_status,
        use_pool_register_transaction, use_pool_url, use_wallet, MinerStatus, Wallet,
    },
    solana::spl_token::amount_to_ui_amount_string,
};

pub fn Mine() -> Element {
    rsx! {
        Col {
            // class: "w-full h-full pb-20 sm:pb-16 mx-auto",
            class: "w-full h-full pb-20 sm:pb-16",
            gap: 16,
            Col {
                class: "w-full max-w-2xl mx-auto px-5 sm:px-8",
                Heading {
                    class: "w-full",
                    title: "Mine",
                    subtitle: "Utilize spare hashpower to earn rewards."
                }
                OrbMiner {
                    class: "relative flex w-[16rem] h-[16rem] mx-auto my-8 sm:my-16",
                    gold: *use_miner_is_active().read()
                }
                MinerData {}
            }
            MineTable {}
        }
    }
}

fn MinerData() -> Element {
    // Get resources
    let mut member = use_member();
    let mut member_record = use_member_record();

    // Build the claim transaction
    let claim_tx = use_miner_claim_transaction(member);

    // Refresh member account
    on_transaction_done(move |_sig| {
        member.restart();
        member_record.restart();
    });

    rsx! {
        Col {
            class: "w-full flex-wrap mx-auto justify-between",
            gap: 8,
            Alert {}
            MinerStatus {}
            MinerHashpower {}
            MinerPendingRewards {}
            MinerRewards {}
            ClaimButton {
                transaction: claim_tx,
            }
        }
    }
}

fn StopStartButton() -> Element {
    let wallet = use_wallet();
    let pool_url = use_pool_url();
    let mut miner_status = use_miner_status();
    let mut member = use_member();
    let mut member_record = use_member_record();
    let register_tx = use_pool_register_transaction();
    let is_active = use_miner_is_active();

    let mut register_with_pool_server = use_future(move || async move {
        let Wallet::Connected(authority) = *wallet.read() else {
            return;
        };
        let Some(pool_url) = pool_url.cloned() else {
            return;
        };
        if miner_status.cloned() != MinerStatus::Registering {
            return;
        }
        if let Ok(_member_record) = use_gateway().register(authority, pool_url).await {
            member.restart();
            member_record.restart();
            miner_status.set(MinerStatus::FetchingChallenge);
        }
    });

    // on successful registration signature (onchain),
    // restart the pool server registration
    //
    // this is the happy path,
    // first the onchain registration signature lands
    // and then we submit for registration with the offchain pool server
    on_transaction_done(move |_sig| {
        if miner_status.cloned() == MinerStatus::Registering {
            register_with_pool_server.restart();
        }
    });
    // submit pool server registration
    //
    // this is the recovery path,
    // where the onchain registration landed
    // but the server registration failed or hasn't been submitted yet
    use_memo(move || {
        if miner_status.cloned() == MinerStatus::Registering {
            register_with_pool_server.restart();
        }
    });

    let controls_class = if *is_active.read() {
        "controls-secondary"
    } else {
        "controls-primary"
    };

    rsx! {
        button {
            class: "flex flex-row gap-2 my-auto px-8 h-12 rounded-full {controls_class}",
            disabled: matches!(*wallet.read(), Wallet::Disconnected),
            onclick: move |_| {
                // set to stopped (miner is active)
                if *is_active.read() {
                    miner_status.set(MinerStatus::Stopped);
                } else {
                    // stopped
                    // if already registered in pool fetch next challenge
                    // or else try registering (both onchain and then with the pool server)
                    if let Some(Ok(_member)) = member.cloned() {
                        // already registered on chain, check if registered with pool server
                        if let Some(Ok(_member_db)) = member_record.cloned() {
                            // registered with the pool server too, fetch challenge
                            miner_status.set(MinerStatus::FetchingChallenge);
                        } else {
                            // only registered onchain, submit registration to pool server
                            miner_status.set(MinerStatus::Registering);
                        }
                    } else if let Some(Ok(tx)) = register_tx.cloned() {
                        // not registered onchain, submit registration transaction
                        miner_status.set(MinerStatus::Registering);
                        submit_transaction(tx);
                    }
                }
            },
            if !*is_active.read() {
                PlayIcon { class: "my-auto h-5" }
                span {
                    class: "my-auto",
                    "Start"
                }
            } else {
                StopIcon { class: "my-auto h-5"  }
                span {
                    class: "my-auto",
                    "Stop"
                }
            }
        }
    }
}

fn MinerStatus() -> Element {
    let miner_status = use_miner_status();
    let status = use_memo(move || match miner_status.cloned() {
        MinerStatus::Registering => "Registering",
        MinerStatus::FetchingChallenge => "Fetching",
        MinerStatus::Hashing => "Hashing",
        MinerStatus::SubmittingSolution => "Submitting",
        MinerStatus::Stopped => "Stopped",
    });
    rsx! {
        Col {
            gap: 4,
            span {
                class: "text-elements-lowEmphasis font-medium",
                "Status"
            }
            Row {
                class: "justify-between",
                span {
                    class: "font-semibold text-2xl sm:text-3xl",
                    "{status}"
                }
                StopStartButton {}
            }
        }
    }
}

fn MinerHashpower() -> Element {
    rsx! {
        Col {
            gap: 4,
            Row {
                class: "justify-between",
                span {
                    class: "text-elements-lowEmphasis font-medium",
                    "Hashpower"
                }
                span {
                    class: "text-elements-lowEmphasis font-medium",
                    "Cores"
                }
            }
            Row {
                class: "justify-between",
                span {
                    class: "font-semibold text-2xl sm:text-3xl",
                    "1230 H/s"
                }
                span {
                    MinerSelectCores {}
                }
            }
        }
    }
}

fn MinerSelectCores() -> Element {
    let mut cores = use_miner_cores();
    let max = crate::cores::get();
    rsx! {
        Row {
            class: "justify-between",
            button {
                class: "bg-white flex items-center justify-center w-12 h-12 bg-gray-200 border border-gray-300 text-black rounded-l hover:bg-gray-300 active:bg-gray-400",
                onclick: move |_| {
                    let current = cores.peek().clone() - 1;
                    cores.set(current.max(1));
                },
                svg {
                    class: "w-6 h-6",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    view_box: "0 0 24 24",
                    path {
                        d: "M20 12H4",
                    }
                }
            }
            button {
                class: "bg-white flex items-center justify-center w-12 h-12 bg-gray-200 border border-gray-300 text-black rounded-l hover:bg-gray-300 active:bg-gray-400",
                onclick: move |_| {
                    let current = cores.peek().clone() + 1;
                    cores.set(current.min(16));
                },
                svg {
                    class: "w-6 h-6",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    view_box: "0 0 24 24",
                    path {
                        d: "M12 4v16M20 12H4",
                    }
                },
            }
            button {
                class: "w-12 h-12 flex items-center justify-center border border-gray-300 mr-2",
                onclick: move |_| {
                    cores.set(max);
                },
                "Max"
            }
            span {
                class: "w-12 h-12 flex items-center justify-center border border-gray-300",
                "{cores}"
            }
        }
    }
}

fn MinerPendingRewards() -> Element {
    let member = use_member();
    let member_record_balance = use_member_record_balance();
    rsx! {
        if let Some(Ok(member_record_balance)) = member_record_balance.cloned() {
            Col {
                gap: 4,
                span {
                    class: "text-elements-lowEmphasis font-medium",
                    "Rewards (pending)"
                }
                if let Some(member) = member.cloned() {
                    if let Ok(member) = member {
                        OreValue {
                            size: TokenValueSize::Large,
                            ui_amount_string: amount_to_ui_amount_string(member_record_balance - member.total_balance, TOKEN_DECIMALS),
                            with_decimal_units: true,
                        }
                    } else {
                        NullValue {}
                    }
                } else {
                    LoadingValue {}
                }
            }
        }
    }
}

fn MinerRewards() -> Element {
    let member = use_member();
    rsx! {
        Col {
            gap: 4,
            span {
                class: "text-elements-lowEmphasis font-medium",
                "Rewards"
            }
            if let Some(member) = member.cloned() {
                if let Ok(member) = member {
                    OreValue {
                        size: TokenValueSize::Large,
                        ui_amount_string: amount_to_ui_amount_string(member.balance, TOKEN_DECIMALS),
                        with_decimal_units: true,
                        gold: true,
                    }
                } else {
                    NullValue {}
                }
            } else {
                LoadingValue {}
            }
        }
    }
}
