use async_trait::async_trait;
use futures_util::{SinkExt, StreamExt};
use gloo_net::websocket::futures::WebSocket;
use gloo_net::websocket::Message as GlooMessage;
use serde::{de::DeserializeOwned, Serialize};
use serde_json;

use crate::gateway::WSS_URL;

use super::{
    AccountNotificationEnvelope, AccountSubscribe, AccountSubscribeConfig,
    AccountSubscribeResponse, JsonRpcRequest, JsonRpcResponse, JsonRpcResponseWithError,
    SubscriptionError,
};

/// A WebSocket client for account subscriptions on Solana RPC using gloo‑net.
pub struct AccountSubscribeGateway {
    writer: futures_util::stream::SplitSink<WebSocket, GlooMessage>,
    reader: futures_util::stream::SplitStream<WebSocket>,
}

impl AccountSubscribeGateway {
    /// Sends a JSON-RPC request over the WebSocket connection
    async fn send_request<T: Serialize>(
        &mut self,
        request: &JsonRpcRequest<T>,
    ) -> Result<(), SubscriptionError> {
        let req_json = serde_json::to_string(&request)
            .map_err(|e| SubscriptionError::ParseError(e.to_string()))?;
        self.writer
            .send(GlooMessage::Text(req_json))
            .await
            .map_err(|e| SubscriptionError::Other(e.to_string()))
    }

    /// Handles the response stream and parses the response for a specific request ID
    async fn handle_response<R: serde::de::DeserializeOwned>(
        &mut self,
        request_id: u64,
    ) -> Result<R, SubscriptionError> {
        while let Some(msg) = self.reader.next().await {
            match msg {
                Ok(GlooMessage::Text(text)) => {
                    // Try parsing a successful response
                    if let Ok(resp) = serde_json::from_str::<JsonRpcResponse<R>>(&text) {
                        if resp.id == request_id {
                            return Ok(resp.result);
                        }
                    }
                    // Try parsing an error response
                    if let Ok(resp_err) = serde_json::from_str::<JsonRpcResponseWithError<R>>(&text)
                    {
                        if resp_err.id == request_id {
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
}

#[cfg(feature = "web")]
#[async_trait(?Send)]
impl AccountSubscribe for AccountSubscribeGateway {
    type SubscriptionId = u64;
    /// Connects to the specified WebSocket URL and returns a new client.
    async fn connect() -> Result<Self, SubscriptionError> {
        let ws = WebSocket::open(WSS_URL)
            .map_err(|e| SubscriptionError::ConnectionError(format!("{:?}", e)))?;
        let (writer, reader) = ws.split();
        Ok(Self { writer, reader })
    }

    async fn subscribe(
        &mut self,
        account: &str,
    ) -> Result<Self::SubscriptionId, SubscriptionError> {
        let config = AccountSubscribeConfig {
            encoding: "jsonParsed".to_string(),
            commitment: "finalized".to_string(),
        };
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: 1,
            method: "accountSubscribe".to_string(),
            params: (account.to_string(), config),
        };

        self.send_request(&request).await?;
        self.handle_response(request.id).await
    }

    async fn unsubscribe(
        &mut self,
        subscription: Self::SubscriptionId,
    ) -> Result<(), SubscriptionError> {
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: 2,
            method: "accountUnsubscribe".to_string(),
            params: (subscription,),
        };

        self.send_request(&request).await?;
        let result = self.handle_response::<bool>(request.id).await?;
        if result {
            Ok(())
        } else {
            Err(SubscriptionError::RpcError(
                "Unsubscribe failed".to_string(),
            ))
        }
    }

    async fn next_notification(
        &mut self,
    ) -> Result<AccountNotificationEnvelope, SubscriptionError> {
        while let Some(msg) = self.reader.next().await {
            match msg {
                Ok(GlooMessage::Text(text)) => {
                    if let Ok(notification) =
                        serde_json::from_str::<AccountNotificationEnvelope>(&text)
                    {
                        return Ok(notification);
                    }
                }
                Ok(_) => continue,
                Err(e) => return Err(SubscriptionError::ConnectionError(e.to_string())),
            }
        }
        Err(SubscriptionError::Other(
            "WebSocket stream ended".to_string(),
        ))
    }
}
