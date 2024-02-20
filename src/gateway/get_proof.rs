use ore::{state::Proof, utils::AccountDeserialize};
use solana_client_wasm::solana_sdk::pubkey::Pubkey;

use super::{proof_pubkey, wasm_client};

pub async fn get_proof(authority: Pubkey) -> Proof {
    let client = wasm_client();
    let proof_address = proof_pubkey(authority);
    let data = client
        .get_account_data(&proof_address)
        .await
        .expect("Failed to get miner account");
    *Proof::try_from_bytes(&data).expect("Failed to parse miner account")
}
