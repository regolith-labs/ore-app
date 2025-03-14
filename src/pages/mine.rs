use dioxus::prelude::*;
use ore_api::consts::TOKEN_DECIMALS;
use ore_miner_types::OutputMessage;

use crate::time::Duration;
use crate::{
    components::*,
    gateway::pool::PoolGateway,
    hooks::{
        on_transaction_done, use_gateway, use_member, use_member_record, use_member_record_balance,
        use_miner, use_miner_claim_transaction, use_miner_cores, use_miner_is_active,
        use_miner_status, use_pool_register_transaction, use_pool_url, use_system_cpu_utilization,
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
                class: "w-full max-w-2xl mx-auto px-5 sm:px-8 gap-8",
                Heading {
                    class: "w-full",
                    title: "Mine",
                    subtitle: "Convert energy into cryptocurrency."
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

    // Refresh member account
    on_transaction_done(move |_sig| {
        member.restart();
        member_record.restart();
    });

    rsx! {
        Col {
            class: "w-full flex-wrap mx-auto justify-between gap-12",
            Alert {}
            MinerStatus {}
            Col {
                class: "w-full gap-4",
                MinerCores {}
                MinePower {}
            }
            TimeRemaining {}
            MinerPendingRewards {}
            MinerRewards {}
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

fn TimeRemaining() -> Element {
    let (out_msg, _in_msg) = use_miner();
    let miner_status = use_miner_status();

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

    let mut info_hidden = use_signal(|| true);

    rsx! {
        if display_time_remaining.cloned() {
            Col {
                gap: 4,
                // Row {
                //     class: "text-elements-lowEmphasis font-medium justify-between",
                //     span {
                //         "Time"
                //     }
                // }
                button {
                    class: "flex flex-col gap-0 group",
                    onclick: move |_| info_hidden.set(!info_hidden.cloned()),
                    Row {
                        class: "w-full justify-between",
                        Row {
                            gap: 2,
                            span {
                                class: "text-elements-lowEmphasis font-medium",
                                "Time"
                            }
                            InfoIcon {
                                class: "h-4 w-4 shrink-0 text-elements-lowEmphasis group-hover:text-elements-highEmphasis transition-all duration-300 ease-in-out my-auto",
                            }
                        }
                        span {
                            class: "mr-2",
                            "{time_remaining.cloned()}s"
                        }
                    }
                    InfoText {
                        class: "text-wrap text-left text-sm max-w-lg mr-auto",
                        text: "Seconds remaining until the mining challenge must be refreshed.",
                        hidden: info_hidden,
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

fn MinerStatus() -> Element {
    let (_out_msg, _in_msg) = use_miner();
    let miner_status = use_miner_status();
    let status = use_memo(move || match miner_status.cloned() {
        MinerStatus::Registering => "Registering",
        MinerStatus::FetchingChallenge => "Fetching",
        MinerStatus::Hashing => "Hashing",
        MinerStatus::SubmittingSolution => "Submitting",
        MinerStatus::Stopped => "Stopped",
    });

    let mut info_hidden = use_signal(|| true);

    rsx! {
        Col {
            gap: 4,
            button {
                class: "flex flex-col gap-0 group",
                onclick: move |_| info_hidden.set(!info_hidden.cloned()),
                Row {
                    gap: 2,
                    span {
                        class: "text-elements-lowEmphasis font-medium",
                        "Status"
                    }
                    InfoIcon {
                        class: "h-4 w-4 shrink-0 text-elements-lowEmphasis group-hover:text-elements-highEmphasis transition-all duration-300 ease-in-out my-auto",
                    }
                }
                InfoText {
                    class: "text-wrap text-left text-sm max-w-lg mr-auto",
                    text: "The status of your miner.",
                    hidden: info_hidden,
                }
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
    let mut info_hidden = use_signal(|| true);
    rsx! {
        Col {
            gap: 4,
            button {
                class: "flex flex-col gap-0 group",
                onclick: move |_| info_hidden.set(!info_hidden.cloned()),
                Row {
                    gap: 2,
                    span {
                        class: "text-elements-lowEmphasis font-medium",
                        "Cores"
                    }
                    InfoIcon {
                        class: "h-4 w-4 shrink-0 text-elements-lowEmphasis group-hover:text-elements-highEmphasis transition-all duration-300 ease-in-out my-auto",
                    }
                }
                InfoText {
                    class: "text-wrap text-left text-sm max-w-lg mr-auto",
                    text: "The number of CPU cores to dedicate to mining. Use more cores to increase your hashpower. The graph below visualizes the current utilization of each CPU core on your machine.",
                    hidden: info_hidden,
                }
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
                        class: "flex items-center justify-center w-12 h-12 controls-secondary rounded-full text-3xl hover:disabled:cursor-not-allowed",
                        disabled: cfg!(feature = "web"),
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
    let mut info_hidden = use_signal(|| true);
    rsx! {
        if let Some(Ok(member_record_balance)) = member_record_balance.cloned() {
            if let Some(Ok(member)) = member.cloned() {
                if member_record_balance > member.total_balance {
                    Col {
                        gap: 4,
                        // span {
                        //     class: "text-elements-lowEmphasis font-medium",
                        //     "Rewards (pending)"
                        // }
                        button {
                            class: "flex flex-col gap-0 group",
                            onclick: move |_| info_hidden.set(!info_hidden.cloned()),
                            Row {
                                gap: 2,
                                span {
                                    class: "text-elements-lowEmphasis font-medium",
                                    "Rewards (pending)"
                                }
                                InfoIcon {
                                    class: "h-4 w-4 shrink-0 text-elements-lowEmphasis group-hover:text-elements-highEmphasis transition-all duration-300 ease-in-out my-auto",
                                }
                            }
                            InfoText {
                                class: "text-wrap text-left text-sm max-w-lg mr-auto",
                                text: "ORE that you have mined, but cannot yet claim. Pending rewards are automatically committed to your claimable balance by the mining pool operator every few hours.",
                                hidden: info_hidden,
                            }
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
    let claim_tx = use_miner_claim_transaction(member);
    let mut info_hidden = use_signal(|| true);
    rsx! {
        Col {
            gap: 4,
            // span {
            //     class: "text-elements-lowEmphasis font-medium",
            //     "Rewards"
            // }
            button {
                class: "flex flex-col gap-0 group",
                onclick: move |_| info_hidden.set(!info_hidden.cloned()),
                Row {
                    gap: 2,
                    span {
                        class: "text-elements-lowEmphasis font-medium",
                        "Rewards"
                    }
                    InfoIcon {
                        class: "h-4 w-4 shrink-0 text-elements-lowEmphasis group-hover:text-elements-highEmphasis transition-all duration-300 ease-in-out my-auto",
                    }
                }
                InfoText {
                    class: "text-wrap text-left text-sm max-w-lg mr-auto",
                    text: "ORE that you have mined and may claim.",
                    hidden: info_hidden,
                }
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
            ClaimButton {
                transaction: claim_tx,
                tx_type: TransactionType::PoolClaim,
            }
        }
    }
}

fn MinePower() -> Element {
    // Get the number of cores
    let max = crate::cores::get();

    // Get CPU utilization
    let cpu_utilization = use_system_cpu_utilization();

    // Normalize rates for CPU utilization values
    let normalized_cpu_utilization = use_memo(move || {
        // Create a vector of utilization rates for each core
        let mut rates: Vec<usize> = vec![0; max];
        let utilization_vec = cpu_utilization.read();
        if !utilization_vec.is_empty() {
            for i in 0..max {
                if i < utilization_vec.len() {
                    rates[i] = utilization_vec[i].clamp(0.0, 100.0).round() as usize;
                }
            }
        }
        rates
    });

    // Split cores evenly between 2 columns
    let cores_per_column = (max + 1) / 2;

    // Create arrays of indices for each column (just sequential numbers for display order)
    let column_indices: Vec<Vec<usize>> = (0..2)
        .map(|col| {
            let start = col * cores_per_column;
            let end = if col == 0 {
                // First column takes half (rounded up)
                cores_per_column
            } else {
                // Second column takes remaining cores
                max
            };
            (start..end).collect()
        })
        .collect();

    rsx! {
        Col {
            class: "relative flex w-full mx-auto pr-2",
            gap: 4,
            // Keyframes for the animation
            // style {
            //     "@keyframes blockPulse {{
            //         0% {{ opacity: 0; }}
            //         100% {{ opacity: 1; }}
            //     }}"
            // }

            // span {
            //     class: "text-elements-lowEmphasis font-medium",
            //     "Core utilization"
            // }

            // Manual column layout with increased spacing between columns
            div {
                class: "flex flex-col md:flex-row gap-8 w-full",
                // Create a column for each group of cores
                for column in column_indices {
                    div {
                        class: "flex-1",
                        // Create system bars for each core in numerical order
                        for core_idx in column {
                            {
                                // Get the utilization rate for this core
                                let rate = normalized_cpu_utilization.read()[core_idx];

                                rsx! {
                                    div {
                                        // Core index
                                        class: "flex items-center gap-1 w-full flex-shrink-0 mb-2",
                                        span {
                                            class: "text-elements-midEmphasis w-6 text-left text-sm flex-shrink-0 font-medium",
                                            "{core_idx}"
                                        }
                                        // Core usage bar
                                        div {
                                            class: "flex-1 mx-1 h-6 overflow-hidden",
                                            // Container for all 10 blocks as a non-flex div
                                            div {
                                                class: "flex h-full w-full",
                                                for j in 0..10 {
                                                    {
                                                        // Fixed block class assignment
                                                        let color = if j < 7 {
                                                            "h-full bg-elements-green"
                                                        } else if j < 9 {
                                                            "h-full bg-elements-yellow"
                                                        } else {
                                                            "h-full bg-elements-red"
                                                        };

                                                        let opacity = if j < (rate + 5) / 10 {
                                                            "opacity-100"
                                                        } else {
                                                            "opacity-0"
                                                        };

                                                        rsx! {
                                                            div {
                                                                class: "transition-all duration-300 ease-in-out {opacity} {color}",
                                                                style: "width: 9%; margin-right: 1%;"
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }

                                        // Usage percentage for each core
                                        span {
                                            class: "text-elements-lowEmphasis text-xs font-medium w-8 text-right flex-shrink-0",
                                            "{rate}%"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
