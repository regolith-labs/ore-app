use ore::EPOCH_DURATION;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use solana_client_wasm::solana_sdk::{
    keccak::{hashv, Hash as KeccakHash},
    pubkey::Pubkey,
    signature::Signature,
    signer::Signer,
};
use web_sys::Worker;

use crate::gateway::{
    get_clock_account, get_keypair, get_proof, get_treasury, send_and_confirm, wasm_client,
};

pub async fn mine(worker: Worker) {
    let keypair = get_keypair();
    let treasury = get_treasury().await;
    let proof = get_proof(keypair.pubkey()).await;
    let req = MineRequest {
        hash: proof.hash.into(),
        difficulty: treasury.difficulty.into(),
        pubkey: keypair.pubkey(),
    };
    let msg = to_value(&req).unwrap();
    worker.post_message(&msg).unwrap();
}

pub async fn submit_solution(res: &MineResponse) -> Option<Signature> {
    // Submit mine tx.
    let mut bus_id = 0;
    let next_hash = res.hash;
    let nonce = res.nonce;
    let keypair = get_keypair();
    let client = wasm_client();
    loop {
        // Check if epoch needs to be reset
        let treasury = get_treasury().await;
        let clock = get_clock_account().await;
        let epoch_end_at = treasury.epoch_start_at.saturating_add(EPOCH_DURATION);

        // Submit restart epoch tx, if needed.
        if clock.unix_timestamp.ge(&epoch_end_at) {
            let ix = ore::instruction::reset(keypair.pubkey());
            send_and_confirm(&client, &[ix]).await;
        }

        // Submit mine tx
        let ix = ore::instruction::mine(
            keypair.pubkey(),
            ore::BUS_ADDRESSES[bus_id],
            next_hash.into(),
            nonce,
        );
        match send_and_confirm(&client, &[ix]).await {
            Some(sig) => return Some(sig),
            None => {
                // Retry on different bus.
                bus_id += 1;
                if bus_id.ge(&ore::BUS_COUNT) {
                    bus_id = 0;
                }
            }
        }
    }
}

/// Mining request for web workers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MineRequest {
    pub hash: KeccakHash,
    pub difficulty: KeccakHash,
    pub pubkey: Pubkey,
}

/// Mining response from web workers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MineResponse {
    pub hash: KeccakHash,
    pub nonce: u64,
}

/// Finds the a valid hash given the mining request.
pub fn find_next_hash(req: MineRequest) -> MineResponse {
    let mut next_hash: KeccakHash;
    let mut nonce = 0u64;
    loop {
        if nonce % 10_000 == 0 {
            log::info!("Nonce: {}", nonce);
        }
        next_hash = hashv(&[
            req.hash.to_bytes().as_slice(),
            req.pubkey.to_bytes().as_slice(),
            nonce.to_be_bytes().as_slice(),
        ]);
        if next_hash.le(&req.difficulty) {
            break;
        }
        nonce += 1;
    }
    MineResponse {
        hash: next_hash,
        nonce,
    }
}
