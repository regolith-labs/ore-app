use crate::gateway::GatewayError;
use crate::steel_app::time::Duration;
use async_std::future::{timeout, Future};

use super::GatewayResult;

pub async fn retry<F, Fut, T>(f: F) -> GatewayResult<T>
where
    F: Fn() -> Fut,
    Fut: Future<Output = GatewayResult<T>>,
{
    const MAX_RETRIES: u32 = 8;
    const INITIAL_BACKOFF: Duration = Duration::from_millis(200);
    const TIMEOUT: Duration = Duration::from_secs(8);
    let mut backoff = INITIAL_BACKOFF;
    for attempt in 0..MAX_RETRIES {
        match timeout(TIMEOUT, f()).await {
            Ok(Ok(result)) => return Ok(result),
            Ok(Err(e)) if attempt < MAX_RETRIES - 1 => {
                match e {
                    GatewayError::AccountNotFound => return Err(e),
                    _ => {
                        async_std::task::sleep(backoff).await;
                        backoff *= 2; // Exponential backoff
                    }
                }
            }
            Ok(Err(e)) => return Err(e),
            Err(_) if attempt < MAX_RETRIES - 1 => {
                async_std::task::sleep(backoff).await;
                backoff *= 2; // Exponential backoff
            }
            Err(_) => return Err(GatewayError::RetryFailed),
        }
    }

    Err(GatewayError::AccountNotFound)
}
