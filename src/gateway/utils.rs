use async_std::future::{timeout, Future};
use serde::{Deserialize, Serialize};
use solana_sdk::transaction::TransactionError;

use crate::time::Duration;

use super::GatewayError;

pub type GatewayResult<T> = Result<T, GatewayError>;

pub async fn retry<F, Fut, T>(f: F) -> GatewayResult<T>
where
    F: Fn() -> Fut,
    Fut: Future<Output = GatewayResult<T>>,
{
    const MAX_RETRIES: u32 = 8;
    const INITIAL_BACKOFF: Duration = Duration::from_millis(200);
    const TIMEOUT: Duration = Duration::from_secs(8);
    let mut backoff = INITIAL_BACKOFF;
    for attempt in 0..MAX_RETRIES {
        match timeout(TIMEOUT, f()).await {
            Ok(Ok(result)) => return Ok(result),
            Ok(Err(e)) if attempt < MAX_RETRIES - 1 => {
                match e {
                    GatewayError::AccountNotFound => return Err(e),
                    _ => {
                        async_std::task::sleep(backoff).await;
                        backoff *= 2; // Exponential backoff
                    }
                }
            }
            Ok(Err(e)) => return Err(e),
            Err(_) if attempt < MAX_RETRIES - 1 => {
                async_std::task::sleep(backoff).await;
                backoff *= 2; // Exponential backoff
            }
            Err(_) => return Err(GatewayError::RetryFailed),
        }
    }

    Err(GatewayError::AccountNotFound)
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TransactionConfirmationStatus {
    Processed,
    Confirmed,
    Finalized,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UiTokenAmount {
    pub ui_amount: Option<f64>,
    pub decimals: u8,
    pub amount: StringAmount,
    pub ui_amount_string: StringDecimals,
}

pub type StringAmount = String;
pub type StringDecimals = String;


#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SimulateTransactionResponse {
    pub err: Option<TransactionError>,
    pub logs: Option<Vec<String>>,
    // pub accounts: Option<Vec<Option<UiAccount>>>,
    pub units_consumed: Option<u64>,
    // pub return_data: Option<UiTransactionReturnData>,
}
