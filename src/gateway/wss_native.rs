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

use super::{
    AccountContext, AccountNotification, AccountNotificationEnvelope, AccountNotificationResult,
    AccountSubscribe, AccountSubscribeConfig, AccountSubscribeParams, AccountSubscribeResponse,
    JsonRpcError, JsonRpcRequest, JsonRpcResponse, JsonRpcResponseWithError, SubscriptionError,
};

pub struct AccountSubscribeNative {
    writer: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    reader: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

impl AccountSubscribeNative {
    pub async fn connect(url: &str) -> Result<Self, SubscriptionError> {
        let url_parsed = url
            .into_client_request()
            .map_err(|e| SubscriptionError::Other(e.to_string()))?;
        let (ws_stream, _) = connect_async(url_parsed)
            .await
            .map_err(|e| SubscriptionError::ConnectionError(e.to_string()))?;
        let (writer, reader) = ws_stream.split();
        Ok(Self { writer, reader })
    }

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
}

#[async_trait]
impl AccountSubscribe for AccountSubscribeNative {
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
        self.send_request(&request).await?;
        while let Some(msg) = self.reader.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    if let Ok(resp) =
                        serde_json::from_str::<JsonRpcResponse<AccountSubscribeResponse>>(&text)
                    {
                        if resp.id == 1 {
                            return Ok(resp.result);
                        }
                    }
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
            id: 2,
            method: "accountUnsubscribe".to_string(),
            params: (subscription,),
        };
        self.send_request(&request).await?;
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
        while let Some(msg) = self.reader.next().await {
            match msg {
                Ok(Message::Text(text)) => {
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
