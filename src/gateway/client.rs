use solana_client_wasm::{
    solana_sdk::{
        instruction::Instruction, signature::Signature, signer::Signer, transaction::Transaction,
    },
    WasmClient,
};

use super::get_keypair;

// const URL: &str = "http://54.86.50.170:8899";
// const URL: &str = "https://api.devnet.solana.com";
const URL: &str = "https://devnet.helius-rpc.com/?api-key=bb9df66a-8cba-404d-b17a-e739fe6a480c";

pub fn wasm_client() -> WasmClient {
    WasmClient::new(URL)
}

// TODO Result type
pub async fn send_and_confirm(client: &WasmClient, ixs: &[Instruction]) -> Option<Signature> {
    let keypair = get_keypair();
    let mut transaction = Transaction::new_with_payer(ixs, Some(&keypair.pubkey()));
    let recent_blockhash = client.get_latest_blockhash().await.unwrap();
    transaction.sign(&[&keypair], recent_blockhash);
    let result = client.send_and_confirm_transaction(&transaction).await;
    match result {
        Ok(sig) => {
            log::info!("Transaction success: {:?}", sig);
            Some(sig)
        }
        Err(err) => {
            log::error!("Transaction failed: {:?}", err);
            None
        }
    }
}
