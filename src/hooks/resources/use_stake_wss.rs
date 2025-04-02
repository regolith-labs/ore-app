use std::collections::HashMap;

use base64::Engine;
use dioxus::prelude::*;

use base64::prelude::BASE64_STANDARD;
use ore_api::consts::MINT_ADDRESS;
use ore_boost_api::state::{boost_pda, stake_pda, Stake};
use solana_sdk::pubkey::Pubkey;
use steel::AccountDeserialize;

use crate::config::LISTED_BOOSTS;
use crate::gateway::ore::OreGateway;
use crate::gateway::{AccountNotificationParams, GatewayError, GatewayResult, UiTokenAmount};
use crate::hooks::{use_gateway, use_wallet, use_wss_subscription, Wallet};
use crate::solana::spl_token::ui_amount_to_amount;
use crate::utils::LiquidityPair;

pub(crate) fn use_stakes_wss_provider() {
    // Hashmap to cache resources
    let mut stakes = HashMap::new();

    // Idle ORE boost
    stakes.insert(MINT_ADDRESS, use_stake_signal(MINT_ADDRESS));

    // Listed boosts
    for boost_meta in LISTED_BOOSTS.iter() {
        stakes.insert(boost_meta.lp_mint, use_stake_signal(boost_meta.lp_mint));
    }

    // Setup context provider
    use_context_provider(|| stakes);
}

fn use_stake_signal(mint_address: Pubkey) -> Signal<GatewayResult<Stake>> {
    // Create and initialize the data signal
    let boost_address = boost_pda(mint_address).0;
    let mut data = use_signal(|| Err(GatewayError::AccountNotFound));
    let wallet = use_wallet();

    // Initialize data with current boost
    use_effect(move || {
        if let Wallet::Connected(pubkey) = *wallet.read() {
            let stake_address = stake_pda(pubkey, boost_address).0;
            log::info!("stake address: {:?}", stake_address);
            spawn(async move {
                let gateway = use_gateway();
                match gateway.get_stake(stake_address).await {
                    Ok(stake) => data.set(Ok(stake)),
                    Err(err) => {
                        log::error!("Failed to initialize stake: {:?}", err);
                        data.set(Err(err));
                    }
                }
            });
        } else {
            log::error!("wallet missing");
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

    // Subscribe
    let subscriber = use_wss_subscription(data.clone(), update_callback.clone());
    use_effect(move || {
        if let Wallet::Connected(pubkey) = *wallet.read() {
            let stake_address = stake_pda(pubkey, boost_address).0;
            subscriber.send(stake_address);
        }
    });

    data
}

pub fn use_stake_wss(mint_address: Pubkey) -> Signal<GatewayResult<Stake>> {
    let stakes: HashMap<Pubkey, Signal<GatewayResult<Stake>>> = use_context();
    if let Some(stake) = stakes.get(&mint_address) {
        *stake
    } else {
        panic!("use_stake_wss: {:?} not found", mint_address);
    }
}

pub fn use_all_stakes() -> HashMap<Pubkey, Signal<GatewayResult<Stake>>> {
    use_context()
}

pub fn use_withdrawable_balances(
    liquidity_pair: Resource<GatewayResult<LiquidityPair>>,
    stake: Signal<GatewayResult<Stake>>,
) -> (
    Signal<GatewayResult<UiTokenAmount>>,
    Signal<GatewayResult<UiTokenAmount>>,
) {
    let stake_a_balance = use_signal(|| {
        let Ok(stake) = stake.cloned() else {
            return Err(GatewayError::Unknown);
        };
        let Some(Ok(liquidity_pair)) = liquidity_pair.cloned() else {
            return Err(GatewayError::Unknown);
        };
        let percentage_shares = stake.balance as f64 / liquidity_pair.shares as f64;
        let amount_f64 = liquidity_pair.balance_a_f64 * percentage_shares;
        let token_a_decimals = liquidity_pair.token_a.decimals;
        let amount_u64 = ui_amount_to_amount(amount_f64, token_a_decimals);
        Ok(UiTokenAmount {
            ui_amount: Some(amount_f64),
            ui_amount_string: format!("{:.1$}", amount_f64, token_a_decimals as usize)
                .trim_end_matches("0")
                .trim_end_matches(".")
                .to_string(),
            amount: amount_u64.to_string(),
            decimals: token_a_decimals as u8,
        })
    });

    let stake_b_balance = use_signal(|| {
        let Ok(stake) = stake.cloned() else {
            return Err(GatewayError::Unknown);
        };
        let Some(Ok(liquidity_pair)) = liquidity_pair.cloned() else {
            return Err(GatewayError::Unknown);
        };
        let percentage_shares = stake.balance as f64 / liquidity_pair.shares as f64;
        let amount_f64 = liquidity_pair.balance_b_f64 * percentage_shares;
        let token_b_decimals = liquidity_pair.token_b.decimals;
        let amount_u64 = ui_amount_to_amount(amount_f64, token_b_decimals);
        Ok(UiTokenAmount {
            ui_amount: Some(amount_f64),
            ui_amount_string: format!("{:.1$}", amount_f64, token_b_decimals as usize)
                .trim_end_matches("0")
                .trim_end_matches(".")
                .to_string(),
            amount: amount_u64.to_string(),
            decimals: token_b_decimals as u8,
        })
    });

    (stake_a_balance, stake_b_balance)
}
