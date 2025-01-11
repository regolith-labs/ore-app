use async_std::future::TimeoutError;
use solana_client_wasm::ClientError;
use steel::ProgramError;

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

impl From<ClientError> for GatewayError {
    fn from(value: ClientError) -> Self {
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

impl From<ProgramError> for GatewayError {
    fn from(value: ProgramError) -> Self {
        log::error!("err: {}", value);
        GatewayError::ProgramBuilderFailed
    }
}
