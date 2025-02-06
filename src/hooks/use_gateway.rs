use std::rc::Rc;

#[cfg(not(feature = "web"))]
use crate::gateway::NativeRpc;
#[cfg(feature = "web")]
use crate::gateway::WebRpc;
use crate::gateway::{Gateway, RPC_URL};

#[cfg(feature = "web")]
pub fn use_gateway() -> Rc<Gateway<WebRpc>> {
    Rc::new(Gateway::new(RPC_URL.to_string()))
}

#[cfg(not(feature = "web"))]
pub fn use_gateway() -> Rc<Gateway<NativeRpc>> {
    Rc::new(Gateway::new(RPC_URL.to_string()))
}
