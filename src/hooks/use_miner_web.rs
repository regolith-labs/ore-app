use dioxus::prelude::*;
use futures::StreamExt;
use gloo_utils::window;
use gloo_worker::Spawnable;
use js_sys::Array;
use ore_miner_web::Miner;
use web_sys::{Blob, BlobPropertyBag, Url};

const JS: Asset = asset!(
    "/public/miner.js",
    AssetOptions::Js(JsAssetOptions::new().with_minify(false))
);
const WASM: Asset = asset!("/public/miner_bg.wasm");

/// two way channel between us and miner (web worker)
pub fn use_miner_provider() {
    // from miner receiver
    let mut from_miner = use_context_provider(|| Signal::new(ore_miner_web::OutputMessage::Init));
    // to miner sender
    let _to_miner = use_coroutine(move |mut rx| async move {
        // callback for miner to send messages back to us
        let mut spawner = Miner::spawner();
        let miner = spawner
            .callback(move |msg| {
                log::info!("from worker: {:?}", msg);
                wasm_bindgen_futures::spawn_local(async move {
                    from_miner.set(msg);
                })
            })
            .spawn_with_loader(shim_url().as_str());

        // queue for miner to listen for messages from us
        while let Some(msg) = rx.next().await {
            log::info!("to worker: {:?}", msg);
            miner.send(msg);
        }
    });
}

fn shim_url() -> String {
    // build js url
    let js_shim_url = Url::new_with_base(
        JS.resolve().to_string_lossy().as_ref(),
        &window().location().href().expect("failed to read href."),
    )
    .expect("failed to create url for javascript entrypoint")
    .to_string();

    // build wasm url
    let wasm_url = Url::new_with_base(
        WASM.resolve().to_string_lossy().as_ref(),
        &window().location().href().expect("failed to read href."),
    )
    .expect("failed to create url for wasm entrypoint")
    .to_string();

    // create bootstrap script
    let array = Array::new();
    array.push(&format!(r#"importScripts("{js_shim_url}");wasm_bindgen("{wasm_url}");"#).into());
    let properties = BlobPropertyBag::new();
    properties.set_type("application/javascript");

    // encode as blob
    let blob = Blob::new_with_str_sequence_and_options(&array, &properties)
        .expect("failed to create blob");

    // serve as url
    Url::create_object_url_with_blob(&blob).expect("failed to create object url")
}
