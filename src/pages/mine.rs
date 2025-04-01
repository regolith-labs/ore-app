use dioxus::prelude::*;
use ore_api::consts::TOKEN_DECIMALS;
use ore_miner_types::OutputMessage;
use solana_sdk::transaction::Transaction;

use crate::{
    components::*,
    gateway::{pool::PoolGateway, GatewayError, GatewayResult},
    hooks::{
        build_commit_claim_instructions, on_transaction_done, use_gateway, use_help_drawer_state,
        use_member, use_member_record, use_member_record_balance, use_miner, use_miner_cores,
        use_miner_is_active, use_miner_status, use_pool, use_pool_register_transaction,
        use_pool_url, use_system_cpu_utilization, use_transaction_status, use_wallet,
        HelpDrawerPage, MinerStatus, PoolRegisterStatus, Wallet,
    },
    solana::spl_token::amount_to_ui_amount_string,
};
use ore_types::request::TransactionType;

pub fn Mine() -> Element {
    rsx! {
        Col {
            class: "w-full h-full pb-20 sm:pb-16",
            gap: 16,
            Col {
                class: "w-full max-w-2xl mx-auto px-5 sm:px-8 gap-8",
                Row {
                    class: "w-full justify-between",
                    Heading {
                        class: "w-full",
                        title: "Mine",
                        subtitle: "Convert energy into cryptocurrency."
                    }
                    MineHelpButton {}
                }
                MinerData {}
            }
            MineTable {}
        }
    }
}

fn MinerData() -> Element {
    // Get resources
    let mut member_record = use_member_record();

    // Refresh member account
    on_transaction_done(move |_sig| {
        member_record.restart();
    });

    rsx! {
        Col { class: "w-full flex-wrap mx-auto justify-between gap-12",
            Alert {}
            MinerStatus {}
            Col { class: "w-full gap-8",
                MinerCores {}
                MinePower {}
                DownloadCTA {}
            }
            TimeRemaining {}
            MinerRewards {}
        }
    }
}

fn StopStartButton() -> Element {
    let wallet = use_wallet();
    let pool_url = use_pool_url();
    let mut miner_status = use_miner_status();
    let member = use_member();
    let mut member_record = use_member_record();
    let mut register_tx_start = use_signal(|| false);
    let register_tx = use_pool_register_transaction(register_tx_start);
    let is_active = use_miner_is_active();

    // listen for onchain pool registration
    use_effect(move || match register_tx.cloned() {
        // commit claim dependency failed, reset
        Some(Ok(PoolRegisterStatus::CommitClaimFailed)) => {
            miner_status.set(MinerStatus::Stopped);
        }
        // submit tx
        Some(Ok(PoolRegisterStatus::Transaction(tx))) => {
            submit_transaction(tx, TransactionType::PoolJoin);
        }
        _ => {}
    });

    // offchain pool server registration
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
        match use_gateway().register(authority, pool_url).await {
            Ok(_member_record) => {
                member_record.restart();
                miner_status.set(MinerStatus::FetchingChallenge);
            }
            Err(err) => {
                log::error!("Error registering with server: {:?}", err);
            }
        }
    });

    // on successful registration signature (onchain),
    // restart the pool server registration
    //
    // this is the happy path,
    // first the onchain registration signature lands
    // and then we submit for registration with the offchain pool server
    on_transaction_done(move |sig| {
        if miner_status.cloned() == MinerStatus::Registering {
            log::info!("registration sig: {:?}", sig);
            register_with_pool_server.restart();
        }
    });
    // submit pool server registration
    //
    // this is the recovery path,
    // where the onchain registration landed
    // but the server registration failed or hasn't been submitted yet
    use_effect(move || {
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
                if *is_active.read() {
                    miner_status.set(MinerStatus::Stopped);
                } else {
                    if let Ok(_member) = *member.read() {
                        if let Some(Ok(_member_db)) = member_record.cloned() {
                            miner_status.set(MinerStatus::FetchingChallenge);
                        } else {
                            miner_status.set(MinerStatus::Registering);
                        }
                    } else {
                        register_tx_start.set(true);
                        miner_status.set(MinerStatus::Registering);
                    }
                }
            },
            if !*is_active.read() {
                PlayIcon { class: "my-auto h-5" }
                span { class: "my-auto", "Start" }
            } else {
                StopIcon { class: "my-auto h-5" }
                span { class: "my-auto", "Stop" }
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
            Col { gap: 4,
                button {
                    class: "flex flex-col gap-0 group",
                    onclick: move |_| info_hidden.set(!info_hidden.cloned()),
                    Row { class: "w-full justify-between",
                        Row { gap: 2,
                            span { class: "text-elements-lowEmphasis font-medium", "Time" }
                            InfoIcon { class: "h-4 w-4 shrink-0 text-elements-lowEmphasis group-hover:text-elements-highEmphasis transition-all duration-300 ease-in-out my-auto" }
                        }
                        span { class: "mr-2", "{time_remaining.cloned()}s" }
                    }
                    InfoText {
                        class: "text-wrap text-left text-sm max-w-lg mr-auto",
                        text: "Seconds remaining until the mining challenge must be refreshed.",
                        hidden: info_hidden,
                    }
                }
                div { class: "w-full h-2 bg-elements-lowEmphasis rounded-full",
                    div {
                        class: "h-full bg-elements-highEmphasis rounded-full transition-all",
                        style: "width: {(100.0 - (time_remaining.cloned() as f32 / 60.0 * 100.0)).max(0.0)}%",
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

    let description = use_memo(move || match miner_status.cloned() {
        MinerStatus::Registering => "Curerntly registering with the pool server.",
        MinerStatus::FetchingChallenge => {
            "Currently fetching the next challenge from the pool server."
        }
        MinerStatus::Hashing => "Currently searching for valid solutions.",
        MinerStatus::SubmittingSolution => "Currently submitting the solution to the pool server.",
        MinerStatus::Stopped => "Currently not active.",
    });

    let mut info_hidden = use_signal(|| true);

    rsx! {
        Col { gap: 4,
            button {
                class: "flex flex-col gap-0 group",
                onclick: move |_| info_hidden.set(!info_hidden.cloned()),
                Row { gap: 2,
                    span { class: "text-elements-lowEmphasis font-medium", "Status" }
                    InfoIcon { class: "h-4 w-4 shrink-0 text-elements-lowEmphasis group-hover:text-elements-highEmphasis transition-all duration-300 ease-in-out my-auto" }
                }
                InfoText {
                    class: "text-wrap text-left text-sm max-w-lg mr-auto",
                    text: "The status of your miner. {description}",
                    hidden: info_hidden,
                }
            }
            Row { class: "justify-between",
                span { class: "font-semibold text-2xl sm:text-3xl", "{status}" }
                StopStartButton {}
            }
        }
    }
}

fn _MinerHashpower() -> Element {
    rsx! {
        Col { gap: 4,
            span { class: "text-elements-lowEmphasis font-medium", "Hashpower" }
            span { class: "font-semibold text-2xl sm:text-3xl", "1230 H/s" }
        }
    }
}

fn MinerCores() -> Element {
    let mut cores = use_miner_cores();
    let max = crate::cores::get();
    let mut info_hidden = use_signal(|| true);
    rsx! {
        Col { gap: 4,
            button {
                class: "flex flex-col gap-0 group",
                onclick: move |_| info_hidden.set(!info_hidden.cloned()),
                Row { gap: 2,
                    span { class: "text-elements-lowEmphasis font-medium", "Cores" }
                    InfoIcon { class: "h-4 w-4 shrink-0 text-elements-lowEmphasis group-hover:text-elements-highEmphasis transition-all duration-300 ease-in-out my-auto" }
                }
                InfoText {
                    class: "text-wrap text-left text-sm max-w-lg mr-auto",
                    text: "The number of CPU cores to dedicate to mining. Use more cores to increase your hashpower. The graph below visualizes the current utilization of each CPU core on your machine.",
                    hidden: info_hidden,
                }
            }
            Row { class: "justify-between",
                span { class: "font-semibold text-2xl sm:text-3xl", "{cores}" }
                Row { gap: 2,
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

#[derive(Eq, PartialEq, Clone)]
pub enum MemberBalance {
    Loading,
    Null,
    Balance(u64),
}

fn MinerRewards() -> Element {
    let member = use_member();
    let member_db_balance = use_member_record_balance();
    let pool = use_pool();
    // init claimable balance
    let mut member_claimable_balance = use_signal(|| MemberBalance::Loading);
    use_effect(move || {
        let balance = match member_db_balance.cloned() {
            Some(member_db_balance) => {
                if let (Ok(member_db_balance), Ok(member)) = (member_db_balance, member.cloned()) {
                    // claimable balance
                    let diff = member_db_balance.saturating_sub(member.total_balance);
                    MemberBalance::Balance(member.balance + diff)
                } else {
                    MemberBalance::Null
                }
            }
            _ => MemberBalance::Loading,
        };
        member_claimable_balance.set(balance);
    });
    // init claim transaction
    let mut claim_tx = use_signal(|| Err(GatewayError::RequestFailed));
    use_effect(move || {
        if let (Some(Ok(member_db_balance)), Ok(member), Some(pool)) =
            (member_db_balance.cloned(), member.cloned(), pool.cloned())
        {
            spawn(async move {
                let gateway = use_gateway();
                if let Ok(ixs) =
                    build_commit_claim_instructions(&gateway.rpc, &pool, &member, member_db_balance)
                        .await
                {
                    let tx = Transaction::new_with_payer(&ixs, Some(&member.authority));
                    claim_tx.set(Ok(tx));
                }
            });
        }
    });
    // hidden toggle
    let mut info_hidden = use_signal(|| true);
    rsx! {
        Col { gap: 4,
            button {
                class: "flex flex-col gap-0 group",
                onclick: move |_| info_hidden.set(!info_hidden.cloned()),
                Row { gap: 2,
                    span { class: "text-elements-lowEmphasis font-medium", "Rewards" }
                    InfoIcon { class: "h-4 w-4 shrink-0 text-elements-lowEmphasis group-hover:text-elements-highEmphasis transition-all duration-300 ease-in-out my-auto" }
                }
                InfoText {
                    class: "text-wrap text-left text-sm max-w-lg mr-auto",
                    text: "ORE that you have mined and may claim.",
                    hidden: info_hidden,
                }
            }
            match *member_claimable_balance.read() {
                MemberBalance::Loading => {
                    rsx! {
                        LoadingValue {}
                    }
                }
                MemberBalance::Null => {
                    rsx! {
                        NullValue {}
                    }
                }
                MemberBalance::Balance(u64) => {
                    rsx! {
                        OreValue {
                            size: TokenValueSize::Large,
                            ui_amount_string: amount_to_ui_amount_string(u64, TOKEN_DECIMALS),
                            with_decimal_units: true,
                            gold: true,
                        }
                    }
                }
            }
            MinerRewardsClaimButton { transaction: claim_tx, tx_type: TransactionType::PoolClaim }
        }
    }
}

#[component]
pub fn MinerRewardsClaimButton(
    transaction: Signal<GatewayResult<Transaction>>,
    tx_type: TransactionType,
) -> Element {
    let pool_url = use_pool_url();
    let member = use_member();
    let mut transaction_status = use_transaction_status();

    let enabled = if let Ok(_) = transaction.read().as_ref() {
        true
    } else {
        false
    };

    rsx! {
        button {
            class: "h-12 w-full rounded-full controls-gold",
            disabled: !enabled,
            onclick: move |_| {
                spawn(async move {
                    if let (Some(pool_url), Ok(member), Ok(tx)) = (
                        pool_url.cloned(),
                        member.cloned(),
                        transaction.cloned(),
                    ) {
                        transaction_status.set(Some(TransactionStatus::Waiting));
                        let sign_partial = sign_transaction_partial(tx).await;
                        match sign_partial {
                            Ok((tx, hash)) => {
                                transaction_status.set(Some(TransactionStatus::Sending(0)));
                                let gateway = use_gateway();
                                match gateway
                                    .commit_claim(member.authority, pool_url, tx, hash)
                                    .await
                                {
                                    Ok(balance_update) => {
                                        transaction_status
                                            .set(
                                                Some(TransactionStatus::Done(balance_update.signature)),
                                            );
                                    }
                                    Err(err) => {
                                        transaction_status.set(Some(TransactionStatus::Error));
                                        log::error!("{:?}", err);
                                    }
                                }
                            }
                            Err(err) => {
                                transaction_status.set(Some(TransactionStatus::Error));
                                log::error!("{:?}", err);
                            }
                        }
                    }
                });
            },
            span { class: "mx-auto my-auto font-semibold", "Claim" }
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
        Col { class: "relative flex w-full mx-auto pr-2", gap: 4,

            // Manual column layout with increased spacing between columns
            div { class: "flex flex-col md:flex-row gap-8 w-full",
                // Create a column for each group of cores
                for column in column_indices {
                    div { class: "flex-1",
                        // Create system bars for each core in numerical order
                        for core_idx in column {
                            {
                                let rate = normalized_cpu_utilization.read()[core_idx];
                                rsx! {
                                    div {
                                        // Core index
                                        class: "flex items-center gap-1 w-full flex-shrink-0 mb-2",
                                        span { class: "text-elements-midEmphasis w-6 text-left text-sm flex-shrink-0 font-medium",
                                            "{core_idx + 1}"
                                        }
                                        // Core usage bar
                                        div { class: "flex-1 mx-1 h-6 overflow-hidden",
                                            // Container for all 10 blocks as a non-flex div
                                            div { class: "flex h-full w-full",
                                                for j in 0..10 {
                                                    {
                                                        let color = if j < 7 {
                                                            "h-full bg-elements-green"
                                                        } else if j < 9 {
                                                            "h-full bg-elements-yellow"
                                                        } else {
                                                            "h-full bg-elements-red"
                                                        };
                                                        let opacity = if j < (rate + 5) / 10 { "opacity-100" } else { "opacity-0" };
                                                        rsx! {
                                                            div {
                                                                class: "transition-all duration-300 ease-in-out {opacity} {color}",
                                                                style: "width: 9%; margin-right: 1%;",
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }

                                        // Usage percentage for each core
                                        span { class: "text-elements-lowEmphasis text-xs font-medium w-8 text-right flex-shrink-0",
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

// Help button specific to the Mine page
fn MineHelpButton() -> Element {
    let mut drawer_state = use_help_drawer_state();

    rsx! {
        button {
            onclick: move |_| {
                let mut current = drawer_state.read().clone();
                current.is_open = true;
                current.current_page = HelpDrawerPage::Mine;
                drawer_state.set(current);
            },
            Row {
                class: "elevated-control elevated-border rounded-full text-sm font-semibold h-12 px-5 hover:cursor-pointer",
                gap: 2,
                span {
                    class: "mx-auto my-auto",
                    "Help"
                }
                BulbIcon {
                    class: "w-4 text-elements-midEmphasis"
                }
            }
        }
    }
}

// Only show the download CTA on web
#[cfg(not(feature = "web"))]
fn DownloadCTA() -> Element {
    rsx! {}
}

#[cfg(feature = "web")]
fn DownloadCTA() -> Element {
    rsx! {
        div {
            class: "w-full mt-4 mb-8",
            div {
                class: "flex items-center justify-between rounded-lg py-4 px-6 border border-elements-gold relative",
                div {
                    class: "flex items-center",
                    DownloadIcon {
                        class: "w-8 h-8 mr-4 text-elements-gold"
                    }
                    div {
                        class: "flex flex-col",
                        span {
                            class: "text-elements-highEmphasis font-medium",
                            "Download the desktop app"
                        }
                        span {
                            class: "text-elements-lowEmphasis text-sm",
                            "Get more power with the native desktop miner."
                        }
                    }
                }
                Link {
                    to: "/download",
                    class: "h-12 px-6 rounded-full controls-gold flex items-center justify-center",
                    span {
                        class: "font-semibold",
                        "Download"
                    }
                }
            }
        }
    }
}
