use dioxus::prelude::*;

use crate::components::VisitorNavigation;
use crate::pages::*;

#[rustfmt::skip]
#[derive(Routable, Clone, PartialEq, Eq)]
pub enum Route {
    // #[layout(AppNavigation)]
    //     #[route("/mine")]
    //     Mine {},
    //     #[route("/pay")]
    //     Pay {},
    //     #[route("/pool/:pool")]
    //     Pool { pool: String },
    //     #[route("/stake")]
    //     Stake {},
    //     #[route("/trade")]
    //     Trade {},
    // #[end_layout]

    #[layout(VisitorNavigation)]
        #[route("/")]
        Landing {},
        // #[route("/download")]
        // Download {},
    #[end_layout]

    #[route("/:.._route")]
    NotFound { _route: Vec<String> }
}
