use std::str::FromStr;

use dioxus::hooks::use_resource;
use dioxus::hooks::Resource;
use once_cell::sync::Lazy;
use ore_pool_types::Member;
use ore_pool_types::RegisterPayload;
use serde::{Deserialize, Deserializer};
use solana_client_wasm::solana_sdk::compute_budget::ComputeBudgetInstruction;
use solana_client_wasm::solana_sdk::transaction::Transaction;
use solana_client_wasm::WasmClient;
use steel::AccountDeserialize;

use crate::steel_app::time::Duration;
use crate::{gateway::GatewayResult, steel_app::solana::sdk::pubkey::Pubkey};

use super::use_gateway;
use super::{use_wallet_status, GetPubkey};

pub static POOLS: Lazy<Vec<Pool>> = Lazy::new(|| {
    // Read the YAML file at compile time
    let yaml_str = include_str!("../../public/config/listed-pools.yaml");

    // Parse the config
    let config: PoolConfig =
        serde_yaml::from_str(yaml_str).expect("Failed to parse listed-pools.yaml");

    config.pools
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
    let wallet_status = use_wallet_status();
    use_resource(move || {
        let gateway = use_gateway();
        async move {
            let pubkey = wallet_status.get_pubkey()?;
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
    let wallet_status = use_wallet_status();
    use_resource(move || {
        let gateway = use_gateway();
        let pool_url = pool_url.clone();
        async move {
            async_std::task::sleep(Duration::from_millis(5_500)).await;
            let pubkey = wallet_status.get_pubkey()?;
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

pub fn use_member_db(pool_url: String) -> Resource<GatewayResult<Member>> {
    let wallet_status = use_wallet_status();
    use_resource(move || {
        let gateway = use_gateway();
        let pool_url = pool_url.clone();
        async move {
            let pubkey = wallet_status.get_pubkey()?;
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

pub fn use_member_onchain(
    pool_address: Pubkey,
) -> Resource<GatewayResult<ore_pool_api::state::Member>> {
    let wallet_status = use_wallet_status();
    use_resource(move || {
        let gateway = use_gateway();
        async move {
            async_std::task::sleep(Duration::from_millis(5_000)).await;
            let pubkey = wallet_status.get_pubkey()?;
            get_member_onchain(&gateway.rpc, pool_address, pubkey).await
        }
    })
}

async fn get_member_onchain(
    rpc_client: &WasmClient,
    pool_address: Pubkey,
    miner: Pubkey,
) -> GatewayResult<ore_pool_api::state::Member> {
    let (member_pda, _) = ore_pool_api::state::member_pda(miner, pool_address);
    let data = rpc_client.get_account_data(&member_pda).await?;
    let member = ore_pool_api::state::Member::try_from_bytes(data.as_slice())?;
    Ok(*member)
}
