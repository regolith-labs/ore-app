use cached::proc_macro::cached;
use solana_client_wasm::solana_sdk::{pubkey::Pubkey, signer::Signer};

use crate::gateway::{get_keypair, send_and_confirm, wasm_client};

use super::GatewayError;

#[cached]
pub fn ore_token_account_address(pubkey: Pubkey) -> Pubkey {
    solana_extra_wasm::program::spl_associated_token_account::get_associated_token_address(
        &pubkey,
        &ore::MINT_ADDRESS,
    )
}

// TODO Result type
pub async fn create_ore_token_account() -> Pubkey {
    // Build instructions.
    let keypair = get_keypair();
    let client = wasm_client();

    // Check if account already exists.
    let token_account_address = ore_token_account_address(keypair.pubkey());
    match client.get_token_account(&token_account_address).await {
        Ok(token_account) => {
            if token_account.is_some() {
                return token_account_address;
            }
        }
        Err(err) => {
            if let GatewayError::NotFound = GatewayError::from(err) {
                // Do nothing
            }
        }
    }

    // Sign and send transaction.
    let ix =
        solana_extra_wasm::program::spl_associated_token_account::instruction::create_associated_token_account(
            &keypair.pubkey(),
            &keypair.pubkey(),
            &ore::MINT_ADDRESS,
            &solana_extra_wasm::program::spl_token::id(),
        );
    send_and_confirm(&client, &[ix]).await;

    // Return token account address
    token_account_address
}
