use solana_client_wasm::solana_sdk::{clock::Clock, sysvar};

use super::wasm_client;

pub async fn get_clock_account() -> Clock {
    let client = wasm_client();
    let data = client
        .get_account_data(&sysvar::clock::ID)
        .await
        .expect("Failed to get miner account");
    bincode::deserialize::<Clock>(&data).expect("Failed to deserialize clock")
}
