/*
Transaction builder
- unix timestamp
inAmount: u64,: total amount to sell
inAmountPerCycle: ttal amount to slel per buy
cycleFrequency: i64: number of seconds between peridic buys
startAt: i64: unix timestamp of when to start




pub fn open_dca_v2(
    ctx: Context<OpenDcaOnBehalf>,
    application_idx: u64,
    in_amount: u64,
    in_amount_per_cycle: u64,
    cycle_frequency: i64,
    min_out_amount: Option<u64>,
    max_out_amount: Option<u64>,
    start_at: Option<i64>,
) -> Result<()> {



 const [dca] = await PublicKey.findProgramAddressSync(
  [
    Buffer.from("dca"),
    userPubKey.toBuffer(),
    inTokenPubKey.toBuffer(),
    outTokenPubKey.toBuffer(),
    new BN(parseInt((Date.now() / 1000).toString())).toArrayLike(
      Buffer,
      "le",
      8
    ),
  ],
  new PublicKey("DCA265Vj8a9CEuX1eb1LWRnDT7uK6q1xMipnNyatn23M")
);
*/

use std::str::FromStr;

use crate::config::Token;
use crate::hooks::{use_gateway, use_wallet, GetPubkey};
use crate::{
    components::TokenInputError,
    gateway::{solana::SolanaGateway, GatewayError, GatewayResult, UiTokenAmount},
    solana::spl_associated_token_account::{
        get_associated_token_address, get_or_create_associated_token_address,
    },
};
use dioxus::prelude::*;
use jupiter_dca_sdk::{
    accounts::Dca,
    instructions::{OpenDcaV2, OpenDcaV2InstructionArgs},
};
use solana_sdk::{
    get_associated_token_address, get_or_create_associated_token_address, pubkey::Pubkey,
    transaction::VersionedTransaction,
};

pub fn use_recurring_transaction(
    sell_token: Signal<Option<Token>>,
    buy_token: Signal<Option<Token>>,
    sell_token_balance: Resource<GatewayResult<UiTokenAmount>>,
    sell_amount: Signal<u64>,
    sell_amount_count: Signal<u64>,
    sell_frequency: Signal<i64>,
    mut err: Signal<Option<TokenInputError>>,
) -> Resource<GatewayResult<VersionedTransaction>> {
    let wallet = use_wallet();

    use_resource(move || async move {
        let pubkey = wallet.pubkey()?;

        let Some(sell_token) = sell_token.read().clone() else {
            return Err(GatewayError::Unknown);
        };

        let Some(buy_token) = buy_token.read().clone() else {
            return Err(GatewayError::Unknown);
        };

        let Some(Ok(sell_token_balance)) = sell_token_balance.read().clone() else {
            err.set(Some(TokenInputError::InsufficientBalance(sell_token)));
            return Err(GatewayError::Unknown);
        };

        // Check if user's current balance is sufficient to cover the total sell amount
        let sell_token_balance_u64 = sell_token_balance
            .amount
            .parse::<u64>()
            .map_err(|_| GatewayError::Unknown)?;

        if sell_token_balance_u64 < sell_amount.cloned() {
            err.set(Some(TokenInputError::InsufficientBalance(sell_token)));
            return Err(GatewayError::Unknown);
        }

        // Rpc
        let gateway = use_gateway();

        // Unix timestamp
        let clock = gateway.rpc.get_clock().await?;
        let current_time = clock.unix_timestamp;
        let timestamp = current_time.to_le_bytes();

        // Jup DCA Program ID
        let dca_program_id =
            Pubkey::from_str("DCA265Vj8a9CEuX1eb1LWRnDT7uK6q1xMipnNyatn23M").unwrap();

        // Get DCA PDA for current user
        let dca_pda = Pubkey::find_program_address(
            &[
                b"dca",
                sell_token.mint.as_ref(),
                buy_token.mint.as_ref(),
                &timestamp,
            ],
            &dca_program_id,
        )
        .0;

        // let dca = Dca::new(
        //     dca_pda,
        //     sell_token.mint,
        //     buy_token.mint,
        //     current_time,
        //     sell_amount,
        //     sell_amount_count,
        //     sell_frequency,
        // );

        // // Get ata for the sell token
        // let sell_token_ata = get_associated_token_address(&sell_token.mint, &pubkey);

        // // Create ata for the sell token based on PDA
        // let in_ata = get_or_create_associated_token_address(&pubkey, &sell_token.mint);

        // // Create ataa for the buy token based on PDA

        // let mut ixs = vec![];
        // let dca_ix_args = OpenDcaV2InstructionArgs {
        //   dca: dca_pda,
        //   user: pubkey,
        //   payer: pubkey,
        //   input_mint: sell_token.mint,
        //   output_mint: buy_token.mint,
        //   system_program: Pubkey::from_str("11111111111111111111111111111111").unwrap(),
        //   token_program: Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap(),
        //   associated_token_program: Pubkey::from_str("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL").unwrap(),
        //   event_authority: Pubkey::from_str("Cspp27eGUDMXxPEdhmEXFVRn6Lt1L7xJyALF3nmnWoBj").unwrap(),
        //   program: Pubkey::from_str("DCA265Vj8a9CEuX1eb1LWRnDT7uK6q1xMipnNyatn23M").unwrap(),
        //   // pub dca: solana_program::pubkey::Pubkey,

        //   // pub user: solana_program::pubkey::Pubkey,

        //   // pub payer: solana_program::pubkey::Pubkey,

        //   // pub input_mint: solana_program::pubkey::Pubkey,

        //   // pub output_mint: solana_program::pubkey::Pubkey,

        //   // pub user_ata: solana_program::pubkey::Pubkey,

        //   // pub in_ata: solana_program::pubkey::Pubkey,

        //   // pub out_ata: solana_program::pubkey::Pubkey,

        //   // pub system_program: solana_program::pubkey::Pubkey,

        //   // pub token_program: solana_program::pubkey::Pubkey,

        //   // pub associated_token_program: solana_program::pubkey::Pubkey,

        //   // pub event_authority: solana_program::pubkey::Pubkey,

        //   // pub program: solana_program::pubkey::Pubkey,ata:

        // }

        // let mut ixs = vec![];

        // userAta will be the ata for the token that is being sold
        // inAta will be the DCA PDA's ata for the token to sell
        // getAssociatedTokenAddressSync(inputMint, dcaPubKey, true)
        // outAta will be the DCA PDA's for the token to buy
        // getAssociatedTokenAddressSync(outputMint, dcaPubKey, true)
        Ok(VersionedTransaction::default())
    })
}
