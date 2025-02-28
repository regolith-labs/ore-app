use dioxus::prelude::*;

use crate::route::Route;

pub fn Landing() -> Element {
    let navigator = use_navigator();
    navigator.replace(Route::Mine {});
    rsx! {}
}
