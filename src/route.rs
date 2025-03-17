use dioxus::prelude::*;

use crate::components::*;
use crate::pages::*;

#[rustfmt::skip]
#[derive(Routable, Clone, PartialEq, Eq)]
pub enum Route {
    #[layout(AppLayout)]
        #[route("/mine")]
        Mine {},
        #[route("/pay")]
        Pay {},
        #[route("/stake")]
        Stake {},
        #[route("/stake/ore")]
        Idle {},
        #[route("/stake/:lp_mint")]
        Pair { lp_mint: String },
        #[route("/trade")]
        Trade {},
        #[route("/trade/:token_pair")]
        TradeWithPair { token_pair: String },
    #[end_layout]

    #[layout(AppModalLayout)]
        #[route("/topup/:address")]
        Topup { address: String },
    #[end_layout]

    #[layout(LandingLayout)]
        #[route("/")]
        Landing {},
    #[end_layout]

    #[layout(VisitorLayout)]
        #[route("/download")]
        Download {},
        #[route("/blog")]
        Blog {},
    #[end_layout]

    #[route("/:.._route")]
    NotFound { _route: Vec<String> }
}
