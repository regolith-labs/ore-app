#[cfg(feature = "worker")]
use gloo_worker::Registrable;
use gloo_worker::{Worker, WorkerScope};
#[cfg(feature = "worker")]
use wasm_bindgen::prelude::*;

pub struct Miner;

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

#[cfg(feature = "worker")]
#[wasm_bindgen(start)]
pub async fn register_miner() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("starting miner");
    Miner::registrar().register();
}
