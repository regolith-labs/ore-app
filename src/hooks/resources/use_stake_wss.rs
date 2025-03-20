use base64::Engine;
use dioxus::prelude::*;

use base64::prelude::BASE64_STANDARD;
use ore_boost_api::state::{boost_pda, stake_pda, Stake};
use solana_sdk::pubkey::Pubkey;
use steel::AccountDeserialize;

use crate::gateway::ore::OreGateway;
use crate::gateway::{AccountNotificationParams, GatewayError, GatewayResult};
use crate::hooks::{use_gateway, use_wallet, use_wss_subscription, Wallet};

pub fn use_stake_wss(mint_address: Pubkey) -> Signal<GatewayResult<Stake>> {
    // Create and initialize the data signal
    let boost_address = boost_pda(mint_address).0;
    let mut data = use_signal(|| Err(GatewayError::AccountNotFound));
    let gateway = use_gateway();
    let wallet = use_wallet();

    // Initialize data with current boost
    spawn(async move {
        if let Wallet::Connected(pubkey) = *wallet.read() {
            let stake_address = stake_pda(pubkey, boost_address).0;
            match gateway.get_stake(stake_address).await {
                Ok(stake) => data.set(Ok(stake)),
                Err(err) => {
                    log::error!("Failed to initialize stake: {:?}", err);
                    data.set(Err(err));
                }
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

        // Unpack the stake account data
        let stake = *Stake::try_from_bytes(data.as_slice()).map_err(|err| anyhow::anyhow!(err))?;

        Ok(stake)
    };

    // Set up WebSocket subscription when wallet is connected
    use_effect(move || {
        if let Wallet::Connected(pubkey) = *wallet.read() {
            let stake_address = stake_pda(pubkey, boost_address).0;
            use_wss_subscription(data.clone(), update_callback.clone(), stake_address);
        }
    });

    data
}
