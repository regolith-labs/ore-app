use super::GatewayError;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AsyncResult<T> {
    Ok(T),
    Loading,
    Error(GatewayError),
}
