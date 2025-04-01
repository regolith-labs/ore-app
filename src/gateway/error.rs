use async_std::future::TimeoutError;
use steel::ProgramError;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum GatewayError {
    Anyhow,
    FailedDeserialization,
    TransactionTimeout,
    NetworkUnavailable,
    AccountNotFound,
    RetryFailed,
    TimeoutError,
    SignatureFailed,
    SerdeJson,
    RequestFailed,
    ProgramBuilderFailed,
    WalletDisconnected,
    JupSwapError,
    ParseTokenStringAmmount,
    KeyringElse,
    KeyringNoEntry,
    BincodeSerialize,
    BincodeDeserialize,
    Unknown,
    WithMessage(String),
}

impl GatewayError {
    pub fn with_message<S: Into<String>>(msg: S) -> Self {
        Self::WithMessage(msg.into())
    }

    pub fn get_message(&self) -> String {
        match self {
            Self::WithMessage(msg) => msg.clone(),
            Self::WalletDisconnected => "Wallet is not connected".to_string(),
            Self::AccountNotFound => "Account not found".to_string(),
            Self::NetworkUnavailable => "Network is unavailable".to_string(),
            Self::TransactionTimeout => "Transaction timed out".to_string(),
            Self::RequestFailed => "Request failed".to_string(),
            Self::JupSwapError => "Swap error occurred".to_string(),
            Self::ProgramBuilderFailed => "Failed to build program".to_string(),
            _ => format!("{:?}", self),
        }
    }
}

impl From<anyhow::Error> for GatewayError {
    fn from(value: anyhow::Error) -> Self {
        log::error!("{:?}", value);
        Self::Anyhow
    }
}

impl From<serde_json::Error> for GatewayError {
    fn from(value: serde_json::Error) -> Self {
        log::error!("{:?}", value);
        Self::SerdeJson
    }
}

impl From<solana_sdk::message::CompileError> for GatewayError {
    fn from(value: solana_sdk::message::CompileError) -> Self {
        log::error!("{:?}", value);
        Self::SignatureFailed
    }
}

impl From<solana_sdk::signer::SignerError> for GatewayError {
    fn from(value: solana_sdk::signer::SignerError) -> Self {
        log::error!("{:?}", value);
        Self::SignatureFailed
    }
}

impl From<keyring::Error> for GatewayError {
    fn from(value: keyring::Error) -> Self {
        log::error!("{:?}", value);
        if let keyring::Error::NoEntry = value {
            Self::KeyringNoEntry
        } else {
            Self::KeyringElse
        }
    }
}

impl From<std::io::Error> for GatewayError {
    fn from(value: std::io::Error) -> Self {
        log::error!("{:?}", value);
        GatewayError::FailedDeserialization
    }
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
    fn from(value: reqwest::Error) -> Self {
        log::error!("{:?}", value);
        GatewayError::RequestFailed
    }
}

impl From<TimeoutError> for GatewayError {
    fn from(value: TimeoutError) -> Self {
        log::error!("{:?}", value);
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

impl From<crate::gateway::wss::SubscriptionError> for GatewayError {
    fn from(value: crate::gateway::wss::SubscriptionError) -> Self {
        log::error!("{:?}", value);
        match value {
            crate::gateway::wss::SubscriptionError::ConnectionError(_) => {
                GatewayError::NetworkUnavailable
            }
            crate::gateway::wss::SubscriptionError::ParseError(_) => {
                GatewayError::FailedDeserialization
            }
            crate::gateway::wss::SubscriptionError::RpcError(_) => GatewayError::RequestFailed,
            crate::gateway::wss::SubscriptionError::Other(_) => GatewayError::Unknown,
        }
    }
}
