use base64::Engine;
use dioxus::prelude::*;

use base64::prelude::BASE64_STANDARD;
use ore_api::consts::TOKEN_DECIMALS;
use ore_boost_api::state::reserve_pda;
use solana_extra_wasm::program::spl_token::amount_to_ui_amount;
use solana_sdk::program_pack::Pack;

use crate::gateway::spl::SplGateway;
use crate::gateway::{AccountNotificationParams, GatewayError, GatewayResult, UiTokenAmount};
use crate::hooks::{use_gateway, use_wss_subscription};
use crate::solana::{spl_associated_token_account, spl_token};

pub(crate) fn use_reserve_balance_wss_provider() {
    let signal = use_reserve_balance_signal();
    use_context_provider(|| signal);
}

fn use_reserve_balance_signal() -> Signal<GatewayResult<UiTokenAmount>> {
    let reserve_address = reserve_pda().0;
    let reserve_tokens_address = use_memo(move || {
        spl_associated_token_account::get_associated_token_address(
            &reserve_address,
            &ore_api::consts::MINT_ADDRESS,
        )
    });

    // Init
    let mut data = use_signal(|| Err(GatewayError::AccountNotFound));
    let _ = use_resource(move || async move {
        let gateway = use_gateway();
        match gateway
            .rpc
            .get_token_balance(&reserve_address, &ore_api::consts::MINT_ADDRESS)
            .await
        {
            Ok(balance) => {
                log::info!("balance: {:?}", balance);
                data.set(Ok(balance))
            }
            Err(err) => {
                log::error!("Failed to fetch claimable balance: {:?}", err);
                data.set(Err(err));
            }
        }
    });

    // Update
    let update_callback = move |notif: &AccountNotificationParams| {
        // Base64 decode
        let data = &notif.result.value.data;
        let data = data.first().ok_or(GatewayError::AccountNotFound)?;
        let data = BASE64_STANDARD
            .decode(data.clone())
            .map_err(|err| anyhow::anyhow!(err))?;

        // Unpack the market account data
        if let Ok(token_account) = spl_token::state::Account::unpack(data.as_slice()) {
            let amount_f64 = amount_to_ui_amount(token_account.amount, TOKEN_DECIMALS);
            Ok(UiTokenAmount {
                ui_amount: Some(amount_f64),
                decimals: TOKEN_DECIMALS,
                amount: token_account.amount.to_string(),
                ui_amount_string: amount_f64.to_string(),
            })
        } else {
            Err(GatewayError::FailedDeserialization)
        }
    };

    // Subscribe
    let subscriber = use_wss_subscription(data.clone(), update_callback.clone());
    use_effect(move || subscriber.send(reserve_tokens_address()));

    data
}

pub fn use_reserve_balance_wss() -> Signal<GatewayResult<UiTokenAmount>> {
    use_context()
}
