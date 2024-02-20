use ore_types::{response::GetTransfersResponse, Transfer};
use solana_client_wasm::solana_sdk::pubkey::Pubkey;

const URL: &str = "https://ore-api-lthm.onrender.com/transfers";

pub async fn get_transfer(sig: String) -> Option<Transfer> {
    let client = reqwest::Client::new();
    match client.get(format!("{}/{}", URL, sig)).send().await {
        Ok(res) => res.json::<Transfer>().await.ok(),
        Err(e) => {
            log::error!("{:?}", e);
            None
        }
    }
}

pub async fn get_transfers(
    user: Option<Pubkey>,
    offset: u64,
    limit: usize,
) -> Option<GetTransfersResponse> {
    let offset = offset.to_string();
    let limit = limit.to_string();
    let mut query = vec![("offset", offset.as_str()), ("limit", limit.as_str())];
    let user_str = user.map(|u| u.to_string());
    let user_ref = user_str.as_deref();
    if let Some(user_str) = user_ref {
        query.push(("user", user_str));
    };
    let client = reqwest::Client::new();
    match client.get(URL).query(&query).send().await {
        Ok(res) => res.json::<GetTransfersResponse>().await.ok(),
        Err(e) => {
            log::error!("{:?}", e);
            None
        }
    }
}
