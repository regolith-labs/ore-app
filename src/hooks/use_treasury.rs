use dioxus::prelude::*;
use ore::{state::Treasury, TREASURY_ADDRESS};

use crate::gateway::AsyncResult;

use super::use_account;

pub fn use_treasury() -> Signal<AsyncResult<Treasury>> {
    use_account(TREASURY_ADDRESS, Some(60)).0
}
