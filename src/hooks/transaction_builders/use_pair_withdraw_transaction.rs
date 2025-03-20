use dioxus::prelude::*;
use ore_boost_api::state::Stake;
use solana_sdk::{
    address_lookup_table::{state::AddressLookupTable, AddressLookupTableAccount},
    compute_budget::ComputeBudgetInstruction,
    hash::Hash,
    message::{v0::Message, VersionedMessage},
    pubkey::Pubkey,
    signature::Signature,
    system_instruction::transfer,
    transaction::VersionedTransaction,
};

use crate::{
    components::TokenInputError,
    config::{BoostMeta, LpType, Token},
    gateway::{
        kamino::KaminoGateway, meteora::MeteoraGateway, GatewayError, GatewayResult, Rpc,
        UiTokenAmount,
    },
    hooks::{use_gateway, use_wallet, Wallet, APP_FEE_ACCOUNT, COMPUTE_UNIT_LIMIT},
    solana::{
        spl_associated_token_account::{
            get_associated_token_address, instruction::create_associated_token_account_idempotent,
        },
        spl_token::{
            self,
            instruction::{close_account, sync_native},
            ui_amount_to_amount,
        },
    },
    utils::LiquidityPair,
};

// Build pair deposit transaction
pub fn use_pair_withdraw_transaction(
    boost_meta: BoostMeta,
    liquidity_pair: Resource<GatewayResult<LiquidityPair>>,
    stake: Signal<GatewayResult<Stake>>,
    stake_a_balance: Resource<GatewayResult<UiTokenAmount>>,
    stake_b_balance: Resource<GatewayResult<UiTokenAmount>>,
    token_a_balance: Resource<GatewayResult<UiTokenAmount>>,
    token_b_balance: Resource<GatewayResult<UiTokenAmount>>,
    lp_balance: Resource<GatewayResult<UiTokenAmount>>,
    input_amount_a: Signal<String>,
    input_amount_b: Signal<String>,
    mut err: Signal<Option<TokenInputError>>,
    mut priority_fee: Signal<u64>,
) -> Resource<GatewayResult<VersionedTransaction>> {
    let wallet = use_wallet();
    use_resource(move || async move {
        // Reset error
        err.set(None);

        // Check if wallet is connected
        let Wallet::Connected(authority) = *wallet.read() else {
            return Err(GatewayError::WalletDisconnected);
        };

        // Parse input amounts
        let Ok(amount_a_f64) = input_amount_a.cloned().parse::<f64>() else {
            return Err(GatewayError::Unknown);
        };
        let Ok(amount_b_f64) = input_amount_b.cloned().parse::<f64>() else {
            return Err(GatewayError::Unknown);
        };
        if amount_a_f64 == 0f64 || amount_b_f64 == 0f64 {
            return Err(GatewayError::Unknown);
        }

        // Get resources
        let Ok(stake) = stake.cloned() else {
            return Err(GatewayError::Unknown);
        };
        let Some(Ok(liquidity_pair)) = liquidity_pair.cloned() else {
            return Err(GatewayError::Unknown);
        };
        let Some(Ok(stake_a_balance)) = stake_a_balance.cloned() else {
            return Err(GatewayError::Unknown);
        };
        let Some(Ok(_stake_b_balance)) = stake_b_balance.cloned() else {
            return Err(GatewayError::Unknown);
        };

        // Get amount u64
        let amount_a_u64 = ui_amount_to_amount(amount_a_f64, liquidity_pair.token_a.decimals);
        let amount_b_u64 = ui_amount_to_amount(amount_b_f64, liquidity_pair.token_b.decimals);
        let stake_a_balance_u64 = stake_a_balance.amount.parse::<u64>().unwrap();

        // Convert input amounts to LP shares
        let shares_amount = (stake.balance as u128)
            .checked_mul(amount_a_u64 as u128)
            .unwrap()
            .checked_div(stake_a_balance_u64 as u128)
            .unwrap() as u64;

        // Check if shares amount is sufficient
        if shares_amount > stake.balance {
            err.set(Some(TokenInputError::InsufficientBalance(
                liquidity_pair.token_a.clone(),
            )));
            return Err(GatewayError::Unknown);
        }

        // Aggregate instructions
        let mut ixs = vec![];

        // Set compute unit limit
        ixs.push(ComputeBudgetInstruction::set_compute_unit_limit(
            COMPUTE_UNIT_LIMIT,
        ));

        // Build ore boost withdraw instruction
        ixs.push(ore_boost_api::sdk::withdraw(
            authority,
            boost_meta.lp_mint,
            shares_amount,
        ));

        // Build sol ata
        let sol_mint = Token::sol().mint;
        let wsol_ata = get_associated_token_address(&authority, &sol_mint);
        let is_sol = liquidity_pair.token_a.is_sol() || liquidity_pair.token_b.is_sol();
        if is_sol {
            ixs.push(create_associated_token_account_idempotent(
                &authority,
                &authority,
                &sol_mint,
                &spl_token::ID,
            ));
            ixs.push(sync_native(&spl_token::ID, &wsol_ata).unwrap());
        }

        // Build other atas, if necessary
        if let Some(Ok(_token_a_balance)) = token_a_balance.cloned() {
            // Noop
        } else {
            ixs.push(create_associated_token_account_idempotent(
                &authority,
                &authority,
                &liquidity_pair.token_a.mint,
                &spl_token::ID,
            ));
        };
        if let Some(Ok(_token_b_balance)) = token_b_balance.cloned() {
            // Noop
        } else {
            ixs.push(create_associated_token_account_idempotent(
                &authority,
                &authority,
                &liquidity_pair.token_b.mint,
                &spl_token::ID,
            ));
        };
        if let Some(Ok(_lp_balance)) = lp_balance.cloned() {
            // Noop
        } else {
            ixs.push(create_associated_token_account_idempotent(
                &authority,
                &authority,
                &boost_meta.lp_mint,
                &spl_token::ID,
            ));
        };

        // Append kamino withdraw instructions
        let withdraw_ix = match boost_meta.lp_type {
            LpType::Kamino => {
                let Ok(ix) = use_gateway()
                    .build_kamino_withdraw_instruction(boost_meta.lp_id, shares_amount, authority)
                    .await
                else {
                    err.set(None);
                    return Err(GatewayError::Unknown);
                };
                ix
            }
            LpType::Meteora => {
                let Ok(ix) = use_gateway()
                    .build_meteora_withdraw_instruction(
                        boost_meta.lp_id,
                        shares_amount,
                        amount_a_u64,
                        amount_b_u64,
                        1,
                        authority,
                    )
                    .await
                else {
                    err.set(None);
                    return Err(GatewayError::Unknown);
                };
                ix
            }
        };
        ixs.push(withdraw_ix);

        // Close the wSOL ata
        if is_sol {
            ixs.push(
                close_account(
                    &spl_token::ID,
                    &wsol_ata,
                    &authority,
                    &authority,
                    &[&authority],
                )
                .unwrap(),
            );
        }

        // Fetch lookup tables
        let mut luts = vec![];
        if let Some(lut) = boost_meta.lut {
            if let Ok(account_data) = use_gateway().rpc.get_account_data(&lut).await {
                if let Ok(address_lookup_table) = AddressLookupTable::deserialize(&account_data) {
                    let address_lookup_table_account = AddressLookupTableAccount {
                        key: lut,
                        addresses: address_lookup_table.addresses.to_vec(),
                    };
                    luts.push(address_lookup_table_account);
                }
            }
        }

        // Include ORE app fee
        let app_fee_account = Pubkey::from_str_const(APP_FEE_ACCOUNT);
        ixs.push(transfer(&authority, &app_fee_account, 5000));

        // Build initial transaction to estimate priority fee
        let tx = VersionedTransaction {
            signatures: vec![Signature::default()],
            message: VersionedMessage::V0(
                Message::try_compile(&authority, &ixs, &luts, Hash::default()).unwrap(),
            ),
        };

        // Get priority fee estimate
        let gateway = use_gateway();
        let dynamic_priority_fee = match gateway.get_recent_priority_fee_estimate(&tx).await {
            Ok(fee) => fee,
            Err(_) => {
                log::error!("Failed to fetch priority fee estimate");
                return Err(GatewayError::Unknown);
            }
        };

        // Add priority fee instruction
        ixs.insert(
            1,
            ComputeBudgetInstruction::set_compute_unit_price(dynamic_priority_fee),
        );

        // Calculate priority fee in lamports
        let adjusted_compute_unit_limit_u64: u64 = COMPUTE_UNIT_LIMIT.into();
        let dynamic_priority_fee_in_lamports =
            (dynamic_priority_fee * adjusted_compute_unit_limit_u64) / 1_000_000;

        // Set priority fee for UI
        priority_fee.set(dynamic_priority_fee_in_lamports);

        // Build final tx with priority fee
        let tx = VersionedTransaction {
            signatures: vec![Signature::default()],
            message: VersionedMessage::V0(
                Message::try_compile(&authority, &ixs, &luts, Hash::default()).unwrap(),
            ),
        };

        Ok(tx)
    })
}
