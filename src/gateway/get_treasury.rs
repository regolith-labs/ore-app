use ore::{state::Treasury, utils::AccountDeserialize, TREASURY_ADDRESS};

use super::wasm_client;

pub async fn get_treasury() -> Treasury {
    let client = wasm_client();
    let data = client
        .get_account_data(&TREASURY_ADDRESS)
        .await
        .expect("Failed to get treasury account");
    *Treasury::try_from_bytes(&data).expect("Failed to parse treasury account")
}
