use dioxus::prelude::*;
use solana_sdk::{native_token::lamports_to_sol, pubkey::Pubkey};
use std::collections::HashMap;

use crate::{
    config::{Token, LISTED_TOKENS},
    gateway::{spl::SplGateway, GatewayError, GatewayResult, Rpc, UiTokenAmount},
    utils::LiquidityPair,
};

use crate::hooks::{use_gateway, use_wallet, Wallet};

use super::{use_ore_price, OrePrice};

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

pub fn _use_sol_balance() -> Resource<GatewayResult<UiTokenAmount>> {
    return use_token_balance(Token::sol().mint);
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
