use dioxus_std::utils::channel::UseChannel;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;
use web_sys::{DedicatedWorkerGlobalScope, MessageEvent, Worker, WorkerOptions, WorkerType};

use crate::miner::{find_next_hash, MineRequest, MiningResult};

pub fn create_worker(ch: &UseChannel<MiningResult>) -> Worker {
    let worker = Worker::new_with_options("worker.js", &worker_options()).unwrap();
    let ch = ch.clone();

    // On message
    worker.set_onmessage(Some(&js_sys::Function::unchecked_from_js(
        Closure::<dyn Fn(MessageEvent)>::new(move |event: MessageEvent| {
            let res: MiningResult = from_value(event.data()).unwrap();
            wasm_bindgen_futures::spawn_local({
                let ch = ch.clone();
                async move {
                    ch.send(res).await.ok();
                }
            });
        })
        .into_js_value(),
    )));

    // On error
    worker.set_onerror(Some(&js_sys::Function::unchecked_from_js(
        Closure::<dyn Fn(MessageEvent)>::new(move |e: MessageEvent| {
            log::info!("Error from worker: {:?}", e.data());
        })
        .into_js_value(),
    )));

    worker
}

#[wasm_bindgen]
pub fn start_worker() {
    log::info!("Starting webworker");

    let self_ = js_sys::global();
    let js_value = std::ops::Deref::deref(&self_);
    let scope = DedicatedWorkerGlobalScope::unchecked_from_js_ref(js_value);
    let scope_ = scope.clone();

    scope.set_onmessage(Some(&js_sys::Function::unchecked_from_js(
        Closure::<dyn Fn(MessageEvent)>::new(move |event: MessageEvent| {
            let req: MineRequest = from_value(event.data()).unwrap();
            let res = find_next_hash(req.hash, req.difficulty, req.pubkey);
            scope_.post_message(&to_value(&res).unwrap()).unwrap();
        })
        .into_js_value(),
    )))
}

fn worker_options() -> WorkerOptions {
    let mut options = WorkerOptions::new();
    options.type_(WorkerType::Module);
    options
}
