use serde_json::{json, Value};

use crate::steel_app::solana::sdk::{clock::Clock, hash::Hash, sysvar};
use crate::steel_app::solana::{
    sdk::signature::Signature, transaction_status::TransactionConfirmationStatus,
};
use crate::steel_app::time::Duration;

use super::utils::retry;
use super::{Gateway, GatewayError, GatewayResult, RPC_URL};

const CONFIRM_RETRIES: usize = 20;
const CONFIRM_DELAY: u64 = 500;

impl Gateway {
    pub async fn get_clock(&self) -> GatewayResult<Clock> {
        retry(|| async {
            let data = self
                .rpc
                .get_account_data(&sysvar::clock::ID)
                .await
                .map_err(GatewayError::from)?;
            bincode::deserialize::<Clock>(&data).or(Err(GatewayError::FailedDeserialization))
        })
        .await
    }

    pub async fn get_latest_blockhash(&self) -> GatewayResult<Hash> {
        retry(|| async {
            self.rpc
                .get_latest_blockhash()
                .await
                .map_err(GatewayError::from)
        })
        .await
    }

    pub async fn confirm_signature(&self, sig: Signature) -> GatewayResult<Signature> {
        // Confirm tx
        for retry in 0..CONFIRM_RETRIES {
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
            log::info!("retry: {}", retry);
        }

        return Err(GatewayError::TransactionTimeout);
    }

    pub async fn get_recent_priority_fee_estimate(treasury: bool) -> u64 {
        let http_client = reqwest::Client::new();
        let mut ore_addresses: Vec<String> = vec![ore_api::id().to_string()];
        if treasury {
            ore_addresses.push(ore_api::consts::TREASURY_ADDRESS.to_string());
        }

        let req = json!({
            "jsonrpc": "2.0",
            "id": "priority-fee-estimate",
            "method": "getPriorityFeeEstimate",
            "params": [{
                "accountKeys": ore_addresses,
                "options": {
                    "recommended": true
                }
            }]
        });

        if let Ok(res) = http_client
            .post(RPC_URL.to_string())
            .json(&req)
            .send()
            .await
        {
            if let Ok(res) = res.json::<Value>().await {
                return res["result"]["priorityFeeEstimate"]
                    .as_f64()
                    .map(|fee| fee as u64)
                    .unwrap_or(0);
            }
        }

        0
    }
}
