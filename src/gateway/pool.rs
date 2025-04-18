use ore_pool_api::state::Member;
use ore_pool_types::{
    BalanceUpdate, ContributePayloadV2, Member as MemberRecord, MemberChallenge, RegisterPayload,
    UpdateBalancePayload,
};
use solana_sdk::{hash::Hash, pubkey::Pubkey, transaction::Transaction};
use steel::AccountDeserialize;

use crate::hooks::MiningEvent;

use super::{Gateway, GatewayError, GatewayResult, Rpc};

pub trait PoolGateway {
    async fn commit_claim(
        &self,
        authority: Pubkey,
        pool_url: String,
        transaction: Transaction,
        hash: Hash,
    ) -> GatewayResult<BalanceUpdate>;
    async fn get_challenge(
        &self,
        authority: Pubkey,
        pool_url: String,
    ) -> GatewayResult<MemberChallenge>;
    async fn get_cutoff(
        &self,
        last_hash_at: i64,
        unix_timestamp: i64,
        buffer_time: i64,
    ) -> GatewayResult<i64>;
    async fn get_member(&self, address: Pubkey) -> GatewayResult<Member>;
    async fn get_member_record(
        &self,
        authority: Pubkey,
        pool_url: String,
    ) -> GatewayResult<MemberRecord>;
    async fn poll_new_challenge(
        &self,
        authority: Pubkey,
        pool_url: String,
        last_hash_at: i64,
    ) -> GatewayResult<MemberChallenge>;
    async fn post_solution(
        &self,
        authority: Pubkey,
        pool_url: String,
        solution: &drillx::Solution,
    ) -> GatewayResult<()>;
    async fn register(&self, authority: Pubkey, pool_url: String) -> GatewayResult<MemberRecord>;
    async fn get_latest_event(
        &self,
        authority: Pubkey,
        pool_url: String,
    ) -> GatewayResult<MiningEvent>;
}

impl<R: Rpc> PoolGateway for Gateway<R> {
    async fn get_challenge(
        &self,
        authority: Pubkey,
        pool_url: String,
    ) -> GatewayResult<MemberChallenge> {
        let get_url = format!("{}/challenge/{}", pool_url, authority);
        let resp = self
            .http
            .get(get_url)
            .send()
            .await
            .map_err(GatewayError::from)?;
        let challenge = resp
            .json::<MemberChallenge>()
            .await
            .map_err(GatewayError::from)?;
        Ok(challenge)
    }

    async fn get_latest_event(
        &self,
        authority: Pubkey,
        pool_url: String,
    ) -> GatewayResult<MiningEvent> {
        crate::time::sleep(1000).await;
        let get_url = format!("{}/event/latest/{}", pool_url, authority);
        let resp = self
            .http
            .get(get_url)
            .send()
            .await
            .map_err(GatewayError::from)?;
        let latest_event = resp
            .json::<MiningEvent>()
            .await
            .map_err(GatewayError::from)?;
        Ok(latest_event)
    }

    async fn get_cutoff(
        &self,
        last_hash_at: i64,
        unix_timestamp: i64,
        buffer_time: i64,
    ) -> GatewayResult<i64> {
        let cutoff = last_hash_at
            .saturating_add(60)
            .saturating_sub(buffer_time)
            .saturating_sub(unix_timestamp)
            .max(0);
        Ok(cutoff)
    }

    async fn get_member(&self, address: Pubkey) -> GatewayResult<Member> {
        let data = self
            .rpc
            .get_account_data(&address)
            .await
            .map_err(GatewayError::from)?;
        Ok(*Member::try_from_bytes(&data)?)
    }

    async fn get_member_record(
        &self,
        authority: Pubkey,
        pool_url: String,
    ) -> GatewayResult<MemberRecord> {
        let get_url = format!("{}/member/{}", pool_url, authority);
        let resp = self
            .http
            .get(get_url)
            .send()
            .await
            .map_err(GatewayError::from)?;
        let member_record = resp
            .json::<MemberRecord>()
            .await
            .map_err(GatewayError::from)?;
        Ok(member_record)
    }

    async fn poll_new_challenge(
        &self,
        authority: Pubkey,
        pool_url: String,
        last_hash_at: i64,
    ) -> GatewayResult<MemberChallenge> {
        loop {
            log::info!("Polling...");
            match self.get_challenge(authority, pool_url.clone()).await {
                Ok(challenge) if challenge.challenge.lash_hash_at != last_hash_at => {
                    return Ok(challenge)
                }
                Ok(_) => {
                    log::info!("Same challenge, retry...");
                }
                Err(err) => {
                    log::error!("Error polling challenge: {:?}", err);
                }
            }
            async_std::task::sleep(std::time::Duration::from_secs(1)).await;
        }
    }

    async fn post_solution(
        &self,
        authority: Pubkey,
        pool_url: String,
        solution: &drillx::Solution,
    ) -> GatewayResult<()> {
        let post_url = format!("{}/contribute", pool_url);
        let payload = ContributePayloadV2 {
            authority,
            solution: *solution,
        };
        self.http
            .post(post_url)
            .json(&payload)
            .send()
            .await
            .map_err(GatewayError::from)?;
        Ok(())
    }

    async fn commit_claim(
        &self,
        authority: Pubkey,
        pool_url: String,
        transaction: Transaction,
        hash: Hash,
    ) -> GatewayResult<BalanceUpdate> {
        let post_url = format!("{}/commit", pool_url);
        let body = UpdateBalancePayload {
            authority,
            transaction,
            hash,
        };
        let resp = match self.http.post(post_url).json(&body).send().await {
            Ok(response) => response,
            Err(err) => {
                log::error!("Error sending commit request: {:?}", err);
                return Err(GatewayError::from(err));
            }
        };
        let resp_text = resp.text().await.unwrap_or_default();
        let balance_update = match serde_json::from_str::<BalanceUpdate>(&resp_text) {
            Ok(update) => update,
            Err(err) => {
                log::error!("Error deserializing response as BalanceUpdate: {:?}", err);
                let error_text = format!("Server response: {}", resp_text);
                log::error!("{}", error_text);
                return Err(anyhow::anyhow!("{}", error_text).into());
            }
        };
        Ok(balance_update)
    }

    async fn register(&self, authority: Pubkey, pool_url: String) -> GatewayResult<MemberRecord> {
        let post_url = format!("{}/register", pool_url);
        let body = RegisterPayload { authority };
        let resp = match self.http.post(post_url).json(&body).send().await {
            Ok(response) => response,
            Err(err) => {
                log::error!("Error sending request: {:?}", err);
                return Err(GatewayError::from(err));
            }
        };
        let member_record = match resp.text().await {
            Ok(text) => match serde_json::from_str::<serde_json::Value>(&text) {
                Ok(value) => match serde_json::from_value::<MemberRecord>(value) {
                    Ok(record) => record,
                    Err(err) => {
                        log::error!("Error deserializing member record from value: {:?}", err);
                        log::error!("Raw response: {}", text);
                        return Err(GatewayError::from(err));
                    }
                },
                Err(err) => {
                    log::error!("Error deserializing response as JSON: {:?}", err);
                    log::error!("Raw response: {}", text);
                    return Err(GatewayError::from(err));
                }
            },
            Err(err) => {
                log::error!("Error reading response text: {:?}", err);
                return Err(GatewayError::from(err));
            }
        };
        Ok(member_record)
    }
}
