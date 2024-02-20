use solana_client_wasm::solana_sdk::signer::Signer;

use crate::gateway::get_keypair;

use super::{proof_pubkey, send_and_confirm, wasm_client};

pub async fn initialize_proof_account() {
    // Return early, if account is already initialized
    let keypair = get_keypair();
    let proof_address = proof_pubkey(keypair.pubkey());
    let client = wasm_client();
    if client.get_account(&proof_address).await.is_ok() {
        return;
    }

    // Sign and send transaction.
    let ix = ore::instruction::register(keypair.pubkey());
    send_and_confirm(&client, &[ix]).await;
}
