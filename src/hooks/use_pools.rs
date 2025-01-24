use std::str::FromStr;

use dioxus::hooks::use_resource;
use dioxus::hooks::Resource;
use once_cell::sync::Lazy;
use ore_pool_types::ContributePayloadV2;
use ore_pool_types::Member;
use ore_pool_types::MemberChallenge;
use ore_pool_types::RegisterPayload;
use serde::{Deserialize, Deserializer};
use solana_sdk::clock::Clock;
use solana_sdk::compute_budget::ComputeBudgetInstruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::transaction::Transaction;
use steel::AccountDeserialize;

use crate::gateway::GatewayError;
use crate::gateway::GatewayResult;
use crate::gateway::Rpc;
use crate::steel_app::time::Duration;

use super::use_gateway;
use super::{use_wallet, GetPubkey};

pub static POOLS: Lazy<Vec<Pool>> = Lazy::new(|| {
    // Read the YAML file at compile time
    let yaml_str = include_str!("../../public/config/listed-pools.yaml");

    // Parse the config
    let config: PoolConfig =
        serde_yaml::from_str(yaml_str).expect("Failed to parse listed-pools.yaml");

    config.pools
});

pub const FIRST_POOL: Lazy<Pool> = Lazy::new(|| {
    POOLS
        .first()
        .expect("Must be at least one entry in listed-pools.yaml")
        .clone()
});

#[derive(Clone, PartialEq, Eq, Deserialize)]
pub struct Pool {
    #[serde(deserialize_with = "deserialize_pubkey")]
    pub address: Pubkey,
    pub url: String,
    pub name: String,
    pub description: String,
    pub image: String,
}

#[derive(Deserialize)]
struct PoolConfig {
    pools: Vec<Pool>,
}

fn deserialize_pubkey<'de, D>(deserializer: D) -> Result<Pubkey, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Pubkey::from_str(&s).map_err(serde::de::Error::custom)
}

pub fn use_register_onchain(pool_address: Pubkey) -> Resource<GatewayResult<Transaction>> {
    let wallet = use_wallet();
    use_resource(move || {
        let gateway = use_gateway();
        async move {
            let pubkey = wallet.get_pubkey()?;
            let compute_budget_ix = ComputeBudgetInstruction::set_compute_unit_limit(20_000);
            let join_ix = ore_pool_api::sdk::join(pubkey, pool_address, pubkey);
            let mut tx = Transaction::new_with_payer(&[compute_budget_ix, join_ix], Some(&pubkey));
            let hash = gateway.rpc.get_latest_blockhash().await?;
            tx.message.recent_blockhash = hash;
            Ok(tx)
        }
    })
}

pub fn use_register_db(pool_url: String) -> Resource<GatewayResult<Member>> {
    let wallet = use_wallet();
    use_resource(move || {
        let gateway = use_gateway();
        let pool_url = pool_url.clone();
        async move {
            async_std::task::sleep(Duration::from_millis(5_500)).await;
            let pubkey = wallet.get_pubkey()?;
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
            let pubkey = wallet.get_pubkey()?;
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

pub async fn get_cutoff<R: Rpc>(
    rpc_client: &R,
    last_hash_at: i64,
    buffer_time: i64,
) -> GatewayResult<i64> {
    let clock = get_clock(rpc_client).await?;
    let cutoff = last_hash_at
        .saturating_add(60)
        .saturating_sub(buffer_time)
        .saturating_sub(clock.unix_timestamp)
        .max(0);
    Ok(cutoff)
}

async fn get_clock<R: Rpc>(rpc_client: &R) -> GatewayResult<Clock> {
    let data = rpc_client
        .get_account_data(&solana_sdk::sysvar::clock::ID)
        .await?;
    bincode::deserialize::<Clock>(data.as_slice())
        .map_err(|_err| GatewayError::FailedDeserialization)
}

pub fn use_member_onchain(
    pool_address: Pubkey,
) -> Resource<GatewayResult<ore_pool_api::state::Member>> {
    let wallet = use_wallet();
    use_resource(move || {
        let gateway = use_gateway();
        async move {
            async_std::task::sleep(Duration::from_millis(5_000)).await;
            let pubkey = wallet.get_pubkey()?;
            get_member_onchain(&gateway.rpc, &pool_address, &pubkey).await
        }
    })
}

async fn get_member_onchain<R: Rpc>(
    rpc_client: &R,
    pool_address: &Pubkey,
    miner: &Pubkey,
) -> GatewayResult<ore_pool_api::state::Member> {
    let (member_pda, _) = ore_pool_api::state::member_pda(*miner, *pool_address);
    let data = rpc_client.get_account_data(&member_pda).await?;
    let member = ore_pool_api::state::Member::try_from_bytes(data.as_slice())?;
    Ok(*member)
}
