use dioxus::prelude::*;
use dioxus_std::utils::rw::UseRw;
#[cfg(feature = "web")]
use futures::StreamExt;
#[cfg(feature = "web")]
use gloo::net::websocket::{futures::WebSocket, Message, WebSocketError};
use ore_types::Transfer;

#[cfg(feature = "web")]
use wasm_bindgen_futures::spawn_local;

use crate::{components::ActivityFilter, gateway::AsyncResult};

use super::use_pubkey;

// const URL: &str = "ws://54.86.50.170:3001/ws";
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
    let pubkey = use_pubkey(cx).to_string();

    // TODO Support desktop
    #[cfg(feature = "web")]
    let _ws = use_coroutine(cx, |mut _rx: UnboundedReceiver<Message>| {
        let filter = filter.clone();
        let transfers = transfers.clone();
        let offset = offset.clone();
        let has_more = has_more.clone();
        async move {
            let ws = WebSocket::open(URL).unwrap();
            let (mut _write, mut read) = ws.split();
            spawn_local(async move {
                while let Some(msg) = read.next().await {
                    if (*offset.current()).eq(&0) {
                        match msg {
                            Ok(Message::Text(text)) => {
                                match serde_json::from_str::<Transfer>(&text) {
                                    Ok(transfer) => {
                                        let mut ts: Vec<Transfer> =
                                            match transfers.read().unwrap().clone() {
                                                AsyncResult::Ok(xx) => xx,
                                                _ => vec![],
                                            };
                                        match *filter.current() {
                                            ActivityFilter::Global => {
                                                ts.insert(0, transfer);
                                            }
                                            ActivityFilter::Personal => {
                                                if transfer.from_address.eq(&pubkey)
                                                    || transfer.to_address.eq(&pubkey)
                                                {
                                                    ts.insert(0, transfer);
                                                }
                                            }
                                        }
                                        if ts.len().gt(&limit) {
                                            has_more.set(true);
                                            ts.truncate(limit);
                                        }
                                        transfers.write(AsyncResult::Ok(ts)).unwrap();
                                    }
                                    Err(e) => log::error!("Failed to deserialize transfer: {}", e),
                                }
                            }
                            Ok(Message::Bytes(_)) => {}
                            Err(WebSocketError::ConnectionClose(event)) => {
                                log::info!("[WebSocket]: {:#?}", event);
                            }
                            Err(err) => log::error!("[WebSocket]: {:#?}", err),
                        }
                    }
                }
            });
        }
    });
}
