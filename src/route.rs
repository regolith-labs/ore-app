use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::components::{
    Claim, Download, Home, Landing, Leaderboard, Miner, Navbar, OreTokenomics, PageNotFound, Send,
    Settings, SimpleNavbar, Tx, User, WhatIsMining,
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
        #[layout(Miner)]
            #[route("/home")]
            Home {},
            #[route("/claim")]
            Claim {},
            #[route("/leaderboard")]
            Leaderboard {},
            #[route("/me")]
            Settings {},
            #[route("/send")]
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
        #[end_layout]
    #[end_layout]

    #[route("/:.._route")]
    PageNotFound { _route: Vec<String> }
}
