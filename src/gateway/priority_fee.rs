use serde_json::{json, Value};
use solana_sdk::transaction::VersionedTransaction;

use super::{Gateway, GatewayError, GatewayResult, Rpc, RPC_URL};

impl<R: Rpc> Gateway<R> {
    pub async fn get_recent_priority_fee_estimate(
        &self,
        tx: &VersionedTransaction,
    ) -> GatewayResult<u64> {
        match bincode::serialize(&tx) {
            Ok(tx_bytes) => {
                let tx_bs58 = bs58::encode(tx_bytes).into_string();
                let req = json!({
                    "jsonrpc": "2.0",
                    "id": "priority-fee-estimate",
                    "method": "getPriorityFeeEstimate",
                    "params": [{
                        "transaction": tx_bs58,
                        "options": {
                            "recommended": true
                        }
                    }]
                });
                if let Ok(res) = self.http.post(RPC_URL.to_string()).json(&req).send().await {
                    if let Ok(res) = res.json::<Value>().await {
                        // Get dynamic fee estimate in microlamports
                        let dynamic_fee_estimate = res["result"]["priorityFeeEstimate"]
                            .as_f64()
                            .map(|fee| fee as u64)
                            .unwrap_or(0);
                        return Ok(dynamic_fee_estimate);
                    } else {
                        log::error!("Failed to parse priority fee json");
                        return Err(GatewayError::Unknown);
                    }
                } else {
                    log::error!("Failed to send priority fee estimate request");
                    return Err(GatewayError::Unknown);
                }
            }
            Err(e) => {
                log::error!("err serializing tx: {}", e);
                Err(GatewayError::Unknown)
            }
        }
    }
}
