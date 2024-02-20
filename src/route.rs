use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::components::{
    Download, Home, Landing, Miner, Navbar, PageNotFound, Send, Settings, Tx, User,
};

#[rustfmt::skip]
#[derive(Routable, Clone)]
pub enum Route {
    #[layout(Navbar)]
        #[route("/")]
        Landing {},
        #[route("/download")]
        Download {},
        #[layout(Miner)]
            #[route("/home")]
            Home {},
            #[route("/me")]
            Settings {},
            #[route("/t/new")]
            Send {},
            #[route("/t/:sig")]
            Tx {
                sig: String,
            },
            #[route("/u/:id")]
            User {
                id: String,
            },
        #[end_layout]
    #[end_layout]

    #[route("/:.._route")]
    PageNotFound { _route: Vec<String> },
}
