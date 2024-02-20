use solana_client_wasm::solana_sdk::{signature::Signature, signer::Signer};

use super::{get_keypair, ore_token_account_address, send_and_confirm, wasm_client};

pub async fn claim(amount: u64) -> Option<Signature> {
    let client = wasm_client();
    let keypair = get_keypair();
    let beneficiary = ore_token_account_address(keypair.pubkey());
    let ix = ore::instruction::claim(keypair.pubkey(), beneficiary, amount);
    send_and_confirm(&client, &[ix]).await
}
