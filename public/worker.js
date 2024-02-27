import init from "/./assets/dioxus/Ore.js";

init("/./assets/dioxus/Ore_bg.wasm").then(wasm => {
  wasm.start_worker();
});
