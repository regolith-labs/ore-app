use async_trait::async_trait;
use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fmt::Debug;
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{self, Message},
    MaybeTlsStream, WebSocketStream,
};
use tungstenite::client::IntoClientRequest;

use crate::gateway::WSS_URL;

use super::{
    AccountNotificationEnvelope, AccountSubscribe, AccountSubscribeConfig,
    AccountSubscribeResponse, JsonRpcRequest, JsonRpcResponse, JsonRpcResponseWithError,
    SubscriptionError,
};

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
        self.handle_response(1).await
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
        let result = self.handle_response::<bool>(2).await?;
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
                Ok(Message::Text(text)) => {
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
