use dioxus_std::utils::channel::UseChannel;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;
use web_sys::{DedicatedWorkerGlobalScope, MessageEvent, Worker, WorkerOptions, WorkerType};
use web_time::Instant;

/// Mining request for web workers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebWorkerRequest {
    pub challenge: [u8; 32],
    pub nonce: u64,
    pub cutoff_time: u64,
}

/// Mining response for web workers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebWorkerResponse {
    pub hash: [u8; 32],
    pub nonce: u64,
    pub difficulty: u32,
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
            let req: WebWorkerRequest = from_value(event.data()).unwrap();
            let res = find_next_hash(req.challenge, req.nonce, req.cutoff_time);
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

pub fn create_web_worker(ch: UseChannel<WebWorkerResponse>) -> Worker {
    let worker = Worker::new_with_options("worker.js", &worker_options()).unwrap();
    let ch = ch.clone();

    // On message
    worker.set_onmessage(Some(&js_sys::Function::unchecked_from_js(
        Closure::<dyn Fn(MessageEvent)>::new(move |event: MessageEvent| {
            let res: WebWorkerResponse = from_value(event.data()).unwrap();
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

// TODO Update this to run for X seconds
pub fn find_next_hash(challenge: [u8; 32], mut nonce: u64, cutoff_time: u64) -> WebWorkerResponse {
    let timer = Instant::now();
    let mut best_hash = [0u8; 32];
    let mut best_nonce = 0u64;
    let mut best_difficulty = 0u32;
    loop {
        println!("Asdf");
        if let Ok(hash) = drillx::hash(&challenge, &nonce.to_le_bytes()) {
            let difficulty = hash.difficulty();
            if difficulty.gt(&best_difficulty) {
                best_difficulty = difficulty;
                best_nonce = nonce;
                best_hash = hash.h;
            }
        }

        if timer.elapsed().as_secs().gt(&cutoff_time) {
            if best_difficulty >= ore::MIN_DIFFICULTY {
                break;
            }
        }

        nonce += 1;
    }
    WebWorkerResponse {
        hash: best_hash,
        nonce: best_nonce,
        difficulty: best_difficulty,
    }
}
