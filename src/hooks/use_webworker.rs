use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

use dioxus::prelude::*;
use dioxus_std::utils::channel::UseChannel;
use serde::{Deserialize, Serialize};
#[cfg(feature = "web")]
use serde_wasm_bindgen::{from_value, to_value};
#[cfg(feature = "web")]
use solana_client_wasm::solana_sdk::{
    keccak::{hashv, Hash as KeccakHash},
    pubkey::Pubkey,
};
#[cfg(feature = "desktop")]
use solana_sdk::{
    keccak::{hashv, Hash as KeccakHash},
    pubkey::Pubkey,
};
#[cfg(feature = "web")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "web")]
use web_sys::{DedicatedWorkerGlobalScope, MessageEvent, Worker, WorkerOptions, WorkerType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WebworkerRequest {
    Pause,
    Mine(MineRequest),
}

/// Mining request for web workers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MineRequest {
    pub hash: KeccakHash,
    pub difficulty: KeccakHash,
    pub pubkey: Pubkey,
}

/// Mining response from web workers
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MiningResult {
    pub hash: KeccakHash,
    pub nonce: u64,
}

#[derive(PartialEq)]
pub struct Miner {
    #[cfg(feature = "web")]
    worker: Worker,
    #[cfg(feature = "desktop")]
    ch: UseChannel<MiningResult>,
}

impl Miner {
    pub fn new(ch: &UseChannel<MiningResult>) -> Self {
        Self {
            #[cfg(feature = "web")]
            worker: create_worker(ch),
            #[cfg(feature = "desktop")]
            ch: ch.clone(),
        }
    }

    pub fn start_mining(&self, hash: KeccakHash, difficulty: KeccakHash, signer: Pubkey) {
        #[cfg(feature = "web")]
        {
            let req = WebworkerRequest::Mine(MineRequest {
                hash,
                difficulty,
                pubkey: signer,
            });
            let msg = to_value(&req).unwrap();
            self.worker.post_message(&msg).unwrap();
        }

        #[cfg(feature = "desktop")]
        {
            let ch = self.ch.clone();
            let flag = Arc::new(AtomicBool::new(false));
            let result = Arc::new(Mutex::new(MiningResult::default()));
            let concurrency = 4;
            let handles: Vec<_> = (0..concurrency)
                .map(|i| {
                    std::thread::spawn({
                        let flag = flag.clone();
                        let result = result.clone();
                        move || {
                            let nonce = u64::MAX.saturating_div(concurrency).saturating_mul(i);
                            if let Some(res) =
                                find_next_hash_par(hash, difficulty, signer, nonce, flag.clone())
                            {
                                flag.store(true, Ordering::Relaxed);
                                let mut w_result = result.lock().unwrap();
                                *w_result = res;
                            }
                        }
                    })
                })
                .collect();
            for h in handles {
                h.join().unwrap();
            }
            let r_result = result.lock().unwrap();
            let res = r_result.clone();
            async_std::task::spawn(async move {
                ch.send(res).await.ok();
            });
        }
    }
}

#[cfg(feature = "web")]
fn find_next_hash(hash: KeccakHash, difficulty: KeccakHash, signer: Pubkey) -> MiningResult {
    let mut next_hash: KeccakHash;
    let mut nonce = 0u64;
    loop {
        if nonce % 10_000 == 0 {
            log::info!("Nonce: {:?}", nonce);
        }
        next_hash = hashv(&[
            hash.to_bytes().as_slice(),
            signer.to_bytes().as_slice(),
            nonce.to_be_bytes().as_slice(),
        ]);
        if next_hash.le(&difficulty) {
            break;
        }
        nonce += 1;
    }
    MiningResult {
        hash: next_hash,
        nonce,
    }
}

#[cfg(feature = "desktop")]
fn find_next_hash_par(
    hash: KeccakHash,
    difficulty: KeccakHash,
    signer: Pubkey,
    nonce: u64,
    flag: Arc<AtomicBool>,
) -> Option<MiningResult> {
    let mut next_hash: KeccakHash;
    let mut nonce = nonce;
    loop {
        if nonce % 10_000 == 0 {
            if flag.load(Ordering::Relaxed) {
                return None;
            }
            log::info!("Nonce: {:?}", nonce);
        }
        next_hash = hashv(&[
            hash.to_bytes().as_slice(),
            signer.to_bytes().as_slice(),
            nonce.to_be_bytes().as_slice(),
        ]);
        if next_hash.le(&difficulty) {
            break;
        }
        nonce += 1;
    }
    Some(MiningResult {
        hash: next_hash,
        nonce,
    })
}

pub fn use_miner<'a>(
    cx: &'a ScopeState,
    // message: &'a UseRef<Option<MiningResult>>,
    ch: &'a UseChannel<MiningResult>,
) -> &'a UseState<Miner> {
    use_state(cx, || Miner::new(ch))
}

#[cfg(feature = "web")]
pub fn create_worker(ch: &UseChannel<MiningResult>) -> Worker {
    let worker = Worker::new_with_options("worker.js", &worker_options()).unwrap();
    let ch = ch.clone();

    // On message
    worker.set_onmessage(Some(&js_sys::Function::unchecked_from_js(
        Closure::<dyn Fn(MessageEvent)>::new(move |event: MessageEvent| {
            let res: MiningResult = from_value(event.data()).unwrap();
            log::info!("Message from worker: {:?}", res);
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

#[cfg(feature = "web")]
#[wasm_bindgen]
pub fn start_webworker() {
    log::info!("Starting webworker");

    let self_ = js_sys::global();
    let js_value = std::ops::Deref::deref(&self_);
    let scope = DedicatedWorkerGlobalScope::unchecked_from_js_ref(js_value);
    let scope_ = scope.clone();

    scope.set_onmessage(Some(&js_sys::Function::unchecked_from_js(
        Closure::<dyn Fn(MessageEvent)>::new(move |event: MessageEvent| {
            log::info!("Received message {:?}", event.data());
            let req: WebworkerRequest = from_value(event.data()).unwrap();
            match req {
                WebworkerRequest::Mine(req) => {
                    let res = find_next_hash(req.hash, req.difficulty, req.pubkey);
                    scope_.post_message(&to_value(&res).unwrap()).unwrap();
                }
                WebworkerRequest::Pause => {
                    // flag.store(true, std::sync::atomic::Ordering::Relaxed);
                }
            };
        })
        .into_js_value(),
    )))
}

#[cfg(feature = "web")]
fn worker_options() -> WorkerOptions {
    let mut options = WorkerOptions::new();
    options.type_(WorkerType::Module);
    options
}
