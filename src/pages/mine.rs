use dioxus::prelude::*;
use ore_api::consts::TOKEN_DECIMALS;
use ore_miner_types::OutputMessage;

use crate::{
    components::*,
    gateway::pool::PoolGateway,
    hooks::{
        on_transaction_done, use_gateway, use_member, use_member_record, use_member_record_balance,
        use_miner, use_miner_claim_transaction, use_miner_cores, use_miner_cpu_utilization,
        use_miner_is_active, use_miner_status, use_pool_register_transaction, use_pool_url,
        use_wallet, MinerStatus, Wallet,
    },
    solana::spl_token::amount_to_ui_amount_string,
};
use ore_types::request::TransactionType;

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
                    subtitle: "Earn crypto by expending compute."
                }
                if cfg!(feature = "web") {
                    OrbMiner {
                        class: "relative flex w-[16rem] h-[16rem] mx-auto my-8 sm:my-16",
                        gold: *use_miner_is_active().read()
                    }
                } else {
                    div {
                        class: "h-8",
                    }
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
            // MinerHashpower {}
            if cfg!(not(feature = "web")) {
                MinerCores {}
            }
            MinerPendingRewards {}
            MinerRewards {}
            ClaimButton {
                transaction: claim_tx,
                tx_type: TransactionType::PoolClaim,
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
                        submit_transaction(tx, TransactionType::PoolJoin);
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
    let (out_msg, _in_msg) = use_miner();
    let miner_status = use_miner_status();
    let status = use_memo(move || match miner_status.cloned() {
        MinerStatus::Registering => "Registering",
        MinerStatus::FetchingChallenge => "Fetching",
        MinerStatus::Hashing => "Hashing",
        MinerStatus::SubmittingSolution => "Submitting",
        MinerStatus::Stopped => "Stopped",
    });
    let display_time_remaining = use_memo(move || match miner_status.cloned() {
        MinerStatus::Hashing | MinerStatus::SubmittingSolution | MinerStatus::FetchingChallenge => {
            true
        }
        _ => false,
    });

    let mut time_remaining = use_signal(|| 60);
    use_effect(move || {
        if let OutputMessage::TimeRemaining(time, _) = out_msg.cloned() {
            time_remaining.set(time);
        }
    });

    // TODO: display
    let cpu_utilization = use_miner_cpu_utilization();
    use_memo(move || {
        let vec = cpu_utilization.read();
        log::info!("cpu utilization: {:?}", vec);
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
        if display_time_remaining.cloned() {
            Col {
                gap: 4,
                Row {
                    class: "text-elements-lowEmphasis font-medium justify-between",
                    span {
                        "Time remaining"
                    }
                    span {
                        "{time_remaining.cloned()}s"
                    }
                }
                div {
                    class: "w-full h-2 bg-elements-lowEmphasis rounded-full",
                    div {
                        class: "h-full bg-elements-highEmphasis rounded-full transition-all",
                        style: "width: {(100.0 - (time_remaining.cloned() as f32 / 60.0 * 100.0)).max(0.0)}%"
                    }
                }
            }
        }
    }
}

fn _MinerHashpower() -> Element {
    rsx! {
        Col {
            gap: 4,
            span {
                class: "text-elements-lowEmphasis font-medium",
                "Hashpower"
            }
            span {
                class: "font-semibold text-2xl sm:text-3xl",
                "1230 H/s"
            }
        }
    }
}

fn MinerCores() -> Element {
    let mut cores = use_miner_cores();
    let max = crate::cores::get();
    rsx! {
        Col {
            gap: 4,
            span {
                class: "text-elements-lowEmphasis font-medium",
                "Cores"
            }
            Row {
                class: "justify-between",
                span {
                    class: "font-semibold text-2xl sm:text-3xl",
                    "{cores}"
                }
                Row {
                    gap: 2,
                    button {
                        class: "flex items-center justify-center w-12 h-12 controls-secondary rounded-full text-3xl",
                        onclick: move |_| {
                            let current = cores.peek().clone() - 1;
                            cores.set(current.max(1));
                        },
                        "–"
                    }
                    button {
                        class: "flex items-center justify-center w-12 h-12 controls-secondary rounded-full text-3xl",
                        onclick: move |_| {
                            let current = cores.peek().clone() + 1;
                            cores.set(current.min(max));
                        },
                        "+"
                    }
                }
            }
        }
    }
}

fn MinerPendingRewards() -> Element {
    let member = use_member();
    let member_record_balance = use_member_record_balance();
    rsx! {
        if let Some(Ok(member_record_balance)) = member_record_balance.cloned() {
            if let Some(Ok(member)) = member.cloned() {
                if member_record_balance > member.total_balance {
                    Col {
                        gap: 4,
                        span {
                            class: "text-elements-lowEmphasis font-medium",
                            "Rewards (pending)"
                        }
                        OreValue {
                            size: TokenValueSize::Large,
                            ui_amount_string: amount_to_ui_amount_string(member_record_balance - member.total_balance, TOKEN_DECIMALS),
                            with_decimal_units: true,
                        }
                    }
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
