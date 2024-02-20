use dioxus::prelude::*;
use ore::{state::Treasury, TREASURY_ADDRESS};

use crate::gateway::AsyncResult;

use super::use_account;

pub fn use_treasury(cx: &ScopeState) -> AsyncResult<Treasury> {
    use_account(cx, TREASURY_ADDRESS)
}
