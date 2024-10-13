use ore_pool_types::{Member, RegisterPayload, RegisterStakerPayload, Staker};
use solana_client_wasm::solana_sdk::pubkey;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;

use crate::gateway::GatewayError;

pub const POOL_ADDRESS: Pubkey = pubkey!("9RrEyMNFhFcrqVikWby5rVn1eXeKHr2SwGRbPhZ7wDCK");
const POOL_URL: &str = "https://official.ec1ipse.me";

pub struct Pool {
    http_client: reqwest::Client,
}

impl Pool {
    pub fn new(http_client: reqwest::Client) -> Self {
        Self { http_client }
    }
    pub async fn post_register(&self, member_authority: Pubkey) -> Result<Member, GatewayError> {
        let post_url = format!("{}/register", POOL_URL);
        let body = RegisterPayload {
            authority: member_authority,
        };
        self.http_client
            .post(post_url)
            .json(&body)
            .send()
            .await?
            .json::<Member>()
            .await
            .map_err(From::from)
    }
    pub async fn post_register_staker(
        &self,
        member_authority: Pubkey,
        mint: Pubkey,
    ) -> Result<Staker, GatewayError> {
        let post_url = format!("{}/register-staker", POOL_URL);
        let body = RegisterStakerPayload {
            authority: member_authority,
            mint,
        };
        self.http_client
            .post(post_url)
            .json(&body)
            .send()
            .await?
            .json::<Staker>()
            .await
            .map_err(From::from)
    }
}
