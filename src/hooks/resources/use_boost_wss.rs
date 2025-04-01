use std::collections::HashMap;

use base64::Engine;
use dioxus::prelude::*;

use base64::prelude::BASE64_STANDARD;
use ore_api::consts::MINT_ADDRESS;
use ore_boost_api::state::{boost_pda, Boost};
use solana_sdk::pubkey::Pubkey;
use steel::AccountDeserialize;

use crate::config::LISTED_BOOSTS;
use crate::gateway::ore::OreGateway;
use crate::gateway::{AccountNotificationParams, GatewayError, GatewayResult};
use crate::hooks::{use_gateway, use_wss_subscription};

pub(crate) fn use_boosts_wss_provider() {
    // Hashmap to cache resources
    let mut boosts = HashMap::new();

    // Idle ORE boost
    let boost_address = boost_pda(MINT_ADDRESS).0;
    boosts.insert(boost_address, use_boost_signal(boost_address));

    // Listed boosts
    for boost_meta in LISTED_BOOSTS.iter() {
        let boost_address = boost_pda(boost_meta.lp_mint).0;
        boosts.insert(boost_address, use_boost_signal(boost_address));
    }

    // Setup context provider
    use_context_provider(|| boosts);
}

fn use_boost_signal(boost_address: Pubkey) -> Signal<GatewayResult<Boost>> {
    // Create and initialize the data signal
    let mut data = use_signal(|| Err(GatewayError::AccountNotFound));
    let gateway = use_gateway();

    // Initialize data with current boost
    spawn(async move {
        match gateway.get_boost(boost_address).await {
            Ok(boost) => data.set(Ok(boost)),
            Err(err) => {
                log::error!("Failed to initialize boost: {:?}", err);
                data.set(Err(err));
            }
        }
    });

    let update_callback = move |notif: &AccountNotificationParams| {
        // Base64 decode
        let data = &notif.result.value.data;
        let data = data.first().ok_or(GatewayError::AccountNotFound)?;
        let data = BASE64_STANDARD
            .decode(data.clone())
            .map_err(|err| anyhow::anyhow!(err))?;

        // Unpack the boost account data
        let boost = *Boost::try_from_bytes(data.as_slice()).map_err(|err| anyhow::anyhow!(err))?;

        Ok(boost)
    };

    // subscribe
    let subscriber = use_wss_subscription(data.clone(), update_callback.clone());
    use_effect(move || subscriber.send(boost_address));

    data
}

pub fn use_boost_wss(mint_address: Pubkey) -> Signal<GatewayResult<Boost>> {
    let boosts: HashMap<Pubkey, Signal<GatewayResult<Boost>>> = use_context();
    let boost_address = boost_pda(mint_address).0;
    if let Some(boost) = boosts.get(&boost_address) {
        *boost
    } else {
        panic!("use_boost_wss: {:?} not found", mint_address);
        // use_boost_signal(mint_address)
    }
}

pub fn use_all_boosts() -> HashMap<Pubkey, Signal<GatewayResult<Boost>>> {
    use_context()
}
