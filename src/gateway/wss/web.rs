use async_trait::async_trait;
use futures_util::{SinkExt, StreamExt};
use gloo_net::websocket::futures::WebSocket;
use gloo_net::websocket::Message as GlooMessage;
use serde_json;

use super::{
    AccountNotificationEnvelope, AccountSubscribe, AccountSubscribeConfig,
    AccountSubscribeResponse, JsonRpcRequest, JsonRpcResponse, JsonRpcResponseWithError,
    SubscriptionError,
};

/// A WebSocket client for account subscriptions on Solana RPC using gloo‑net.
pub struct AccountSubscribeWeb {
    writer: futures_util::stream::SplitSink<WebSocket, GlooMessage>,
    reader: futures_util::stream::SplitStream<WebSocket>,
}

impl AccountSubscribeWeb {
    /// Connects to the specified WebSocket URL and returns a new client.
    pub async fn connect(url: &str) -> Result<Self, SubscriptionError> {
        let ws = WebSocket::open(url)
            .map_err(|e| SubscriptionError::ConnectionError(format!("{:?}", e)))?;
        let (writer, reader) = ws.split();
        Ok(Self { writer, reader })
    }
}

#[cfg(feature = "web")]
#[async_trait(?Send)]
impl AccountSubscribe for AccountSubscribeWeb {
    type SubscriptionId = u64;

    async fn subscribe(
        &mut self,
        account: &str,
        config: AccountSubscribeConfig,
    ) -> Result<Self::SubscriptionId, SubscriptionError> {
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: 1,
            method: "accountSubscribe".to_string(),
            params: (account.to_string(), config),
        };

        let req_json = serde_json::to_string(&request)
            .map_err(|e| SubscriptionError::ParseError(e.to_string()))?;
        self.writer
            .send(GlooMessage::Text(req_json))
            .await
            .map_err(|e| SubscriptionError::Other(e.to_string()))?;

        while let Some(msg) = self.reader.next().await {
            match msg {
                Ok(GlooMessage::Text(text)) => {
                    // Try parsing a successful response.
                    if let Ok(resp) =
                        serde_json::from_str::<JsonRpcResponse<AccountSubscribeResponse>>(&text)
                    {
                        if resp.id == 1 {
                            return Ok(resp.result);
                        }
                    }
                    // Try parsing an error response.
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
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: 2, // Different id for unsubscribe.
            method: "accountUnsubscribe".to_string(),
            params: (subscription,),
        };

        let req_json = serde_json::to_string(&request)
            .map_err(|e| SubscriptionError::ParseError(e.to_string()))?;
        self.writer
            .send(GlooMessage::Text(req_json))
            .await
            .map_err(|e| SubscriptionError::Other(e.to_string()))?;

        while let Some(msg) = self.reader.next().await {
            match msg {
                Ok(GlooMessage::Text(text)) => {
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
        while let Some(msg) = self.reader.next().await {
            match msg {
                Ok(GlooMessage::Text(text)) => {
                    if let Ok(notification) =
                        serde_json::from_str::<AccountNotificationEnvelope>(&text)
                    {
                        return Some(Ok(notification));
                    }
                }
                Ok(_) => continue,
                Err(e) => return Some(Err(SubscriptionError::ConnectionError(e.to_string()))),
            }
        }
        None
    }
}
