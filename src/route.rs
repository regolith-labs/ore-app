use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::components::{
    Claim, Download, Home, Landing, Miner, Navbar, PageNotFound, Send, Settings, Tx, User,
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
            #[route("/claim")]
            Claim {},
            #[route("/me")]
            Settings {},
            #[route("/send")]
            Send {},
            #[route("/tx/:sig")]
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
    PageNotFound { _route: Vec<String> }
}
