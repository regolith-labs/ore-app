use async_trait::async_trait;
use fastrand;
use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use serde::Serialize;
use serde_json;
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{self, Message},
    MaybeTlsStream, WebSocketStream,
};
use tungstenite::client::IntoClientRequest;

use crate::gateway::WSS_URL;

use super::{
    AccountNotificationEnvelope, AccountSubscribe, AccountSubscribeConfig, JsonRpcRequest,
    JsonRpcResponse, JsonRpcResponseWithError, SubscriptionError,
};

/// WebSocket client for account subscriptions on Solana RPC
pub struct AccountSubscribeGateway {
    writer: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    reader: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

impl AccountSubscribeGateway {
    async fn send_request<T: Serialize>(
        &mut self,
        request: &JsonRpcRequest<T>,
    ) -> Result<(), SubscriptionError> {
        let req_json = serde_json::to_string(request)
            .map_err(|e| SubscriptionError::ParseError(e.to_string()))?;
        self.writer
            .send(Message::Text(req_json.into()))
            .await
            .map_err(|e: tokio_tungstenite::tungstenite::Error| {
                SubscriptionError::Other(e.to_string())
            })
    }

    async fn handle_response<R: serde::de::DeserializeOwned>(
        &mut self,
        request_id: u64,
    ) -> Result<R, SubscriptionError> {
        while let Some(msg) = self.reader.next().await {
            match msg {
                Ok(Message::Text(text)) => {
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

#[async_trait]
impl AccountSubscribe for AccountSubscribeGateway {
    type SubscriptionId = u64;
    async fn connect() -> Result<Self, SubscriptionError> {
        let url_parsed = WSS_URL
            .into_client_request()
            .map_err(|e| SubscriptionError::Other(e.to_string()))?;
        let (ws_stream, _) = connect_async(url_parsed)
            .await
            .map_err(|e| SubscriptionError::ConnectionError(e.to_string()))?;
        let (writer, reader) = ws_stream.split();
        Ok(Self { writer, reader })
    }

    async fn subscribe(
        &mut self,
        account: &str,
        request_id: u64,
    ) -> Result<Self::SubscriptionId, SubscriptionError> {
        let config = AccountSubscribeConfig {
            encoding: "base64".to_string(),
            commitment: "confirmed".to_string(),
        };
        log::info!("request id: {}", request_id);
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: request_id,
            method: "accountSubscribe".to_string(),
            params: (account.to_string(), config),
        };
        self.send_request(&request).await?;
        self.handle_response(request_id).await
    }

    async fn unsubscribe(
        &mut self,
        subscription: Self::SubscriptionId,
    ) -> Result<(), SubscriptionError> {
        // Generate a random ID for this request
        let request_id = fastrand::u64(..);
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: request_id,
            method: "accountUnsubscribe".to_string(),
            params: (subscription,),
        };
        self.send_request(&request).await?;
        let result = self.handle_response::<bool>(request_id).await?;
        if result {
            log::info!("unsubcribed!");
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
                Ok(Message::Text(text)) => {
                    log::info!("text: {:?}", text);
                    match serde_json::from_str::<AccountNotificationEnvelope>(&text) {
                        Ok(notification) => {
                            if notification.method == "accountNotification" {
                                log::info!("notif id: {:?}", notification.params.subscription);
                                return Ok(notification);
                            } else {
                                log::info!("Ignoring non-account notification: {:?}", notification);
                                continue;
                            }
                        }
                        Err(e) => {
                            log::error!("Failed to parse notification: {}, text: {}", e, text);
                            continue;
                        }
                    }
                }
                Ok(msg) => {
                    log::info!("{:?}", msg);
                    continue;
                }
                Err(e) => return Err(SubscriptionError::ConnectionError(e.to_string())),
            }
        }
        Err(SubscriptionError::Other(
            "WebSocket stream ended".to_string(),
        ))
    }
}
