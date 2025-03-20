use base64::Engine;
use dioxus::prelude::*;

use base64::prelude::BASE64_STANDARD;
use ore_api::state::{proof_pda, Proof};
use ore_boost_api::state::boost_pda;
use solana_sdk::pubkey::Pubkey;
use steel::AccountDeserialize;

use crate::gateway::ore::OreGateway;
use crate::gateway::{AccountNotificationParams, GatewayError, GatewayResult};
use crate::hooks::{use_gateway, use_wss_subscription};

pub fn use_boost_proof_wss(mint_address: Pubkey) -> Signal<GatewayResult<Proof>> {
    // Create and initialize the data signal
    let boost_address = boost_pda(mint_address).0;
    let proof_address = proof_pda(boost_address).0;
    let mut data = use_signal(|| Err(GatewayError::AccountNotFound));
    let gateway = use_gateway();

    // Initialize data with current boost
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

    // Set up WebSocket subscription when wallet is connected
    use_wss_subscription(data.clone(), update_callback.clone(), proof_address);

    data
}
