use dioxus::prelude::*;

use crate::components::{Home, PageNotFound};

#[rustfmt::skip]
#[derive(Routable, Clone)]
pub enum Route {
    #[route("/home")]
    Home {},

    #[route("/:.._route")]
    PageNotFound { _route: Vec<String> }
}
