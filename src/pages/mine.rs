use dioxus::prelude::*;
use ore_api::consts::TOKEN_DECIMALS;
use solana_extra_wasm::program::spl_token::amount_to_ui_amount_string;

use crate::{
    components::*, 
    gateway::pool::PoolGateway,
    hooks::{on_transaction_done, use_gateway, use_member, use_member_record, use_miner_claim_transaction, use_miner_is_active, use_miner_status, use_pool_register_transaction, use_pool_url, use_wallet, MinerStatus, Wallet}, 
};

pub fn Mine() -> Element {    
    rsx! {
        Col {
            class: "w-full h-full pb-20 sm:pb-16 max-w-2xl mx-auto px-5 sm:px-8",
            gap: 4,
            Heading {
                class: "w-full",
                title: "Mine",
                subtitle: "Utilize spare hashpower to harvest ORE."
            }
            OrbMiner {
                class: "relative flex w-[16rem] h-[16rem] mx-auto my-8 sm:my-16",
                gold: *use_miner_is_active().read()
            }
        }
        Col {
            class: "w-full max-w-3xl mx-auto",
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

    let mut f = use_future(move || async move {
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

    on_transaction_done(move |_sig| {
        if miner_status.cloned() == MinerStatus::Registering {
            f.restart();
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
            onclick: move |_| {
                if *is_active.read() {
                    miner_status.set(MinerStatus::Stopped);
                } else {
                    if let Some(Ok(_member)) = member.cloned() {
                        miner_status.set(MinerStatus::FetchingChallenge);
                    } else if let Some(Ok(tx)) = register_tx.cloned() {
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
    let status = use_memo(move || {
        match miner_status.cloned() {
            MinerStatus::Registering => "Registering",
            MinerStatus::FetchingChallenge => "Fetching",
            MinerStatus::Hashing => "Hashing",
            MinerStatus::SubmittingSolution => "Submitting",
            MinerStatus::Stopped => "Stopped",
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
    }
}

fn MinerHashpower() -> Element {
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

fn MinerPendingRewards() -> Element {
    let member = use_member();
    let member_record = use_member_record();
    rsx! {
        if let Some(Ok(member_record)) = member_record.cloned() {
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
                            ui_amount_string: amount_to_ui_amount_string(member_record.total_balance as u64 - member.total_balance, TOKEN_DECIMALS),
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
