use crate::prelude::*;
use std::time::Duration;

// ==================
// Poll Transaction Status (Public)
// ==================
impl SargonOS {
    /// Polls the state of a Transaction until we can determine its `TransactionStatus`.
    pub async fn poll_transaction_status(
        &self,
        intent_hash: TransactionIntentHash,
    ) -> Result<TransactionStatus> {
        let (status, _) = self
            .poll_transaction_status_with_delays(intent_hash)
            .await?;
        Ok(status)
    }
}

// ==================
// Poll Transaction Status (Internal)
// ==================
impl SargonOS {
    /// Polls the state of a Transaction until we can determine its `TransactionStatus`.
    ///
    /// This is the internal implementation of `poll_transaction_status`, which is the public API.
    /// It returns the `TransactionStatus`, but also the list of delays between each poll.
    async fn poll_transaction_status_with_delays(
        &self,
        intent_hash: TransactionIntentHash,
    ) -> Result<(TransactionStatus, Vec<u64>)> {
        let network_id = self.current_network_id()?;
        let gateway_client = GatewayClient::new(
            self.clients.http_client.driver.clone(),
            network_id,
        );
        let mut delays: Vec<u64> = vec![];

        // The delay increment is set to 1 second in production, but 1 millisecond in tests.
        // This will make the tests run with almost no delay, while the production code will have a 2s delay after first call,
        // a 3s delay after second call, 4s after third and so on.
        #[cfg(test)]
        const DELAY_INCREMENT: u64 = 1;
        #[cfg(not(test))]
        const DELAY_INCREMENT: u64 = 1000;

        let mut delay_duration = DELAY_INCREMENT;

        loop {
            // Increase delay by 1 second on subsequent calls
            delay_duration += DELAY_INCREMENT;
            let sleep_duration = Duration::from_millis(delay_duration);

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
                            delays.push(delay_duration);
                            async_std::task::sleep(sleep_duration).await;
                        }
                        TransactionStatusResponsePayloadStatus::CommittedSuccess => {
                            return Ok((TransactionStatus::Success, delays));
                        }
                        TransactionStatusResponsePayloadStatus::CommittedFailure => {
                            return Ok((TransactionStatus::Failed { reason: TransactionStatusReason::from_raw_error(response.error_message) }, delays));
                        }
                        TransactionStatusResponsePayloadStatus::PermanentlyRejected => {
                            return Ok((TransactionStatus::PermanentlyRejected { reason: TransactionStatusReason::from_raw_error(response.error_message) }, delays));
                        }
                        TransactionStatusResponsePayloadStatus::TemporarilyRejected => {
                            return Ok((TransactionStatus::TemporarilyRejected { current_epoch: Epoch::from(response.ledger_state.epoch) }, delays));
                        }
                    }
                }
                None => {
                    delays.push(delay_duration);
                    async_std::task::sleep(sleep_duration).await;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
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

        // Result should be `Success`
        assert_eq!(result.0, TransactionStatus::Success);
        // and there shouldn't be any delays
        assert!(result.1.is_empty());
    }

    #[actix_rt::test]
    async fn poll_status_pending_then_failure() {
        // This test will simulate the case where the first response is empty (no payload status),
        // the second is `Pending` and the third is a `CommittedFailure`
        let result = simulate_poll_status(vec![
            sample_empty(),
            sample_pending(),
            sample_committed_failure(None),
        ])
        .await;

        // Result should be `Failed`
        assert_eq!(
            result.0,
            TransactionStatus::Failed {
                reason: TransactionStatusReason::Unknown
            }
        );
        // and there should have been a delay of 2s after first call, and 3s after the second call
        assert_eq!(result.1, vec![2, 3]);
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

        // Result should be `PermanentlyRejected`
        assert_eq!(
            result.0,
            TransactionStatus::PermanentlyRejected {
                reason: TransactionStatusReason::WorktopError
            }
        );
        // and there should have been a delay of 2s after first call
        assert_eq!(result.1, vec![2]);
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
        // Result should be `TemporarilyRejected`
        assert_eq!(
            result.0,
            TransactionStatus::TemporarilyRejected { current_epoch }
        );
        // and there should have been a delay of 2s after first call
        assert_eq!(result.1, vec![2]);
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
            .poll_transaction_status(TransactionIntentHash::sample())
            .await
            .expect_err("Expected an error");

        assert_eq!(result, CommonError::NetworkResponseBadCode);
    }

    // Creates a `MockNetworkingDriver` that returns the given list of responses sequentially,
    // and then call `poll_transaction_status` to get the result.
    async fn simulate_poll_status(
        responses: Vec<TransactionStatusResponse>,
    ) -> (TransactionStatus, Vec<u64>) {
        let mock_driver = MockNetworkingDriver::with_responses(responses);

        let req = SUT::boot_test_with_networking_driver(Arc::new(mock_driver));

        let os =
            actix_rt::time::timeout(SARGON_OS_TEST_MAX_ASYNC_DURATION, req)
                .await
                .unwrap()
                .unwrap();

        os.poll_transaction_status_with_delays(TransactionIntentHash::sample())
            .await
            .unwrap()
    }

    // Helper functions to create sample responses

    fn sample_empty() -> TransactionStatusResponse {
        TransactionStatusResponse {
            known_payloads: vec![],
            ledger_state: LedgerState::sample_stokenet(),
            error_message: None,
        }
    }

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
