use dioxus::prelude::*;
use ore_api::state::Proof;

use crate::{
    gateway::GatewayResult,
    hooks::{use_gateway, MinerStatusMessage, MinerToolbarState, UpdateMinerToolbarState},
    miner::Miner,
};

// TODO Move this somewhere

pub async fn try_start_mining(
    miner: Signal<Miner>,
    proof: Proof,
    toolbar_state: &mut Signal<MinerToolbarState>,
) -> GatewayResult<()> {
    let gateway = use_gateway();

    let clock = gateway.get_clock().await?;
    let cutoff_time = proof
        .last_hash_at
        .saturating_add(60)
        .saturating_sub(clock.unix_timestamp)
        .max(0) as u64;
    toolbar_state.set_status_message(MinerStatusMessage::Searching);
    miner
        .read()
        .start_mining(proof.challenge.into(), 0, cutoff_time)
        .await;

    Ok(())
}
