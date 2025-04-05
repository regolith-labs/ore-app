use base64::Engine;
use dioxus::prelude::*;

use base64::prelude::BASE64_STANDARD;
use ore_boost_api::state::Config as BoostConfig;
use steel::AccountDeserialize;

use crate::gateway::ore::OreGateway;
use crate::gateway::{AccountNotificationParams, GatewayError, GatewayResult};
use crate::hooks::{use_gateway, use_wss_subscription};

pub(crate) fn use_boost_config_wss_provider() {
    use_context_provider(|| use_boost_config_signal());
}

fn use_boost_config_signal() -> Signal<GatewayResult<BoostConfig>> {
    let boost_config_address = ore_boost_api::state::config_pda().0;

    // Init
    let mut data = use_signal(|| Err(GatewayError::AccountNotFound));
    use_effect(move || {
        spawn(async move {
            let gateway = use_gateway();
            match gateway.get_boost_config(boost_config_address).await {
                Ok(boost_config) => data.set(Ok(boost_config)),
                Err(err) => {
                    log::error!("Failed to initialize boost config: {:?}", err);
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

        // Unpack the boost config account data
        let boost_config =
            *BoostConfig::try_from_bytes(data.as_slice()).map_err(|err| anyhow::anyhow!(err))?;
        Ok(boost_config)
    };

    // Subscribe
    let subscriber = use_wss_subscription(data.clone(), update_callback.clone());
    use_memo(move || {
        subscriber.send(boost_config_address);
    });

    data
}

pub fn use_boost_config_wss() -> Signal<GatewayResult<BoostConfig>> {
    use_context()
}
