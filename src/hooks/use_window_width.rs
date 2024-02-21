// use dioxus::prelude::*;
// use wasm_bindgen::prelude::*;

// #[allow(dead_code)]
// pub struct WindowWidth(i32);

// pub fn _use_window_width(cx: &ScopeState) -> i32 {
//     use_shared_state::<WindowWidth>(cx).unwrap().read().0
// }

// pub fn _track_window_width(cx: &ScopeState) {
//     use_shared_state_provider(cx, || WindowWidth(0));
//     let window_width = use_shared_state::<WindowWidth>(cx).unwrap();

//     use_effect(cx, (), |_| {
//         let window_width = window_width.clone();
//         let closure = Closure::wrap(Box::new(move |width: i32| {
//             *window_width.write() = WindowWidth(width);
//         }) as Box<dyn FnMut(i32)>);
//         setup_resize_listener(&closure);
//         closure.forget();
//         async move {}
//     });
// }

// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_name = setupResizeListener)]
//     fn setup_resize_listener(closure: &Closure<dyn FnMut(i32)>);
// }
