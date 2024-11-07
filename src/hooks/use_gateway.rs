use std::rc::Rc;

use crate::gateway::{Gateway, RPC_URL};

pub fn use_gateway() -> Rc<Gateway> {
    Rc::new(Gateway::new(RPC_URL.to_string()))
}
