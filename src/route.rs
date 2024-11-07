use dioxus::prelude::*;

use crate::pages::*;

#[rustfmt::skip]
#[derive(Routable, Clone)]
pub enum Route {
    #[route("/")]
    Landing {},

    #[route("/mine")]
    Mine {},
    #[route("/stake")]
    Stake {},
    #[route("/trade")]
    Trade {},

    #[route("/:.._route")]
    NotFound { _route: Vec<String> }
}
