use gloo_worker::{Worker, WorkerScope};

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

impl Miner {
    pub fn path() -> &'static str {
        "worker/miner.js"
    }
}
