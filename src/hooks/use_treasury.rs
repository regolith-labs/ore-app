use dioxus::prelude::*;
use dioxus_std::utils::rw::UseRw;
use ore::{state::Treasury, TREASURY_ADDRESS};

use crate::gateway::AsyncResult;

use super::use_account;

pub fn use_treasury(cx: &ScopeState) -> (&mut UseRw<AsyncResult<Treasury>>, &UseFuture<()>) {
    use_account(cx, TREASURY_ADDRESS, Some(60))
}
