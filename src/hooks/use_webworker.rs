use std::ops::Deref;

use dioxus::prelude::*;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;
use web_sys::{DedicatedWorkerGlobalScope, MessageEvent, Worker, WorkerOptions, WorkerType};

use crate::{
    find_next_hash,
    gateway::{WebworkerRequest, WebworkerResponse},
};

pub trait ResetWorker {
    fn reset(&self, message: &UseRef<Option<WebworkerResponse>>);
}

impl ResetWorker for &UseState<Worker> {
    fn reset(&self, message: &UseRef<Option<WebworkerResponse>>) {
        self.get().terminate();
        self.set(create_worker(message))
    }
}

pub fn use_webworker(cx: &ScopeState) -> (&UseState<Worker>, &UseRef<Option<WebworkerResponse>>) {
    let message = use_ref::<Option<WebworkerResponse>>(cx, || None);
    let worker = use_state(cx, || create_worker(message));
    (worker, message)
}

pub fn create_worker(message: &UseRef<Option<WebworkerResponse>>) -> Worker {
    let worker = Worker::new_with_options("worker.js", &worker_options()).unwrap();
    let message = message.clone();

    // On message
    worker.set_onmessage(Some(&js_sys::Function::unchecked_from_js(
        Closure::<dyn Fn(MessageEvent)>::new(move |event: MessageEvent| {
            let res: WebworkerResponse = from_value(event.data()).unwrap();
            log::info!("Message from worker: {:?}", res);
            *message.write() = Some(res);
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
pub fn start_webworker() {
    log::info!("Starting webworker");

    let self_ = js_sys::global();
    let js_value = self_.deref();
    let scope = DedicatedWorkerGlobalScope::unchecked_from_js_ref(js_value);
    let scope_ = scope.clone();

    scope.set_onmessage(Some(&js_sys::Function::unchecked_from_js(
        Closure::<dyn Fn(MessageEvent)>::new(move |event: MessageEvent| {
            log::info!("Received message {:?}", event.data());
            let req: WebworkerRequest = from_value(event.data()).unwrap();
            match dbg!(req) {
                WebworkerRequest::Mine(req) => {
                    let scope_ = scope_.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        if let Some(res) = find_next_hash(req).await {
                            scope_.post_message(&to_value(&res).unwrap()).unwrap();
                        }
                        log::info!("A");
                    });
                    log::info!("B");
                }
                WebworkerRequest::Pause => {
                    // flag.store(true, std::sync::atomic::Ordering::Relaxed);
                }
            };
        })
        .into_js_value(),
    )))
}

fn worker_options() -> WorkerOptions {
    let mut options = WorkerOptions::new();
    options.type_(WorkerType::Module);
    options
}
