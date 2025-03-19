use base64::Engine;
use dioxus::prelude::*;

use base64::prelude::BASE64_STANDARD;
use solana_sdk::native_token::lamports_to_sol;
use solana_sdk::program_pack::Pack;
use solana_sdk::pubkey::Pubkey;

use crate::config::Token;
use crate::gateway::{AccountNotificationParams, GatewayError, GatewayResult, UiTokenAmount};
use crate::hooks::{use_wallet, use_wss_subscription, Wallet};

use super::get_token_balance;

pub fn use_sol_balance_wss() -> Signal<GatewayResult<UiTokenAmount>> {
    let update_callback = move |notif: &AccountNotificationParams| {
        let lamports = notif.result.value.lamports;
        let sol = lamports_to_sol(lamports);
        let token_amount = UiTokenAmount {
            ui_amount: Some(sol),
            decimals: 8,
            amount: format!("{}", lamports),
            ui_amount_string: format!("{}", sol),
        };
        Ok(token_amount)
    };

    use_balance_wss(Token::sol().mint, update_callback)
}

pub fn use_token_balance_wss(token: Token) -> Signal<GatewayResult<UiTokenAmount>> {
    let update_callback = move |notif: &AccountNotificationParams| {
        let data = &notif.result.value.data;
        let data = data.first().ok_or(GatewayError::AccountNotFound)?;
        let data = BASE64_STANDARD
            .decode(data.clone())
            .map_err(|err| anyhow::anyhow!(err))?;

        // Unpack the token account data
        let token_account = crate::solana::spl_token::state::Account::unpack(data.as_slice())
            .map_err(|err| anyhow::anyhow!(err))?;

        let amount = token_account.amount;
        let ui_amount = amount as f64 / 10f64.powi(token.decimals as i32);
        let token_amount = UiTokenAmount {
            ui_amount: Some(ui_amount),
            decimals: token.decimals,
            amount: amount.to_string(),
            ui_amount_string: ui_amount.to_string(),
        };

        Ok(token_amount)
    };

    use_balance_wss(token.mint, update_callback)
}

/// Common impl for token balance subscriptions
fn use_balance_wss<U>(mint: Pubkey, update_callback: U) -> Signal<GatewayResult<UiTokenAmount>>
where
    U: Fn(&AccountNotificationParams) -> GatewayResult<UiTokenAmount> + Clone + 'static,
{
    let wallet = use_wallet();

    // Create and initialize the data signal
    let mut data = use_signal(|| Err(GatewayError::AccountNotFound));

    // Initialize data with current balance
    spawn(async move {
        if let Wallet::Connected(pubkey) = *wallet.read() {
            match get_token_balance(pubkey, mint).await {
                Ok(initial_data) => data.set(Ok(initial_data)),
                Err(err) => {
                    log::error!("Failed to initialize token balance: {:?}", err);
                    data.set(Err(err));
                }
            }
        }
    });

    // Set up WebSocket subscription when wallet is connected
    use_effect(move || {
        if let Wallet::Connected(pubkey) = *wallet.read() {
            let address = match mint.eq(&Token::sol().mint) {
                true => pubkey,
                false => crate::solana::spl_associated_token_account::get_associated_token_address(
                    &pubkey, &mint,
                ),
            };
            use_wss_subscription(data.clone(), update_callback.clone(), address);
        }
    });

    data
}
