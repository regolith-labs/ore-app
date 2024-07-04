use solana_client_wasm::{
    solana_sdk::{program_error::ProgramError, signature::ParseSignatureError},
    ClientError,
};

pub type GatewayResult<T> = Result<T, GatewayError>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GatewayError {
    FailedDeserialization,
    FailedAta,
    FailedRegister,
    TransactionTimeout,
    NetworkUnavailable,
    AccountNotFound,
    // SimulationFailed,
    RequestFailed,
    ProgramBuilderFailed,
    WalletAdapterDisconnected,
    SerdeJsonFailed,
    ParseSignatureFailed,
    Unknown,
}

impl From<reqwest::Error> for GatewayError {
    fn from(_value: reqwest::Error) -> Self {
        GatewayError::RequestFailed
    }
}

impl From<ClientError> for GatewayError {
    fn from(value: ClientError) -> Self {
        let msg = value.to_string();
        if msg.starts_with("Client error: Invalid param: could not find account")
            || msg.starts_with("Client error: AccountNotFound: ")
        {
            GatewayError::AccountNotFound
        } else if msg.starts_with("Client error: error sending request") {
            GatewayError::NetworkUnavailable
        } else {
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

impl From<serde_json::Error> for GatewayError {
    fn from(value: serde_json::Error) -> Self {
        log::error!("err: {}", value);
        GatewayError::SerdeJsonFailed
    }
}

impl From<ParseSignatureError> for GatewayError {
    fn from(value: ParseSignatureError) -> Self {
        log::error!("err: {}", value);
        GatewayError::ParseSignatureFailed
    }
}
