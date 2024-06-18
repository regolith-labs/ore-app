use std::rc::Rc;

use dioxus::prelude::*;

use crate::gateway::{Gateway, API_URL};

use super::use_rpc_url;

pub fn use_gateway() -> Rc<Gateway> {
    let rpc_url = use_rpc_url().read().0.clone();
    Rc::new(Gateway::new(API_URL.to_string(), rpc_url))
}
