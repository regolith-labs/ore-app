use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::components::{
    Claim, Download, Home, HowItWorks, Landing, Leaderboard, Miner, Navbar, PageNotFound, Send,
    Settings, Tokenomics, Tx, User,
};

#[rustfmt::skip]
#[derive(Routable, Clone)]
pub enum Route {
    #[route("/")]
    Landing {},
    #[route("/how-it-works")]
    HowItWorks {},
    #[route("/tokenomics")]
    Tokenomics {},

    #[layout(Navbar)]
        #[route("/download")]
        Download {},
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
