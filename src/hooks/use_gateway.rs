use std::rc::Rc;

use dioxus::prelude::*;

use crate::gateway::Gateway;

pub fn use_gateway(cx: &ScopeState) -> Rc<Gateway> {
    use_context::<Rc<Gateway>>(cx).unwrap().clone()
}
