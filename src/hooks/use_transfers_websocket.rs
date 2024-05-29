use dioxus::prelude::*;
use dioxus_std::utils::{channel::use_channel, rw::UseRw};
use futures::StreamExt;
use gloo::net::websocket::{futures::WebSocket, Message, WebSocketError};
use ore_types::Transfer;
use wasm_bindgen_futures::spawn_local;

use crate::{components::ActivityFilter, gateway::AsyncResult};

use super::use_pubkey;

const URL: &str = "wss://ore-websockets.onrender.com/ws";

// TODO Attempt reconnect if connection is lost

/// Spawn a coroutine with a websocket connection
pub fn use_transfers_websocket(
    cx: &ScopeState,
    filter: &UseState<ActivityFilter>,
    transfers: &mut UseRw<AsyncResult<Vec<Transfer>>>,
    offset: &UseState<u64>,
    has_more: &UseState<bool>,
    limit: usize,
) {
    let pubkey = use_pubkey(cx);
    let ch = use_channel::<Transfer>(cx, 1);

    let _ = use_future(cx, (), |_| {
        let mut rx = ch.clone().receiver();
        let filter = filter.clone();
        let transfers = transfers.clone();
        let offset = offset.clone();
        let has_more = has_more.clone();
        async move {
            while let Ok(transfer) = rx.recv().await {
                if (*offset.current()).eq(&0) {
                    let mut new_transfers: Vec<Transfer> = match transfers.read().unwrap().clone() {
                        AsyncResult::Ok(xx) => xx,
                        _ => vec![],
                    };
                    match *filter.current() {
                        ActivityFilter::Global => {
                            new_transfers.insert(0, transfer);
                        }
                        ActivityFilter::Personal => {
                            if transfer.from_address.eq(&pubkey.to_string())
                                || transfer.to_address.eq(&pubkey.to_string())
                            {
                                new_transfers.insert(0, transfer);
                            }
                        }
                    }
                    if new_transfers.len().gt(&limit) {
                        has_more.set(true);
                        new_transfers.truncate(limit);
                    }
                    transfers.write(AsyncResult::Ok(new_transfers)).unwrap();
                }
            }
        }
    });

    let _ws = use_coroutine(cx, |mut _rx: UnboundedReceiver<Message>| {
        let ch = ch.clone();
        async move {
            let ws = WebSocket::open(URL).unwrap();
            let (mut _write, mut rx) = ws.split();
            spawn_local(async move {
                while let Some(msg) = rx.next().await {
                    match msg {
                        Ok(Message::Text(text)) => match serde_json::from_str::<Transfer>(&text) {
                            Ok(transfer) => {
                                ch.send(transfer).await.ok();
                            }
                            Err(err) => {
                                log::error!("Failed to parse transfer: {:?}", err)
                            }
                        },
                        Ok(Message::Bytes(_)) => {}
                        Err(WebSocketError::ConnectionClose(event)) => {
                            log::info!("[WebSocket]: {:#?}", event);
                        }
                        Err(err) => log::error!("[WebSocket]: {:#?}", err),
                    }
                }
            });
        }
    });
}
