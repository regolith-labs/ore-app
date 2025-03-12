use ore_pool_api::state::Member;
use ore_pool_types::{
    ContributePayloadV2, Member as MemberRecord, MemberChallenge, RegisterPayload,
};
use solana_sdk::pubkey::Pubkey;
use steel::AccountDeserialize;

use crate::hooks::MiningEvent;

use super::{solana::SolanaGateway, Gateway, GatewayError, GatewayResult, Rpc};

pub trait PoolGateway {
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
        async_std::task::sleep(std::time::Duration::from_secs(1)).await;
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
        log::info!("buffer time: {}", buffer_time);
        log::info!("clock: {:?}", unix_timestamp);
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

    async fn register(&self, authority: Pubkey, pool_url: String) -> GatewayResult<MemberRecord> {
        let post_url = format!("{}/register", pool_url);
        let body = RegisterPayload { authority };
        let resp = self
            .http
            .post(post_url)
            .json(&body)
            .send()
            .await
            .map_err(GatewayError::from)?;
        let member_record = resp
            .json::<MemberRecord>()
            .await
            .map_err(GatewayError::from)?;
        Ok(member_record)
    }
}
