use dioxus::prelude::*;

use crate::components::{AppNavigation, LandingNavigation};
use crate::pages::*;

#[rustfmt::skip]
#[derive(Routable, Clone, PartialEq, Eq)]
pub enum Route {
    #[layout(LandingNavigation)]
        #[route("/")]
        Landing {},
    #[end_layout]

    #[layout(AppNavigation)]
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
