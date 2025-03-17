use async_std::future::TimeoutError;
use steel::ProgramError;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GatewayError {
    FailedDeserialization,
    TransactionTimeout,
    NetworkUnavailable,
    AccountNotFound,
    RetryFailed,
    TimeoutError,
    SignatureFailed,
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
