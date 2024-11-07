use crate::steel_app::solana::{
    sdk::signature::Signature, transaction_status::TransactionConfirmationStatus,
};
use crate::steel_app::time::Duration;

use super::{Gateway, GatewayError, GatewayResult};

const CONFIRM_RETRIES: usize = 20;
const CONFIRM_DELAY: u64 = 500;

impl Gateway {
    pub async fn confirm_signature(&self, sig: Signature) -> GatewayResult<Signature> {
        // Confirm tx
        for _ in 0..CONFIRM_RETRIES {
            // Delay before confirming
            async_std::task::sleep(Duration::from_millis(CONFIRM_DELAY)).await;

            // Fetch transaction status
            match self.rpc.get_signature_statuses(&[sig]).await {
                Ok(signature_statuses) => {
                    for signature_status in signature_statuses {
                        if let Some(signature_status) = signature_status.as_ref() {
                            if signature_status.confirmation_status.is_some() {
                                if let Some(current_commitment) =
                                    signature_status.confirmation_status.as_ref()
                                {
                                    match current_commitment {
                                        TransactionConfirmationStatus::Processed => {}
                                        TransactionConfirmationStatus::Confirmed
                                        | TransactionConfirmationStatus::Finalized => {
                                            log::info!("Confirmed: true");
                                            return Ok(sig);
                                        }
                                    }
                                }
                            } else {
                                log::info!("No status");
                            }
                        }
                    }
                }

                // Handle confirmation errors
                Err(err) => {
                    log::error!("Error confirming: {:?}", err);
                }
            }
        }

        return Err(GatewayError::TransactionTimeout);
    }
}
