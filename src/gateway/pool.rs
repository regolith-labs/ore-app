use ore_pool_api::state::Member;
use ore_pool_types::{ContributePayloadV2, Member as MemberRecord, MemberChallenge, RegisterPayload, PoolMemberMiningEvent};
use solana_sdk::pubkey::Pubkey;
use steel::AccountDeserialize;

use super::{Gateway, GatewayError, GatewayResult, Rpc, solana::SolanaGateway};

pub trait PoolGateway {
    async fn get_challenge(&self, authority: Pubkey, pool_url: String) -> GatewayResult<MemberChallenge>;
    async fn get_cutoff(&self, last_hash_at: i64, buffer_time: i64) -> GatewayResult<i64>;
    async fn get_member(&self, address: Pubkey) -> GatewayResult<Member>;
    async fn get_member_record(&self, authority: Pubkey, pool_url: String) -> GatewayResult<MemberRecord>;
    async fn poll_new_challenge(&self, authority: Pubkey, pool_url: String, last_hash_at: i64) -> GatewayResult<MemberChallenge>;
    async fn post_solution(&self, authority: Pubkey, pool_url: String, solution: &drillx::Solution) -> GatewayResult<()>;
    async fn register(&self, authority: Pubkey, pool_url: String) -> GatewayResult<MemberRecord>;
    async fn get_latest_event(&self, authority: Pubkey, pool_url: String) -> GatewayResult<PoolMemberMiningEvent>;
}

impl<R: Rpc> PoolGateway for Gateway<R> {
    async fn get_challenge(&self, authority: Pubkey, pool_url: String) -> GatewayResult<MemberChallenge> {
        let get_url = format!("{}/challenge/{}", pool_url, authority);
        let resp = self.http.get(get_url).send().await.map_err(GatewayError::from)?;
        let challenge = resp.json::<MemberChallenge>().await.map_err(GatewayError::from)?;
        Ok(challenge)
    }

    async fn get_latest_event(&self, authority: Pubkey, pool_url: String) -> GatewayResult<PoolMemberMiningEvent> {
        let get_url = format!("{}/event/latest/{}", pool_url, authority);
        let resp = self.http.get(get_url).send().await.map_err(GatewayError::from)?;
        let latest_event = resp.json::<PoolMemberMiningEvent>().await.map_err(GatewayError::from)?;
        log::info!("Latest event in get_latest_event: {:?}", latest_event);
        Ok(latest_event)
    }

    async fn get_cutoff(&self, last_hash_at: i64, buffer_time: i64) -> GatewayResult<i64> {
        let clock = self.rpc.get_clock().await?;
        let cutoff = last_hash_at
            .saturating_add(60)
            .saturating_sub(buffer_time)
            .saturating_sub(clock.unix_timestamp)
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

    async fn get_member_record(&self, authority: Pubkey, pool_url: String) -> GatewayResult<MemberRecord> {
        let get_url = format!("{}/member/{}", pool_url, authority);
        let resp = self.http.get(get_url).send().await.map_err(GatewayError::from)?;
        let member_record = resp.json::<MemberRecord>().await.map_err(GatewayError::from)?;
        Ok(member_record)
    }

    async fn poll_new_challenge(&self, authority: Pubkey, pool_url: String, last_hash_at: i64) -> GatewayResult<MemberChallenge> {
        loop {
            log::info!("Polling...");
            let challenge = self.get_challenge(authority, pool_url.clone()).await?;
            if challenge.challenge.lash_hash_at == last_hash_at {
                async_std::task::sleep(std::time::Duration::from_secs(1)).await;
            } else {
                return Ok(challenge);
            }
        }
    }

    async fn post_solution(&self, authority: Pubkey, pool_url: String, solution: &drillx::Solution) -> GatewayResult<()> {
        let post_url = format!("{}/contribute", pool_url);
        let payload = ContributePayloadV2 {
            authority,
            solution: *solution,
        };
        self.http.post(post_url).json(&payload).send().await.map_err(GatewayError::from)?;
        Ok(())
    }

    async fn register(&self, authority: Pubkey, pool_url: String) -> GatewayResult<MemberRecord> {
        let post_url = format!("{}/register", pool_url);
        let body = RegisterPayload { authority };
        let resp = self.http.post(post_url).json(&body).send().await.map_err(GatewayError::from)?;
        let member_record = resp.json::<MemberRecord>().await.map_err(GatewayError::from)?;
        Ok(member_record)
    }
}
