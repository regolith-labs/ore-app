// #[cfg(not(feature = "web"))]
mod native;
#[cfg(feature = "web")]
mod web;

#[cfg(not(feature = "web"))]
pub use native::*;
#[cfg(feature = "web")]
pub use web::*;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fmt::Debug;

////////////////////////////////////////////////////////////////////////////
// JSON‑RPC Types
////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Serialize, Deserialize)]
pub(super) struct JsonRpcRequest<T> {
    pub jsonrpc: String,
    pub id: u64,
    pub method: String,
    pub params: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct JsonRpcResponse<T> {
    pub jsonrpc: String,
    pub id: u64,
    pub result: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct JsonRpcError {
    pub code: i64,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct JsonRpcResponseWithError<T> {
    pub jsonrpc: String,
    pub id: u64,
    pub result: Option<T>,
    pub error: Option<JsonRpcError>,
}

////////////////////////////////////////////////////////////////////////////
// Account Subscription Request/Response Types
////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Serialize, Deserialize)]
pub(super) struct AccountSubscribeConfig {
    pub encoding: String,
    pub commitment: String,
}

pub(super) type AccountSubscribeResponse = u64;

////////////////////////////////////////////////////////////////////////////
// Account Notification Types
////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Serialize, Deserialize, Clone)]
pub(super) struct AccountContext {
    pub slot: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(super) struct AccountNotification {
    pub data: Vec<String>,
    pub executable: bool,
    pub lamports: u64,
    pub owner: String,
    #[serde(rename = "rentEpoch")]
    pub rent_epoch: u64,
    pub space: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(super) struct AccountNotificationResult {
    pub context: AccountContext,
    pub value: AccountNotification,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountNotificationEnvelope {
    pub jsonrpc: String,
    pub method: String,
    pub params: AccountNotificationParams,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountNotificationParams {
    pub result: AccountNotificationResult,
    pub subscription: u64,
}

////////////////////////////////////////////////////////////////////////////
// Subscription Trait
////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub enum SubscriptionError {
    ConnectionError(String),
    ParseError(String),
    RpcError(String),
    Other(String),
}

// TODO: reconnect logic
#[cfg_attr(not(feature = "web"), async_trait)]
#[cfg_attr(feature = "web", async_trait(?Send))]
pub trait AccountSubscribe: Sized {
    type SubscriptionId: Copy + Debug;
    async fn connect() -> Result<Self, SubscriptionError>;
    async fn subscribe(&mut self, account: &str)
        -> Result<Self::SubscriptionId, SubscriptionError>;
    async fn unsubscribe(
        &mut self,
        subscription: Self::SubscriptionId,
    ) -> Result<(), SubscriptionError>;
    async fn next_notification(&mut self)
        -> Result<AccountNotificationEnvelope, SubscriptionError>;
}
