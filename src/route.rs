use dioxus::prelude::*;

use crate::components::NavBarLayout;
use crate::pages::*;

#[rustfmt::skip]
#[derive(Routable, Clone, PartialEq, Eq)]
pub enum Route {
    #[route("/")]
    Landing {},

    #[layout(NavBarLayout)]
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
