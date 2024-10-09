use crate::prelude::*;
use std::time::Duration;

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

        // The delay increment is set to 1 second in production, but 0 seconds in tests.
        // This will make the tests run with 0s delay, while the production code will have a 2s delay on first call,
        // a 3s delay on second call, 4s on third and so on.
        #[cfg(test)]
        const DELAY_INCREMENT: u64 = 0;
        #[cfg(not(test))]
        const DELAY_INCREMENT: u64 = 1;

        let mut delay_duration = DELAY_INCREMENT;
        loop {
            // Increase delay by 1 second on subsequent calls
            delay_duration += DELAY_INCREMENT;
            let sleep_duration = Duration::from_secs(delay_duration);

            let response = gateway_client
                .get_transaction_status(intent_hash.clone())
                .await?;

            match response
                .known_payloads
                .first()
                .and_then(|payload| payload.payload_status.clone())
            {
                Some(status) => {
                    match status {
                        TransactionStatusResponsePayloadStatus::Unknown |
                        TransactionStatusResponsePayloadStatus::Pending |
                        TransactionStatusResponsePayloadStatus::CommitPendingOutcomeUnknown => {
                            async_std::task::sleep(sleep_duration).await;
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
                    async_std::task::sleep(sleep_duration).await;
                }
            }
        }
    }
}

#[cfg(test)]
mod submit_transaction_tests {
    use super::*;
    use actix_rt::time::timeout;
    use std::{future::Future, time::Duration};

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonOS;

    #[actix_rt::test]
    async fn submit_transaction_success() {
        let notarized_transaction = NotarizedTransaction::sample();
        let response = TransactionSubmitResponse { duplicate: false };
        let body = serde_json::to_vec(&response).unwrap();

        let mock_driver =
            MockNetworkingDriver::with_spy(200, body, |request| {
                // Verify the body sent matches the expected one
                let sent_request = TransactionSubmitRequest::new(
                    NotarizedTransaction::sample(),
                );
                let sent_body = serde_json::to_vec(&sent_request).unwrap();

                assert_eq!(request.body.bytes, sent_body);
            });

        let req = SUT::boot_test_with_networking_driver(Arc::new(mock_driver));

        let os =
            actix_rt::time::timeout(SARGON_OS_TEST_MAX_ASYNC_DURATION, req)
                .await
                .unwrap()
                .unwrap();

        let result = os
            .submit_transaction(notarized_transaction.clone())
            .await
            .unwrap();

        let expected_result =
            notarized_transaction.signed_intent().intent().intent_hash();

        assert_eq!(result, expected_result);
    }

    #[actix_rt::test]
    async fn submit_transaction_failure() {
        let notarized_transaction = NotarizedTransaction::sample();
        let mock_driver = MockNetworkingDriver::new_always_failing();

        let req = SUT::boot_test_with_networking_driver(Arc::new(mock_driver));

        let os =
            actix_rt::time::timeout(SARGON_OS_TEST_MAX_ASYNC_DURATION, req)
                .await
                .unwrap()
                .unwrap();

        let result = os
            .submit_transaction(notarized_transaction)
            .await
            .expect_err("Expected an error");

        assert_eq!(result, CommonError::NetworkResponseBadCode);
    }
}

#[cfg(test)]
mod poll_status_tests {
    use super::*;
    use actix_rt::time::timeout;
    use std::{future::Future, time::Duration};

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonOS;

    #[actix_rt::test]
    async fn poll_status_success() {
        // This test will simulate the case where the first response is a `CommittedSuccess`
        let result =
            simulate_poll_status(vec![sample_committed_success()]).await;

        assert_eq!(result, TransactionStatus::Success);
    }

    #[actix_rt::test]
    async fn poll_status_empty_then_failure() {
        // This test will simulate the case where the first response is empty (no payload status),
        // while the second response is a `CommittedFailure`
        let result = simulate_poll_status(vec![
            sample_pending(),
            sample_committed_failure(None),
        ])
        .await;

        assert_eq!(
            result,
            TransactionStatus::Failed {
                reason: TransactionStatusReason::Unknown
            }
        );
    }

    #[actix_rt::test]
    async fn poll_status_unknown_then_permanently_rejected() {
        // This test will simulate the case where the first response is `Unknown`,
        // while the second response is a `PermanentlyRejected`
        let result = simulate_poll_status(vec![
            sample_unknown(),
            sample_permanently_rejected(Some("AssertionFailed".to_owned())),
        ])
        .await;

        assert_eq!(
            result,
            TransactionStatus::PermanentlyRejected {
                reason: TransactionStatusReason::WorktopError
            }
        );
    }

    #[actix_rt::test]
    async fn poll_status_commit_pending_outcome_unknown_then_temporarily_rejected(
    ) {
        // This test will simulate the case where the first response is `Unknown`,
        // while the second response is a `PermanentlyRejected`
        let result = simulate_poll_status(vec![
            sample_commit_pending_outcome_unknown(),
            sample_temporarily_rejected(),
        ])
        .await;

        let current_epoch = Epoch::from(LedgerState::sample_stokenet().epoch);
        assert_eq!(
            result,
            TransactionStatus::TemporarilyRejected { current_epoch }
        );
    }

    #[actix_rt::test]
    async fn poll_status_error() {
        // This test will simulate the case where we fail to get a `TransactionStatusResponse` from gateway
        let mock_driver = MockNetworkingDriver::new_always_failing();

        let req = SUT::boot_test_with_networking_driver(Arc::new(mock_driver));

        let os =
            actix_rt::time::timeout(SARGON_OS_TEST_MAX_ASYNC_DURATION, req)
                .await
                .unwrap()
                .unwrap();

        let result = os
            .poll_transaction_status(IntentHash::sample())
            .await
            .expect_err("Expected an error");

        assert_eq!(result, CommonError::NetworkResponseBadCode);
    }

    // Creates a `MockNetworkingDriver` that returns the given list of responses sequentially,
    // and then call `poll_transaction_status` to get the result.
    async fn simulate_poll_status(
        responses: Vec<TransactionStatusResponse>,
    ) -> TransactionStatus {
        let mock_driver = MockNetworkingDriver::with_responses(responses);

        let req = SUT::boot_test_with_networking_driver(Arc::new(mock_driver));

        let os =
            actix_rt::time::timeout(SARGON_OS_TEST_MAX_ASYNC_DURATION, req)
                .await
                .unwrap()
                .unwrap();

        os.poll_transaction_status(IntentHash::sample())
            .await
            .unwrap()
    }

    // Helper functions to create sample responses

    fn sample_unknown() -> TransactionStatusResponse {
        TransactionStatusResponse {
            known_payloads: vec![
                TransactionStatusResponsePayloadItem::sample_unknown(),
            ],
            ledger_state: LedgerState::sample_stokenet(),
            error_message: None,
        }
    }

    fn sample_pending() -> TransactionStatusResponse {
        TransactionStatusResponse {
            known_payloads: vec![
                TransactionStatusResponsePayloadItem::sample_pending(),
            ],
            ledger_state: LedgerState::sample_stokenet(),
            error_message: None,
        }
    }

    fn sample_commit_pending_outcome_unknown() -> TransactionStatusResponse {
        TransactionStatusResponse {
            known_payloads: vec![TransactionStatusResponsePayloadItem::sample_commit_pending_outcome_unknown()],
            ledger_state: LedgerState::sample_stokenet(),
            error_message: None,
        }
    }

    fn sample_committed_success() -> TransactionStatusResponse {
        TransactionStatusResponse {
            known_payloads: vec![
                TransactionStatusResponsePayloadItem::sample_committed_success(
                ),
            ],
            ledger_state: LedgerState::sample_stokenet(),
            error_message: None,
        }
    }

    fn sample_committed_failure(
        error_message: Option<String>,
    ) -> TransactionStatusResponse {
        TransactionStatusResponse {
            known_payloads: vec![
                TransactionStatusResponsePayloadItem::sample_committed_failure(
                ),
            ],
            ledger_state: LedgerState::sample_stokenet(),
            error_message,
        }
    }

    fn sample_permanently_rejected(
        error_message: Option<String>,
    ) -> TransactionStatusResponse {
        TransactionStatusResponse {
            known_payloads: vec![TransactionStatusResponsePayloadItem::sample_committed_permanently_rejected()],
            ledger_state: LedgerState::sample_stokenet(),
            error_message,
        }
    }

    fn sample_temporarily_rejected() -> TransactionStatusResponse {
        TransactionStatusResponse {
            known_payloads: vec![TransactionStatusResponsePayloadItem::sample_temporarily_rejected()],
            ledger_state: LedgerState::sample_stokenet(),
            error_message: None,
        }
    }
}
