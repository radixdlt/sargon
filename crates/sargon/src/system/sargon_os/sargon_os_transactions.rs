use crate::prelude::*;
use tokio::time::{sleep, Duration};

// ==================
// Submit Transaction
// ==================
#[uniffi::export]
impl SargonOS {
    /// Submits a notarized transaction payload to the network.
    pub async fn submit_transaction(
        &self,
        notarized_transaction: NotarizedTransaction,
    ) -> Result<IntentHash> {
        let network_id = self.current_network_id()?;
        let gateway_client = GatewayClient::new(
            self.clients.http_client.driver.clone(),
            network_id,
        );
        gateway_client
            .submit_notarized_transaction(notarized_transaction)
            .await
    }

    /// Submits a compiled transaction payload to the network.
    pub async fn submit_compiled_transaction(
        &self,
        compiled_notarized_intent: CompiledNotarizedIntent,
    ) -> Result<IntentHash> {
        self.submit_transaction(compiled_notarized_intent.decompile())
            .await
    }
}

// ==================
// Poll Transaction Status
// ==================
#[uniffi::export]
impl SargonOS {
    /// Polls the state of a Transaction until we can determine its `TransactionStatus`.
    pub async fn poll_transaction_status(
        &self,
        intent_hash: IntentHash,
    ) -> Result<TransactionStatus> {
        let network_id = self.current_network_id()?;
        let gateway_client = GatewayClient::new(
            self.clients.http_client.driver.clone(),
            network_id,
        );

        let mut delay_duration = 1;
        loop {
            // Increase delay by 1 second on subsequent calls
            delay_duration += 1;
            let sleep_duration = Duration::from_secs(delay_duration);

            let response = gateway_client
                .get_transaction_status(intent_hash.clone())
                .await?;

            match response.known_payloads.first().and_then(|payload| payload.payload_status.clone()) {
                Some(status) => {
                    match status {
                        TransactionStatusResponsePayloadStatus::Unknown |
                        TransactionStatusResponsePayloadStatus::Pending |
                        TransactionStatusResponsePayloadStatus::CommitPendingOutcomeUnknown => {
                            sleep(sleep_duration).await;
                        }
                        TransactionStatusResponsePayloadStatus::CommittedSuccess => {
                            return Ok(TransactionStatus::Success);
                        }
                        TransactionStatusResponsePayloadStatus::CommittedFailure => {
                            return Ok(TransactionStatus::Failed { reason: TransactionStatusReason::from_raw_error(response.error_message) });
                        }
                        TransactionStatusResponsePayloadStatus::PermanentlyRejected => {
                            return Ok(TransactionStatus::PermanentlyRejected { reason: TransactionStatusReason::from_raw_error(response.error_message) });
                        }
                        TransactionStatusResponsePayloadStatus::TemporarilyRejected => {
                            return Ok(TransactionStatus::TemporarilyRejected { current_epoch: Epoch::from(response.ledger_state.epoch) });
                        }
                    }
                }
                None => {
                    sleep(sleep_duration).await;
                }
            }
        }
    }
}
