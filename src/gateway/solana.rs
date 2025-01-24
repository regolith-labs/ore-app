use solana_sdk::signature::Signature;
use steel::Clock;

use crate::gateway::transaction_confirmation_status::TransactionConfirmationStatus;

use super::{GatewayError, GatewayResult, Rpc};

const CONFIRM_RETRIES: usize = 20;
const CONFIRM_DELAY: u64 = 1_500;

pub trait SolanaGateway {
    async fn get_clock(&self) -> GatewayResult<Clock>;
    async fn confirm_signature(&self, sig: Signature) -> GatewayResult<Signature>;
}

impl<R: Rpc> SolanaGateway for R {
    async fn get_clock(&self) -> GatewayResult<Clock> {
        let data = self
            .get_account_data(&solana_sdk::sysvar::clock::ID)
            .await
            .map_err(GatewayError::from)?;
        bincode::deserialize::<Clock>(&data).or(Err(GatewayError::FailedDeserialization))
    }
    async fn confirm_signature(&self, sig: Signature) -> GatewayResult<Signature> {
        // Confirm tx
        for retry in 0..CONFIRM_RETRIES {
            // Delay before confirming
            async_std::task::sleep(crate::time::Duration::from_millis(CONFIRM_DELAY)).await;
            // Fetch transaction status
            match self.get_signature_statuses(&[sig]).await {
                Ok(signature_statuses) => {
                    for signature_status in signature_statuses {
                        if let Some(signature_status) = signature_status.as_ref() {
                            match signature_status {
                                TransactionConfirmationStatus::Processed => {}
                                TransactionConfirmationStatus::Confirmed
                                | TransactionConfirmationStatus::Finalized => {
                                    log::info!("Confirmed: true");
                                    return Ok(sig);
                                }
                            }
                        }
                    }
                }
                // Handle confirmation errors
                Err(err) => {
                    log::error!("Error confirming: {:?}", err);
                }
            }
            log::info!("retry: {}", retry);
        }
        return Err(GatewayError::TransactionTimeout);
    }
}
