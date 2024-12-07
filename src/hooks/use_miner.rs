use dioxus::prelude::*;
use futures::StreamExt;
use gloo_worker::{Spawnable, Worker, WorkerScope};

type FromMiner = Signal<String>;
type ToMiner = Coroutine<String>;
/// two way channel between us and miner (web worker)
pub fn use_miner() -> (FromMiner, ToMiner) {
    // from miner receiver
    let mut from_miner = use_signal(|| "init".to_string());
    // to miner sender
    let to_miner = use_coroutine(|mut rx| async move {
        // build new miner
        let mut spawner = Miner::spawner();
        let miner = spawner
            // callback for miner to send messages back to us
            .callback(move |msg| {
                spawn({
                    async move {
                        // send message back to us
                        from_miner.set(msg);
                    }
                });
            })
            // spawn new miner
            .spawn(Miner::path());
        // miner listen for messages from us
        while let Some(msg) = rx.next().await {
            // send message from us to miner
            log::info!("msg received: {:?}", msg);
            miner.send(msg);
        }
    });
    // two way channel
    (from_miner, to_miner)
}

pub struct Miner;

impl Miner {
    fn path() -> &'static str {
        "worker/miner.js"
    }
}

impl Worker for Miner {
    /// Update message type.
    type Message = String;
    /// Incoming message type.
    type Input = String;
    /// Outgoing message type.
    type Output = String;

    /// Creates an instance of a worker.
    fn create(_: &WorkerScope<Self>) -> Self {
        Miner
    }

    /// Receives an update.
    ///
    /// This method is called when the worker send messages to itself via [`WorkerScope::send_message`].
    fn update(&mut self, _scope: &WorkerScope<Self>, _msg: Self::Message) {}

    /// Receives an input from a connected bridge.
    ///
    /// When a bridge sends an input via [`WorkerBridge::send`](crate::WorkerBridge::send), the worker will receive the
    /// input via this method.
    fn received(
        &mut self,
        scope: &WorkerScope<Self>,
        msg: Self::Input,
        id: gloo_worker::HandlerId,
    ) {
        // Simulate processing the received message
        let output = format!("Processed: {}", msg);
        log::info!("{}", output);

        // Send the result back to the main thread
        scope.respond(id, output)
    }
}
