use async_trait::async_trait;
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fmt::Debug;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

//
// JSON‑RPC Types
//

/// A generic JSON‑RPC request envelope.
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcRequest<T> {
    pub jsonrpc: String, // typically "2.0"
    pub id: u64,
    pub method: String,
    pub params: T,
}

/// A generic JSON‑RPC response envelope.
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcResponse<T> {
    pub jsonrpc: String, // typically "2.0"
    pub id: u64,
    pub result: T,
}

/// A JSON‑RPC error object.
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i64,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

/// A JSON‑RPC response envelope that can include an error.
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcResponseWithError<T> {
    pub jsonrpc: String,
    pub id: u64,
    pub result: Option<T>,
    pub error: Option<JsonRpcError>,
}

//
// Account Subscription Request/Response Types
//

/// Configuration for an account subscription request.
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountSubscribeConfig {
    pub encoding: String,   // e.g., "jsonParsed"
    pub commitment: String, // e.g., "finalized"
}

/// The parameters for an account subscription request, serialized as an array.
/// The first element is the account public key, the second is the config.
pub type AccountSubscribeParams = (String, AccountSubscribeConfig);

/// The result for an account subscription request – a subscription ID.
pub type AccountSubscribeResponse = u64;

//
// Account Notification Types
//

/// The "context" field in the notification.
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountContext {
    pub slot: u64,
}

/// The "value" field holding the account data.
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountNotification {
    /// Typically an array where the first element is the account data (a base58‑encoded string)
    /// and the second is the encoding (e.g., "base58").
    pub data: Vec<String>,
    pub executable: bool,
    pub lamports: u64,
    pub owner: String,
    #[serde(rename = "rentEpoch")]
    pub rent_epoch: u64,
    /// Optionally, the allocated space.
    pub space: Option<u64>,
}

/// The result object combining context and account value.
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountNotificationResult {
    pub context: AccountContext,
    pub value: AccountNotification,
}

/// The full notification envelope for account updates.
/// This envelope includes the subscription ID.
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountNotificationEnvelope {
    pub subscription: u64,
    pub result: AccountNotificationResult,
}

//
// Subscription Trait
//

/// Errors that may occur during subscription operations.
#[derive(Debug)]
pub enum SubscriptionError {
    ConnectionError(String),
    ParseError(String),
    RpcError(String),
    Other(String),
}

/// A trait for subscribing to account updates via Solana’s RPC WebSocket API.
/// The implementor will be responsible for constructing/sending JSON‑RPC requests,
/// and parsing JSON‑RPC responses and notifications.
#[async_trait]
pub trait AccountSubscribe {
    /// The type used to represent a subscription ID (typically a number).
    type SubscriptionId: Copy + Debug;

    /// Subscribes to updates for the specified account.
    ///
    /// This method constructs a JSON‑RPC request with method "accountSubscribe",
    /// sends it over the WebSocket, and parses the JSON‑RPC response to extract the subscription ID.
    async fn subscribe(
        &mut self,
        account: &str,
        config: AccountSubscribeConfig,
    ) -> Result<Self::SubscriptionId, SubscriptionError>;

    /// Unsubscribes from account notifications for the given subscription ID.
    ///
    /// This method constructs and sends a JSON‑RPC request with method "accountUnsubscribe"
    /// and expects a JSON‑RPC response indicating success.
    async fn unsubscribe(
        &mut self,
        subscription: Self::SubscriptionId,
    ) -> Result<(), SubscriptionError>;

    /// Waits for the next account notification.
    ///
    /// Incoming JSON‑RPC notifications (with method "accountNotification") are parsed
    /// into an `AccountNotificationEnvelope`. Returns `None` if the connection is closed.
    async fn next_notification(
        &mut self,
    ) -> Option<Result<AccountNotificationEnvelope, SubscriptionError>>;
}

//
// Implementation for tokio‑tungstenite
//

pub struct TungsteniteAccountSubscribeClient {
    writer: futures_util::stream::SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    reader: futures_util::stream::SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

impl TungsteniteAccountSubscribeClient {
    /// Connects to the specified WebSocket URL and returns a new client.
    pub async fn connect(url: &str) -> Result<Self, SubscriptionError> {
        let url_parsed = url
            .parse()
            .map_err(|e| SubscriptionError::Other(e.to_string()))?;
        let (ws_stream, _) = connect_async(url_parsed)
            .await
            .map_err(|e| SubscriptionError::ConnectionError(e.to_string()))?;
        let (writer, reader) = ws_stream.split();
        Ok(Self { writer, reader })
    }

    /// Helper method to send a JSON‑RPC request.
    async fn send_request<T: Serialize>(
        &mut self,
        request: &JsonRpcRequest<T>,
    ) -> Result<(), SubscriptionError> {
        let req_json = serde_json::to_string(request)
            .map_err(|e| SubscriptionError::ParseError(e.to_string()))?;
        self.writer
            .send(Message::Text(req_json))
            .await
            .map_err(|e| SubscriptionError::ConnectionError(e.to_string()))
    }
}

#[async_trait]
impl AccountSubscribe for TungsteniteAccountSubscribeClient {
    type SubscriptionId = u64;

    async fn subscribe(
        &mut self,
        account: &str,
        config: AccountSubscribeConfig,
    ) -> Result<Self::SubscriptionId, SubscriptionError> {
        // Build the JSON‑RPC request for "accountSubscribe".
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: 1, // In a real implementation, use an id generator.
            method: "accountSubscribe".to_string(),
            params: (account.to_string(), config),
        };

        self.send_request(&request).await?;

        // Wait for a JSON‑RPC response with id 1.
        while let Some(msg) = self.reader.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    // Try parsing a successful response.
                    if let Ok(resp) =
                        serde_json::from_str::<JsonRpcResponse<AccountSubscribeResponse>>(&text)
                    {
                        if resp.id == 1 {
                            return Ok(resp.result);
                        }
                    }
                    // Otherwise try parsing an error response.
                    if let Ok(resp_err) = serde_json::from_str::<
                        JsonRpcResponseWithError<AccountSubscribeResponse>,
                    >(&text)
                    {
                        if resp_err.id == 1 {
                            let err_msg = resp_err
                                .error
                                .map(|e| e.message)
                                .unwrap_or_else(|| "Unknown RPC error".to_string());
                            return Err(SubscriptionError::RpcError(err_msg));
                        }
                    }
                    // If the message is not the expected response, ignore it.
                }
                Ok(_) => continue,
                Err(e) => return Err(SubscriptionError::ConnectionError(e.to_string())),
            }
        }
        Err(SubscriptionError::Other(
            "WebSocket stream ended unexpectedly".to_string(),
        ))
    }

    async fn unsubscribe(
        &mut self,
        subscription: Self::SubscriptionId,
    ) -> Result<(), SubscriptionError> {
        // Build the JSON‑RPC request for "accountUnsubscribe".
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: 2, // Different id for unsubscribe.
            method: "accountUnsubscribe".to_string(),
            params: (subscription,),
        };

        self.send_request(&request).await?;

        // Wait for the unsubscribe response with id 2.
        while let Some(msg) = self.reader.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    if let Ok(resp) = serde_json::from_str::<JsonRpcResponse<bool>>(&text) {
                        if resp.id == 2 {
                            if resp.result {
                                return Ok(());
                            } else {
                                return Err(SubscriptionError::RpcError(
                                    "Unsubscribe failed".to_string(),
                                ));
                            }
                        }
                    }
                    if let Ok(resp_err) =
                        serde_json::from_str::<JsonRpcResponseWithError<bool>>(&text)
                    {
                        if resp_err.id == 2 {
                            let err_msg = resp_err
                                .error
                                .map(|e| e.message)
                                .unwrap_or_else(|| "Unknown RPC error".to_string());
                            return Err(SubscriptionError::RpcError(err_msg));
                        }
                    }
                }
                Ok(_) => continue,
                Err(e) => return Err(SubscriptionError::ConnectionError(e.to_string())),
            }
        }
        Err(SubscriptionError::Other(
            "WebSocket stream ended unexpectedly".to_string(),
        ))
    }

    async fn next_notification(
        &mut self,
    ) -> Option<Result<AccountNotificationEnvelope, SubscriptionError>> {
        // Wait for the next JSON‑RPC notification message.
        while let Some(msg) = self.reader.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    // We expect notifications to have method "accountNotification".
                    if let Ok(notification) =
                        serde_json::from_str::<AccountNotificationEnvelope>(&text)
                    {
                        return Some(Ok(notification));
                    }
                    // Otherwise ignore messages that aren’t notifications.
                }
                Ok(_) => continue,
                Err(e) => return Some(Err(SubscriptionError::ConnectionError(e.to_string()))),
            }
        }
        None
    }
}
