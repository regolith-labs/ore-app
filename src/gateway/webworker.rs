use std::rc::Rc;

use ore::EPOCH_DURATION;
#[cfg(feature = "web")]
use solana_client_wasm::solana_sdk::{signature::Signature, signer::Signer};
#[cfg(feature = "desktop")]
use solana_sdk::{signature::Signature, signer::Signer};

use super::{signer, Gateway, GatewayResult};

use crate::hooks::MiningResult;

pub async fn submit_solution(
    gateway: &Rc<Gateway>,
    res: &MiningResult,
) -> GatewayResult<Signature> {
    // Submit mine tx.
    let mut bus_id = 0;
    let next_hash = res.hash;
    let nonce = res.nonce;
    let signer = signer();
    loop {
        // Check if epoch needs to be reset
        let treasury = gateway.get_treasury().await?;
        let clock = gateway.get_clock().await?;
        let epoch_end_at = treasury.epoch_start_at.saturating_add(EPOCH_DURATION);

        // Submit restart epoch tx, if needed.
        if clock.unix_timestamp.ge(&epoch_end_at) {
            let ix = ore::instruction::reset(signer.pubkey());
            gateway.send_and_confirm(&[ix]).await?;
        }

        // Submit mine tx
        let ix = ore::instruction::mine(
            signer.pubkey(),
            ore::BUS_ADDRESSES[bus_id],
            next_hash.into(),
            nonce,
        );
        match gateway.send_and_confirm(&[ix]).await {
            Ok(sig) => return Ok(sig),
            Err(_err) => {
                // Retry on different bus.
                bus_id += 1;
                if bus_id.ge(&ore::BUS_COUNT) {
                    bus_id = 0;
                }
            }
        }
    }
}
