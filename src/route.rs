use dioxus::prelude::*;

use crate::components::NavbarLayout;
use crate::pages::*;

#[rustfmt::skip]
#[derive(Routable, Clone,PartialEq, Eq)]
pub enum Route {
    #[route("/")]
    Landing {},

    #[layout(NavbarLayout)]
        #[route("/mine")]
        Mine {},
        #[route("/stake")]
        Stake {},
        #[route("/trade")]
        Trade {},
    #[end_layout]

    #[route("/:.._route")]
    NotFound { _route: Vec<String> }
}
