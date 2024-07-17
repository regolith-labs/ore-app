use dioxus::prelude::*;

use crate::components::{
    Claim, Download, Home, Landing, MinerToolbarLayout, Navbar, OreTokenomics, PageNotFound, Send,
    Settings, SimpleNavbar, Stake, Tx, Upgrade, User, WhatIsMining,
};

#[rustfmt::skip]
#[derive(Routable, Clone)]
pub enum Route {
    #[route("/")]
    Landing {},

    #[layout(SimpleNavbar)]
        #[route("/what-is-mining")]
        WhatIsMining {},
        #[route("/ore-tokenomics")]
        OreTokenomics {},
        #[route("/download")]
        Download {},
    #[end_layout]

    #[layout(Navbar)]
        #[layout(MinerToolbarLayout)]
            #[route("/home")]
            Home {},
            #[route("/claim")]
            Claim {},
            #[route("/stake")]
            Stake {},
            #[route("/settings")]
            Settings {},
            #[route("/send/:to")]
            Send {
                to: String
            },
            #[route("/tx/:sig")]
            Tx {
                sig: String,
            },
            #[route("/u/:id")]
            User {
                id: String,
            },
            #[route("/upgrade")]
            Upgrade {}, 
        #[end_layout]
    #[end_layout]

    #[route("/:.._route")]
    PageNotFound { _route: Vec<String> }
}
