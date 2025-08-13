use ore_boost_api::state::{Boost, Config as BoostConfig, Stake};
use ore_types::{
    request::{LinkXAccountRequest, TransactionEvent},
    response::{AccessTokenResponse, RequestTokenResponse, WaitlistResponse},
};
use serde::Deserialize;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use std::str::FromStr;
use steel::AccountDeserialize;

use super::{Gateway, GatewayError, GatewayResult, Rpc};

const ORE_API_URL: &str = "https://api.ore.supply";
// const ORE_API_URL: &str = "http://localhost:3000";

// Define a response type for waitlist status
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaitlistStatus {
    pub is_registered: bool,
    pub screen_name: Option<String>,
    pub waitlist_number: Option<i64>,
    pub profile_image_url: Option<String>,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct TopHolder {
    pub address: String,
    pub balance: f64,
}

pub trait OreGateway {
    // Accounts
    async fn get_boost(&self, address: Pubkey) -> GatewayResult<Boost>;
    async fn get_stake(&self, address: Pubkey) -> GatewayResult<Stake>;
    async fn get_boost_config(&self, address: Pubkey) -> GatewayResult<BoostConfig>;

    // API
    async fn get_boost_yield_7d(&self, boost_address: Pubkey) -> GatewayResult<f64>;
    async fn get_ore_holders(&self) -> GatewayResult<u64>;
    async fn get_ore_top_holders(&self) -> GatewayResult<Vec<TopHolder>>;
    async fn log_transaction_event(
        &self,
        transaction: TransactionEvent,
    ) -> GatewayResult<Signature>;
    async fn get_x_request_token(&self) -> GatewayResult<String>;
    async fn get_x_access_token(
        &self,
        oauth_token: String,
        oauth_verifier: String,
    ) -> GatewayResult<AccessTokenResponse>;
    async fn link_x_account(
        &self,
        user_id: String,
        msg: String,
        signature: Signature,
        address: Pubkey,
        access_token: String,
    ) -> GatewayResult<i64>;
    async fn validate_waitlist_status(&self, address: Pubkey) -> GatewayResult<WaitlistStatus>;
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

    async fn get_boost_config(&self, address: Pubkey) -> GatewayResult<BoostConfig> {
        let data = self
            .rpc
            .get_account_data(&address)
            .await
            .map_err(GatewayError::from)?;
        Ok(*BoostConfig::try_from_bytes(&data)?)
    }

    async fn get_stake(&self, address: Pubkey) -> GatewayResult<Stake> {
        let data = self
            .rpc
            .get_account_data(&address)
            .await
            .map_err(GatewayError::from)?;
        Ok(*Stake::try_from_bytes(&data)?)
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

    async fn get_ore_top_holders(&self) -> GatewayResult<Vec<TopHolder>> {
        let get_url = format!("{}/holders/top", ORE_API_URL);
        let resp = self
            .http
            .get(get_url)
            .send()
            .await
            .map_err(GatewayError::from)?;
        let top_holders = resp
            .json::<Vec<TopHolder>>()
            .await
            .map_err(GatewayError::from)?;
        Ok(top_holders)
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

    async fn get_x_request_token(&self) -> GatewayResult<String> {
        let get_url = format!("{}/oauth/x/request_token", ORE_API_URL);
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

    async fn get_x_access_token(
        &self,
        oauth_token: String,
        oauth_verifier: String,
    ) -> GatewayResult<AccessTokenResponse> {
        let url = format!("{}/oauth/x/access_token", ORE_API_URL);
        let resp = self
            .http
            .get(url)
            .query(&[
                ("oauth_token", oauth_token),
                ("oauth_verifier", oauth_verifier),
            ])
            .send()
            .await
            .map_err(GatewayError::from)?;

        // Check for error response
        if !resp.status().is_success() {
            // Special handling for 409 Conflict (x account already exists with solana address)
            if resp.status() == reqwest::StatusCode::CONFLICT {
                // Get response text
                let text = match resp.text().await {
                    Ok(text) => text,
                    Err(e) => {
                        log::error!("Failed to read error response body: {}", e);
                        return Err(GatewayError::RequestFailed);
                    }
                };

                // Attempt to parse the JSON for user details
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
                    // Extract screen_name and solana_address
                    let screen_name = json
                        .get("screen_name")
                        .and_then(|name| name.as_str())
                        .unwrap_or("unknown")
                        .to_string();
                    let solana_address = json
                        .get("solana_address")
                        .and_then(|addr| addr.as_str())
                        .unwrap_or("unknown")
                        .to_string();

                    return Err(GatewayError::XAccountExists {
                        screen_name,
                        solana_address,
                    });
                }

                // If we couldn't parse the response, return a generic error
                return Err(GatewayError::RequestFailed);
            }

            // For all other errors, or if JSON parsing failed
            return Err(GatewayError::RequestFailed);
        }

        // Success path - deserialize JSON
        let body = resp.text().await.map_err(GatewayError::from)?;
        let access_token =
            serde_json::from_str::<AccessTokenResponse>(&body).map_err(GatewayError::from)?;
        Ok(access_token)
    }

    async fn link_x_account(
        &self,
        user_id: String,
        msg: String,
        signature: Signature,
        address: Pubkey,
        access_token: String,
    ) -> GatewayResult<i64> {
        let url = format!("{}/oauth/x/link_account", ORE_API_URL);
        let resp = self
            .http
            .post(url)
            .json(&LinkXAccountRequest {
                user_id,
                msg,
                signature,
                address,
                access_token,
            })
            .send()
            .await
            .map_err(GatewayError::from)?;
        let body = resp.text().await.map_err(GatewayError::from)?;
        let waitlist_num = i64::from_str(&body).map_err(|_| GatewayError::RequestFailed)?;
        Ok(waitlist_num)
    }

    async fn validate_waitlist_status(&self, address: Pubkey) -> GatewayResult<WaitlistStatus> {
        let url = format!("{}/oauth/x/check_waitlist", ORE_API_URL);

        let resp = self
            .http
            .get(url)
            .query(&[("address", address.to_string())])
            .send()
            .await
            .map_err(|e| {
                log::error!("Failed to send request: {:?}", e);
                GatewayError::from(e)
            })?;

        // Check if the response was successful
        if !resp.status().is_success() {
            return Err(GatewayError::RequestFailed);
        }

        let body = resp.text().await.map_err(GatewayError::from)?;

        // Parse json response as WaitlistResponse
        let waitlist_response =
            serde_json::from_str::<WaitlistResponse>(&body).map_err(|e| GatewayError::from(e))?;

        // Check if the account exists
        if waitlist_response.exists {
            // Account exists, return WaitlistStatus with data
            return Ok(WaitlistStatus {
                is_registered: true,
                screen_name: Some(waitlist_response.screen_name),
                waitlist_number: Some(waitlist_response.waitlist_number),
                profile_image_url: waitlist_response.profile_image_url,
            });
        }

        // Continue with normal flow if account doesn't exist
        Ok(WaitlistStatus {
            is_registered: false,
            screen_name: None,
            waitlist_number: None,
            profile_image_url: None,
        })
    }
}
