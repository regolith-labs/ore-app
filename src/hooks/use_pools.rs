use dioxus::hooks::use_resource;
use dioxus::hooks::Resource;
use ore_pool_types::ContributePayloadV2;
use ore_pool_types::Member;
use ore_pool_types::MemberChallenge;
use ore_pool_types::RegisterPayload;
use solana_sdk::pubkey::Pubkey;
use steel::AccountDeserialize;

use crate::gateway::GatewayError;
use crate::gateway::solana::SolanaGateway;
use crate::gateway::GatewayResult;
use crate::gateway::Rpc;
use crate::time::Duration;

use super::use_gateway;
use super::use_wallet;
use super::GetPubkey;


pub fn use_register_db(pool_url: String) -> Resource<GatewayResult<Member>> {
    let wallet = use_wallet();
    use_resource(move || {
        let gateway = use_gateway();
        let pool_url = pool_url.clone();
        async move {
            async_std::task::sleep(Duration::from_millis(5_500)).await;
            let pubkey = wallet.pubkey()?;
            let post_url = format!("{}/register", pool_url);
            let body = RegisterPayload { authority: pubkey };
            let resp = gateway.http.post(post_url).json(&body).send().await;
            match resp {
                Err(err) => {
                    log::error!("{:?}", err);
                    Err(err).map_err(From::from)
                }
                Ok(resp) => resp.json::<Member>().await.map_err(From::from),
            }
        }
    })
}

pub async fn get_updated_challenge(
    http_client: &reqwest::Client,
    pool_url: &str,
    miner: &str,
    last_hash_at: i64,
) -> GatewayResult<MemberChallenge> {
    loop {
        let challenge = get_challenge(http_client, pool_url, miner).await?;
        if challenge.challenge.lash_hash_at == last_hash_at {
            async_std::task::sleep(std::time::Duration::from_secs(1)).await;
        } else {
            return Ok(challenge);
        }
    }
}

pub async fn post_solution(
    http_client: &reqwest::Client,
    pool_url: &str,
    miner: &Pubkey,
    solution: &drillx::Solution,
) -> GatewayResult<()> {
    let post_url = format!("{}/contribute", pool_url);
    let payload = ContributePayloadV2 {
        authority: *miner,
        solution: *solution,
    };
    let resp = http_client.post(post_url).json(&payload).send().await?;
    match resp.error_for_status() {
        Err(err) => {
            log::error!("{:?}", err);
            Err(err).map_err(From::from)
        }
        Ok(_) => Ok(()),
    }
}

async fn get_challenge(
    http_client: &reqwest::Client,
    pool_url: &str,
    miner: &str,
) -> GatewayResult<MemberChallenge> {
    let get_url = format!("{}/challenge/{}", pool_url, miner);
    let resp = http_client.get(get_url).send().await?;
    match resp.error_for_status() {
        Err(err) => {
            log::error!("{:?}", err);
            Err(err).map_err(From::from)
        }
        Ok(resp) => resp.json::<MemberChallenge>().await.map_err(From::from),
    }
}

pub fn use_member_db(pool_url: String) -> Resource<GatewayResult<Member>> {
    let wallet = use_wallet();
    use_resource(move || {
        let gateway = use_gateway();
        let pool_url = pool_url.clone();
        async move {
            let pubkey = wallet.pubkey()?;
            let get_url = format!("{}/member/{}", pool_url, pubkey);
            let resp = gateway.http.get(get_url).send().await?;
            match resp.error_for_status() {
                Err(err) => {
                    log::error!("{:?}", err);
                    Err(err).map_err(From::from)
                }
                Ok(resp) => resp.json::<Member>().await.map_err(From::from),
            }
        }
    })
}

pub async fn get_cutoff(
    last_hash_at: i64,
    buffer_time: i64,
) -> GatewayResult<i64> {
    let gateway = use_gateway();
    let clock = gateway.rpc.get_clock().await?;
    let cutoff = last_hash_at
        .saturating_add(60)
        .saturating_sub(buffer_time)
        .saturating_sub(clock.unix_timestamp)
        .max(0);
    Ok(cutoff)
}
