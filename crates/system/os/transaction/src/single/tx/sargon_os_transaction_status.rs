use crate::prelude::*;
use std::time::Duration;

#[async_trait::async_trait]
pub trait OsTXStatusPolling {
    async fn poll_transaction_status(
        &self,
        intent_hash: TransactionIntentHash,
    ) -> Result<TransactionStatus>;

    async fn poll_transaction_status_with_delays(
        &self,
        intent_hash: TransactionIntentHash,
    ) -> Result<(TransactionStatus, Vec<u64>)>;
}

// ==================
// Poll Transaction Status (Public)
// ==================
#[async_trait::async_trait]
impl OsTXStatusPolling for SargonOS {
    /// Polls the state of a Transaction until we can determine its `TransactionStatus`.
    async fn poll_transaction_status(
        &self,
        intent_hash: TransactionIntentHash,
    ) -> Result<TransactionStatus> {
        self.poll_transaction_status_with_delays(intent_hash)
            .await
            .map(|(status, _)| status)
    }

    // ==================
    // Poll Transaction Status (Internal)
    // ==================
    /// Polls the state of a Transaction until we can determine its `TransactionStatus`.
    ///
    /// This is the internal implementation of `poll_transaction_status`, which is the public API.
    /// It returns the `TransactionStatus`, but also the list of delays between each poll.
    async fn poll_transaction_status_with_delays(
        &self,
        intent_hash: TransactionIntentHash,
    ) -> Result<(TransactionStatus, Vec<u64>)> {
        let gateway_client = self.gateway_client()?;
        let mut delays: Vec<u64> = vec![];
        let mut delay_duration = POLLING_DELAY_INCREMENT_IN_SECONDS;

        loop {
            // Increase delay by 1 second on subsequent calls
            delay_duration += POLLING_DELAY_INCREMENT_IN_SECONDS;

            #[cfg(test)]
            let sleep_duration = Duration::from_millis(delay_duration); // make it faster for tests
            #[cfg(not(test))]
            let sleep_duration = Duration::from_secs(delay_duration);

            let response = match gateway_client
                .get_transaction_status(intent_hash.clone())
                .await
            {
                Ok(response) => response,
                Err(_) => {
                    delays.push(delay_duration);
                    async_std::task::sleep(sleep_duration).await;
                    continue;
                }
            };

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

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonOS;
    #[allow(clippy::upper_case_acronyms)]
    type TSR = TransactionStatusResponse;

    #[actix_rt::test]
    async fn poll_status_success() {
        // This test will simulate the case where the first response is a `CommittedSuccess`
        let result =
            simulate_poll_status(vec![TSR::sample_committed_success()]).await;

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
            TSR::sample_empty(),
            TSR::sample_pending(),
            TSR::sample_committed_failure(None),
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
            TSR::sample_unknown(),
            TSR::sample_permanently_rejected(Some(
                "AssertionFailed".to_owned(),
            )),
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
            TSR::sample_commit_pending_outcome_unknown(),
            TSR::sample_temporarily_rejected(),
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
    async fn poll_status_error_then_success() {
        // This test will simulate the case where we fail to get a `TransactionStatusResponse` from gateway on the first response,
        // while the second response is a `CommittedSuccess`.
        let responses = vec![
            MockNetworkingDriverResponse::new_failing(),
            MockNetworkingDriverResponse::new_success(
                TSR::sample_committed_success(),
            ),
        ];

        let mock_driver = MockNetworkingDriver::new_with_responses(responses);

        let req = SUT::boot_test_with_networking_driver(Arc::new(mock_driver));

        let os =
            actix_rt::time::timeout(SARGON_OS_TEST_MAX_ASYNC_DURATION, req)
                .await
                .unwrap()
                .unwrap();

        let result =
            os.poll_transaction_status_with_delays(
                TransactionIntentHash::sample(),
            )
            .await
            .unwrap();

        // Result should be `Success`
        assert_eq!(result.0, TransactionStatus::Success);
        // and there should have been a delay of 2s after first call
        assert_eq!(result.1, vec![2]);
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
}
