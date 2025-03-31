use ore_api::state::Proof;
use ore_boost_api::state::{Boost, Stake};
use ore_types::request::TransactionEvent;
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use std::str::FromStr;
use steel::AccountDeserialize;

use super::{Gateway, GatewayError, GatewayResult, Rpc};

const ORE_API_URL: &str = "https://api.ore.supply";

pub trait OreGateway {
    // Accounts
    async fn get_boost(&self, address: Pubkey) -> GatewayResult<Boost>;
    async fn get_stake(&self, address: Pubkey) -> GatewayResult<Stake>;
    async fn get_proof(&self, address: Pubkey) -> GatewayResult<Proof>;

    // API
    async fn get_boost_yield_7d(&self, boost_address: Pubkey) -> GatewayResult<f64>;
    async fn get_ore_holders(&self) -> GatewayResult<u64>;
    async fn log_transaction_event(
        &self,
        transaction: TransactionEvent,
    ) -> GatewayResult<Signature>;
    async fn get_twitter_request_token(&self) -> GatewayResult<String>;
}

impl<R: Rpc> OreGateway for Gateway<R> {
    async fn get_boost(&self, address: Pubkey) -> GatewayResult<Boost> {
        let data = self
            .rpc
            .get_account_data(&address)
            .await
            .map_err(GatewayError::from)?;
        Ok(*Boost::try_from_bytes(&data)?)
    }

    async fn get_stake(&self, address: Pubkey) -> GatewayResult<Stake> {
        let data = self
            .rpc
            .get_account_data(&address)
            .await
            .map_err(GatewayError::from)?;
        Ok(*Stake::try_from_bytes(&data)?)
    }

    async fn get_proof(&self, address: Pubkey) -> GatewayResult<Proof> {
        let data = self
            .rpc
            .get_account_data(&address)
            .await
            .map_err(GatewayError::from)?;
        Ok(*Proof::try_from_bytes(&data)?)
    }

    async fn get_boost_yield_7d(&self, boost_address: Pubkey) -> GatewayResult<f64> {
        let get_url = format!("{}/boosts/{}/yield", ORE_API_URL, boost_address);
        let resp = self
            .http
            .get(get_url)
            .send()
            .await
            .map_err(GatewayError::from)?;
        let yield_7d = resp.json::<f64>().await.map_err(GatewayError::from)?;
        Ok(yield_7d)
    }

    async fn get_ore_holders(&self) -> GatewayResult<u64> {
        let get_url = format!("{}/holders", ORE_API_URL);
        let resp = self
            .http
            .get(get_url)
            .send()
            .await
            .map_err(GatewayError::from)?;
        let holders = resp.json::<u64>().await.map_err(GatewayError::from)?;
        Ok(holders)
    }

    async fn log_transaction_event(
        &self,
        transaction: TransactionEvent,
    ) -> GatewayResult<Signature> {
        let url = format!("{}/events/transaction", ORE_API_URL);
        let resp = self
            .http
            .post(url)
            .json(&transaction)
            .send()
            .await
            .map_err(GatewayError::from)?;
        let body = resp.text().await.map_err(GatewayError::from)?;
        let sig = Signature::from_str(&body).map_err(|_| GatewayError::RequestFailed)?;
        Ok(sig)
    }

    async fn get_twitter_request_token(&self) -> GatewayResult<String> {
        let get_url = format!("{}/auth/twitter/token", ORE_API_URL);
        let resp = self
            .http
            .get(get_url)
            .send()
            .await
            .map_err(GatewayError::from)?;
        let body = resp.text().await.map_err(GatewayError::from)?;
        let token =
            serde_json::from_str::<RequestTokenResponse>(&body).map_err(GatewayError::from)?;
        Ok(token.oauth_token)
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct RequestTokenResponse {
    oauth_callback_confirmed: String,
    oauth_token: String,
    oauth_token_secret: String,
}
