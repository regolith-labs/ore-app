import init from "/./assets/dioxus/ORE.js";

init("/./assets/dioxus/ORE_bg.wasm").then(wasm => {
  wasm.start_worker();
});
