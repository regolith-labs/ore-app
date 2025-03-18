use dioxus::prelude::*;
use ore_api::consts::MINT_ADDRESS;
use solana_sdk::{native_token::lamports_to_sol, pubkey::Pubkey};
use std::collections::HashMap;

use crate::{
    config::{Token, LISTED_TOKENS},
    gateway::{
        spl::SplGateway, AccountSubscribe, AccountSubscribeGateway, GatewayError, GatewayResult,
        Rpc, UiTokenAmount,
    },
    hooks::{FromWssMsg, GetPubkey, ToWssMsg},
    utils::LiquidityPair,
};

use crate::hooks::{use_gateway, use_wallet, Wallet};

use super::{use_ore_price, use_wss, OrePrice};

pub(crate) fn use_token_balance_provider() {
    let mut token_balances = HashMap::new();

    for pubkey in LISTED_TOKENS.keys() {
        let token_mint = *pubkey;
        token_balances.insert(token_mint, use_token_balance_resource(token_mint));
    }

    use_context_provider(|| token_balances);
}

fn use_token_balance_resource(mint: Pubkey) -> Resource<GatewayResult<UiTokenAmount>> {
    let wallet_status = use_wallet();
    use_resource(move || async move {
        match *wallet_status.read() {
            Wallet::Disconnected => Err(GatewayError::AccountNotFound.into()),
            Wallet::Connected(pubkey) => get_token_balance(pubkey, mint).await,
        }
    })
}

pub fn use_token_balance(mint: Pubkey) -> Resource<GatewayResult<UiTokenAmount>> {
    let token_balances: HashMap<Pubkey, Resource<GatewayResult<UiTokenAmount>>> = use_context();
    if let Some(balance) = token_balances.get(&mint) {
        *balance
    } else {
        use_token_balance_resource(mint)
    }
}

pub fn use_token_balance_for_token(
    token: Signal<Option<Token>>,
) -> Resource<GatewayResult<UiTokenAmount>> {
    let token_balances: HashMap<Pubkey, Resource<GatewayResult<UiTokenAmount>>> = use_context();
    if let Some(token) = token.cloned() {
        if let Some(balance) = token_balances.get(&token.mint) {
            *balance
        } else {
            use_token_balance_resource(token.mint)
        }
    } else {
        use_resource(move || async move { Err(GatewayError::Unknown.into()) })
    }
}

async fn get_token_balance(pubkey: Pubkey, mint: Pubkey) -> GatewayResult<UiTokenAmount> {
    if mint == Token::sol().mint {
        use_gateway()
            .rpc
            .get_balance(&pubkey)
            .await
            .map(|lamports| {
                let sol = lamports_to_sol(lamports);
                UiTokenAmount {
                    ui_amount: Some(sol),
                    decimals: 8,
                    amount: format!("{}", lamports).to_owned(),
                    ui_amount_string: format!("{}", sol).to_owned(),
                }
            })
            .map_err(GatewayError::from)
    } else {
        use_gateway()
            .rpc
            .get_token_balance(&pubkey, &mint)
            .await
            .map_err(GatewayError::from)
    }
}

// TODO: Shouldn't we be using use_token_balance instead of get_token_balance?
pub fn use_token_balances_for_liquidity_pair(
    liquidity_pair: Resource<GatewayResult<LiquidityPair>>,
) -> (
    Resource<GatewayResult<UiTokenAmount>>,
    Resource<GatewayResult<UiTokenAmount>>,
) {
    let wallet = use_wallet();

    let token_a_balance = use_resource(move || async move {
        if let Some(Ok(liquidity_pair)) = liquidity_pair.read().as_ref() {
            match *wallet.read() {
                Wallet::Disconnected => Err(GatewayError::AccountNotFound.into()),
                Wallet::Connected(authority) => {
                    get_token_balance(authority, liquidity_pair.token_a.mint).await
                }
            }
        } else {
            Err(GatewayError::Unknown)
        }
    });

    let token_b_balance = use_resource(move || async move {
        if let Some(Ok(liquidity_pair)) = liquidity_pair.read().as_ref() {
            match *wallet.read() {
                Wallet::Disconnected => Err(GatewayError::AccountNotFound.into()),
                Wallet::Connected(authority) => {
                    get_token_balance(authority, liquidity_pair.token_b.mint).await
                }
            }
        } else {
            Err(GatewayError::Unknown)
        }
    });

    (token_a_balance, token_b_balance)
}

pub fn use_sol_balance() -> Resource<GatewayResult<UiTokenAmount>> {
    return use_token_balance(Token::sol().mint);
}

pub fn use_sol_balance_wss() -> Signal<GatewayResult<UiTokenAmount>> {
    // Update callback for SOL balance
    let update_callback = move |msg: &FromWssMsg, current_sub_id: u64| {
        if let FromWssMsg::Notif(notif) = msg {
            if notif.subscription.eq(&current_sub_id) {
                let lamports = notif.result.value.lamports;
                let sol = lamports_to_sol(lamports);
                let token_amount = UiTokenAmount {
                    ui_amount: Some(sol),
                    decimals: 8,
                    amount: format!("{}", lamports),
                    ui_amount_string: format!("{}", sol),
                };
                return Some(token_amount);
            }
        }
        None
    };

    use_balance_wss(Token::sol().mint, update_callback)
}

/// Generic function to handle WebSocket balance subscriptions
///
/// This function provides a unified API for subscribing to token balance updates via WebSocket.
/// It handles the initial balance fetch, subscription management, and updates.
///
/// # Parameters
/// * `mint` - The mint address of the token to track
/// * `update_callback` - Callback that processes WebSocket messages and returns token amounts when relevant
///
/// # Returns
/// A Signal containing the current token balance or an error
pub fn use_balance_wss<U>(mint: Pubkey, update_callback: U) -> Signal<GatewayResult<UiTokenAmount>>
where
    U: Fn(&FromWssMsg, u64) -> Option<UiTokenAmount> + 'static,
{
    let wallet = use_wallet();
    let (from_wss, to_wss) = use_wss();
    let mut sub_id = use_signal(|| 0);
    let sub_request_id = use_memo(move || AccountSubscribeGateway::request_id());
    let mut balance: Signal<GatewayResult<UiTokenAmount>> =
        use_signal(|| Err(GatewayError::AccountNotFound));

    // Fetch initial balance if wallet is connected
    let mut balance_clone = balance.clone();
    spawn(async move {
        match *wallet.read() {
            Wallet::Disconnected => balance_clone.set(Err(GatewayError::AccountNotFound)),
            Wallet::Connected(pubkey) => match get_token_balance(pubkey, mint).await {
                Ok(b) => balance_clone.set(Ok(b)),
                Err(err) => {
                    log::error!("Failed to fetch initial balance: {:?}", err);
                    balance_clone.set(Err(err));
                }
            },
        }
    });

    // Handle subscription ID tracking
    use_effect(move || {
        let msg = from_wss.cloned();

        // Track subscription ID
        if let FromWssMsg::Subscription(rid, sid) = msg {
            if sub_request_id.eq(&rid) {
                sub_id.set(sid);
            }
        }
    });

    // Handle balance updates separately
    use_effect(move || {
        let msg = from_wss.cloned();
        let current_sub_id = *sub_id.read();

        // Only process notification messages
        if let FromWssMsg::Notif(_) = &msg {
            // Update balance when notification is received
            if let Some(token_amount) = update_callback(&msg, current_sub_id) {
                balance.set(Ok(token_amount));
            }
        }
    });

    // Subscribe to account updates when wallet is connected
    use_effect(move || {
        if let Wallet::Connected(pubkey) = *wallet.read() {
            to_wss.send(ToWssMsg::Subscribe(sub_request_id(), pubkey));
        }
    });

    // Unsubscribe when component is dropped
    use_drop(move || {
        let current_sub_id = *sub_id.read();
        if current_sub_id > 0 {
            to_wss.send(ToWssMsg::Unsubscribe(current_sub_id));
        }
    });

    balance
}

pub fn use_ore_balance() -> Resource<GatewayResult<UiTokenAmount>> {
    let wallet_status = use_wallet();
    use_resource(move || async move {
        match *wallet_status.read() {
            Wallet::Disconnected => Err(GatewayError::AccountNotFound.into()),
            Wallet::Connected(pubkey) => use_gateway().rpc.get_ore_balance(&pubkey).await,
        }
    })
}

pub fn use_ore_supply() -> Resource<GatewayResult<UiTokenAmount>> {
    use_resource(move || async move {
        use_gateway()
            .rpc
            .get_token_supply(&ore_api::consts::MINT_ADDRESS)
            .await
            .map_err(GatewayError::from)
    })
}

pub fn use_ore_market_cap() -> Resource<GatewayResult<f64>> {
    let supply = use_ore_supply();
    let price = use_ore_price();

    use_resource(move || async move {
        let Some(Ok(supply)) = supply.cloned() else {
            return Err(GatewayError::Unknown);
        };
        let Some(OrePrice(price)) = price.cloned() else {
            return Err(GatewayError::Unknown);
        };
        let Some(supply) = supply.ui_amount else {
            return Err(GatewayError::Unknown);
        };
        let market_cap = supply * price;
        Ok(market_cap)
    })
}
