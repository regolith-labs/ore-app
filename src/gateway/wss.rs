use async_trait::async_trait;
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fmt::Debug;
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{self, Message},
    MaybeTlsStream, WebSocketStream,
};

//
// JSON‑RPC Types
//

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcRequest<T> {
    pub jsonrpc: String,
    pub id: u64,
    pub method: String,
    pub params: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcResponse<T> {
    pub jsonrpc: String,
    pub id: u64,
    pub result: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i64,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

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

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountSubscribeConfig {
    pub encoding: String,
    pub commitment: String,
}

pub type AccountSubscribeParams = (String, AccountSubscribeConfig);
pub type AccountSubscribeResponse = u64;

//
// Account Notification Types
//

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountContext {
    pub slot: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountNotification {
    pub data: Vec<String>,
    pub executable: bool,
    pub lamports: u64,
    pub owner: String,
    #[serde(rename = "rentEpoch")]
    pub rent_epoch: u64,
    pub space: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountNotificationResult {
    pub context: AccountContext,
    pub value: AccountNotification,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountNotificationEnvelope {
    pub subscription: u64,
    pub result: AccountNotificationResult,
}

//
// Subscription Trait
//

#[derive(Debug)]
pub enum SubscriptionError {
    ConnectionError(String),
    ParseError(String),
    RpcError(String),
    Other(String),
}

#[async_trait]
pub trait AccountSubscribe {
    type SubscriptionId: Copy + Debug;

    async fn subscribe(
        &mut self,
        account: &str,
        config: AccountSubscribeConfig,
    ) -> Result<Self::SubscriptionId, SubscriptionError>;

    async fn unsubscribe(
        &mut self,
        subscription: Self::SubscriptionId,
    ) -> Result<(), SubscriptionError>;

    async fn next_notification(
        &mut self,
    ) -> Option<Result<AccountNotificationEnvelope, SubscriptionError>>;
}
