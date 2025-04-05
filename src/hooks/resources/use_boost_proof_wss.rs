use base64::Engine;
use dioxus::prelude::*;

use base64::prelude::BASE64_STANDARD;
use ore_api::state::{proof_pda, Proof};
use steel::AccountDeserialize;

use crate::gateway::ore::OreGateway;
use crate::gateway::{AccountNotificationParams, GatewayError, GatewayResult};
use crate::hooks::{use_gateway, use_wss_subscription};

pub(crate) fn use_boost_proof_wss_provider() {
    use_context_provider(|| use_boost_proof_signal());
}

fn use_boost_proof_signal() -> Signal<GatewayResult<Proof>> {
    let boost_config_address = ore_boost_api::state::config_pda().0;
    let proof_address = proof_pda(boost_config_address).0;

    // Init
    let mut data = use_signal(|| Err(GatewayError::AccountNotFound));
    use_effect(move || {
        spawn(async move {
            let gateway = use_gateway();
            match gateway.get_proof(proof_address).await {
                Ok(boost) => data.set(Ok(boost)),
                Err(err) => {
                    log::error!("Failed to initialize boost proof: {:?}", err);
                    data.set(Err(err));
                }
            }
        });
    });

    // Update
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

pub fn use_boost_proof_wss() -> Signal<GatewayResult<Proof>> {
    use_context()
}
