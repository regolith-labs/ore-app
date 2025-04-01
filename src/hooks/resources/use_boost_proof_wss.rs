use std::collections::HashMap;

use base64::Engine;
use dioxus::prelude::*;

use base64::prelude::BASE64_STANDARD;
use ore_api::consts::MINT_ADDRESS;
use ore_api::state::{proof_pda, Proof};
use ore_boost_api::state::boost_pda;
use solana_sdk::pubkey::Pubkey;
use steel::AccountDeserialize;

use crate::config::LISTED_BOOSTS;
use crate::gateway::ore::OreGateway;
use crate::gateway::{AccountNotificationParams, GatewayError, GatewayResult};
use crate::hooks::{use_gateway, use_wss_subscription};

pub(crate) fn use_boost_proofs_wss_provider() {
    // Hashmap to cache resources
    let mut boost_proofs = HashMap::new();

    // Idle ORE boost
    let boost_address = boost_pda(MINT_ADDRESS).0;
    let proof_address = proof_pda(boost_address).0;
    boost_proofs.insert(proof_address, use_boost_proof_signal(proof_address));

    // Listed boosts
    for boost_meta in LISTED_BOOSTS.iter() {
        let boost_address = boost_pda(boost_meta.lp_mint).0;
        let proof_address = proof_pda(boost_address).0;
        boost_proofs.insert(proof_address, use_boost_proof_signal(proof_address));
    }

    // Setup context provider
    use_context_provider(|| boost_proofs);
}

fn use_boost_proof_signal(proof_address: Pubkey) -> Signal<GatewayResult<Proof>> {
    let gateway = use_gateway();
    // Create and initialize the data signal
    let mut data = use_signal(|| Err(GatewayError::AccountNotFound));
    spawn(async move {
        match gateway.get_proof(proof_address).await {
            Ok(boost) => data.set(Ok(boost)),
            Err(err) => {
                log::error!("Failed to initialize boost proof: {:?}", err);
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

        // Unpack the proof account data
        let proof = *Proof::try_from_bytes(data.as_slice()).map_err(|err| anyhow::anyhow!(err))?;

        Ok(proof)
    };

    // Subscribe
    let subscriber = use_wss_subscription(data.clone(), update_callback.clone());
    use_memo(move || {
        subscriber.send(proof_address);
    });

    data
}

pub fn use_boost_proof_wss(mint_address: Pubkey) -> Signal<GatewayResult<Proof>> {
    let boost_proofs: HashMap<Pubkey, Signal<GatewayResult<Proof>>> = use_context();
    let boost_address = boost_pda(mint_address).0;
    let proof_address = proof_pda(boost_address).0;
    if let Some(boost_proof) = boost_proofs.get(&proof_address) {
        *boost_proof
    } else {
        panic!("use_boost_proof_wss: {:?} not found", mint_address);
    }
}

pub fn use_all_boost_proofs() -> HashMap<Pubkey, Signal<GatewayResult<Proof>>> {
    use_context()
}
