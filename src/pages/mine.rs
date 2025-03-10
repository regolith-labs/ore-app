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
                class: "w-full max-w-2xl mx-auto px-5 sm:px-8",
                Heading {
                    class: "w-full",
                    title: "Mine",
                    subtitle: "Convert energy into cryptocurrency."
                }
                MinePower {}
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
            class: "w-full flex-wrap mx-auto justify-between gap-10",
            Alert {}
            MinerStatus {}
            // MinerHashpower {}
            if cfg!(not(feature = "web")) {
                MinerCores {}
            }
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
                        "New challenge in"
                    }
                    span {
                        class: "mr-2",
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
    let claim_tx = use_miner_claim_transaction(member);
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
            ClaimButton {
                transaction: claim_tx,
                tx_type: TransactionType::PoolClaim,
            }
        }
    }
}

fn MinePower() -> Element {
    // Handle CPU utilization differently for web and desktop
    let cpu_utilization = if cfg!(feature = "web") {
        // Web implementation with simulated values
        let mut signal = use_signal(|| vec![0.0; 12]).clone();
        let miner_is_active = use_miner_is_active();

        // Set up web cpu usage with simulated values
        use_future(move || {
            async move {
                loop {
                    let mut new_values = vec![0.0; 12];

                    // Miner is active
                    if *miner_is_active.read() {
                        // First core runs at ~70% (with some variation)
                        new_values[0] = 65.0 + (rand::random::<f32>() * 10.0);

                        // 2-3 additional cores hover around 10-30%
                        for i in 1..4 {
                            new_values[i] = 10.0 + (rand::random::<f32>() * 20.0);
                        }
                    } else {
                        // Miner is paused,just have 2-3 cores hovering at 10-30%
                        for i in 0..3 {
                            new_values[i] = 10.0 + (rand::random::<f32>() * 20.0);
                        }
                    }

                    signal.set(new_values);

                    // Sleep to simulate some delay
                    async_std::task::sleep(Duration::from_millis(1000)).await;
                }
            }
        });

        signal
    } else {
        // Get desktop CPU usage
        use_system_cpu_utilization()
    };

    // let cores = use_miner_cores();
    let max = if cfg!(feature = "web") {
        12
    } else {
        crate::cores::get()
    };

    // Get the CPU utilization values
    let use_rates = use_memo(move || {
        // Create a vector of utilization rates for each core
        let mut rates: Vec<usize> = vec![0; max];

        // For both web and desktop, process the utilization values
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

    // Create random animation durations for each core
    let animation_durations = (0..max)
        .map(|i| (((i * 17) % 7) as f32 * 0.1 + 0.3).to_string())
        .collect::<Vec<_>>();

    // Calculate how many cores go in each column
    let cores_per_column = 6;
    let num_columns = (max + cores_per_column - 1) / cores_per_column;

    // Create arrays of indices for each column (just sequential numbers for display order)
    let column_indices: Vec<Vec<usize>> = (0..num_columns)
        .map(|col| {
            let start = col * cores_per_column;
            let end = (start + cores_per_column).min(max);
            (start..end).collect()
        })
        .collect();

    rsx! {
        Col {
            class: "relative flex w-full mx-auto my-8 sm:my-16",
            gap: 4,
            // Keyframes for the animation
            style {
                "@keyframes blockPulse {{
                    0% {{ opacity: 0; }} 
                    100% {{ opacity: 1; }}
                }}"
            }
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
                                // Get the rate for this core
                                let rate = use_rates.read()[core_idx];

                                rsx! {
                                    div {
                                        // Core index
                                        class: "flex items-center gap-1 w-full flex-shrink-0 mb-2",
                                        span {
                                            class: "text-elements-midEmphasis w-6 text-left flex-shrink-0 font-medium",
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
                                                        let block_class = if j < 6 {
                                                            "h-full bg-elements-green"
                                                        } else if j < 8 {
                                                            "h-full bg-elements-yellow"
                                                        } else {
                                                            "h-full bg-elements-red"
                                                        };

                                                        rsx! {
                                                            if j < (rate + 5) / 10 {
                                                                div {
                                                                    class: "{block_class}",
                                                                    style: "width: 9%; margin-right: 1%;"
                                                                }
                                                            } else if j == (rate + 5) / 10 && rate > 0 && rate < 100 {
                                                                // The "walking" block that appears and disappears - only for non-zero rates
                                                                div {
                                                                    class: "{block_class}",
                                                                    style: "width: 9%; margin-right: 1%; animation: blockPulse {0}s infinite alternate-reverse ease-in-out;".replace("{0}", &animation_durations[core_idx])
                                                                }
                                                            } else {
                                                                // Empty blocks
                                                                div {
                                                                    class: "h-full",
                                                                    style: "width: 9%; margin-right: 1%;"
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        // Usage percentage for each core
                                        span {
                                            class: "text-elements-lowEmphasis text-xs w-8 text-right flex-shrink-0",
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
