use std::ops::Deref;

use dioxus::prelude::*;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;
use web_sys::{DedicatedWorkerGlobalScope, MessageEvent, Worker, WorkerOptions, WorkerType};

use crate::{
    find_next_hash,
    gateway::{MineRequest, MineResponse},
};

pub fn use_webworker(cx: &ScopeState) -> (&mut Worker, &UseRef<Option<MineResponse>>) {
    let message = use_ref::<Option<MineResponse>>(cx, || None);

    let worker = cx.use_hook(|| {
        let worker = Worker::new_with_options("worker.js", &worker_options()).unwrap();
        let message = message.clone();

        let f: Closure<dyn Fn(MessageEvent)> = Closure::new(move |event: MessageEvent| {
            let res: MineResponse = from_value(event.data()).unwrap();
            log::info!("Message from worker: {:?}", res);
            *message.write() = Some(res);
        });

        let val = f.into_js_value();
        let f = js_sys::Function::unchecked_from_js(val);
        worker.set_onmessage(Some(&f));
        worker
    });

    (worker, message)
}

#[wasm_bindgen]
pub fn start_webworker() {
    log::info!("Starting webworker");

    let self_ = js_sys::global();
    let js_value = self_.deref();
    let scope = DedicatedWorkerGlobalScope::unchecked_from_js_ref(js_value);
    let _scope = scope.clone();

    let f: Closure<dyn Fn(MessageEvent)> = Closure::new(move |event: MessageEvent| {
        let req: MineRequest = from_value(event.data()).unwrap();
        let res = find_next_hash(req);
        _scope.post_message(&to_value(&res).unwrap()).unwrap();
    });

    let val = f.into_js_value();
    let f = js_sys::Function::unchecked_from_js(val);
    scope.set_onmessage(Some(&f))
}

fn worker_options() -> WorkerOptions {
    let mut options = WorkerOptions::new();
    options.type_(WorkerType::Module);
    options
}
