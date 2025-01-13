use async_std::future::TimeoutError;
use async_std::future::{timeout, Future};
use steel::ProgramError;

use crate::steel_app::time::Duration;

pub type GatewayResult<T> = Result<T, GatewayError>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GatewayError {
    FailedDeserialization,
    TransactionTimeout,
    NetworkUnavailable,
    AccountNotFound,
    RetryFailed,
    TimeoutError,
    SignatureDenied,
    RequestFailed,
    ProgramBuilderFailed,
    WalletAdapterDisconnected,
    JupSwapError,
    ParseTokenStringAmmount,
    Unknown,
}

impl From<std::num::ParseFloatError> for GatewayError {
    fn from(value: std::num::ParseFloatError) -> Self {
        log::error!("{:?}", value);
        GatewayError::ParseTokenStringAmmount
    }
}

impl From<jupiter_swap_api_client::ClientError> for GatewayError {
    fn from(value: jupiter_swap_api_client::ClientError) -> Self {
        log::error!("{:?}", value);
        GatewayError::JupSwapError
    }
}

impl From<reqwest::Error> for GatewayError {
    fn from(_value: reqwest::Error) -> Self {
        GatewayError::RequestFailed
    }
}

impl From<TimeoutError> for GatewayError {
    fn from(_value: TimeoutError) -> Self {
        GatewayError::TimeoutError
    }
}

#[cfg(feature = "web")]
impl From<solana_client_wasm::ClientError> for GatewayError {
    fn from(value: solana_client_wasm::ClientError) -> Self {
        let msg = value.to_string();
        if msg.starts_with("Client error: Invalid param: could not find account")
            || msg.starts_with("Client error: AccountNotFound: ")
            || msg.ends_with("not found.")
        {
            GatewayError::AccountNotFound
        } else if msg.starts_with("Client error: error sending request") {
            GatewayError::NetworkUnavailable
        } else {
            log::info!("Err: {:?}", msg);
            GatewayError::Unknown
        }
    }
}

#[cfg(feature = "desktop")]
impl From<solana_client::client_error::ClientError> for GatewayError {
    fn from(value: solana_client::client_error::ClientError) -> Self {
        log::error!("{:?}", value);
        GatewayError::RequestFailed
    }
}

impl From<ProgramError> for GatewayError {
    fn from(value: ProgramError) -> Self {
        log::error!("err: {}", value);
        GatewayError::ProgramBuilderFailed
    }
}

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
