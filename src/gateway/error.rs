use solana_client_wasm::ClientError;

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
