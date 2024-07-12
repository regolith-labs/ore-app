use std::rc::Rc;

use crate::gateway::{Gateway, API_URL, RPC_URL};

pub fn use_gateway() -> Rc<Gateway> {
    Rc::new(Gateway::new(API_URL.to_string(), RPC_URL.to_string()))
}
