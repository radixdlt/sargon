use crate::prelude::*;
use std::time::Duration;

// ==================
// Poll PreAuthorization Status (Public)
// ==================
impl SargonOS {
    /// Polls the state of a PreAuthorization until we can determine its `TransactionStatus`.
    pub async fn poll_pre_authorization_status(
        &self,
        intent_hash: SubintentHash,
        expiration: Option<DappToWalletInteractionSubintentExpiration>,
    ) -> Result<TransactionStatus> {
        let (status, _) = self
            .poll_pre_authorization_status_with_delays(intent_hash, expiration)
            .await?;

        Ok(status)
    }
}

// ==================
// Poll PreAuthorization Status (Internal)
// ==================
impl SargonOS {
    /// Polls the state of a Subintent until it is submitted
    ///
    /// It returns the `TransactionIntentHash`, but also the list of delays between each poll.
    async fn poll_pre_authorization_status_with_delays(
        &self,
        intent_hash: SubintentHash,
        expiration: Option<DappToWalletInteractionSubintentExpiration>,
    ) -> Result<(TransactionStatus, Vec<u64>)> {
        let network_id = self.current_network_id()?;
        let gateway_client = GatewayClient::new(
            self.clients.http_client.driver.clone(),
            network_id,
        );

        let expiration_timestamp = self.expiration_timestamp(expiration);

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

            // Check if the subinent has already expired. We subtract the `sleep_duration` to consider the case
            // where the subinent was submitted after last poll, but the `expiration_timestamp` was also reached before this poll.
            // We must check the subinent status before assuming it had expired.
            if Timestamp::now_utc().sub(sleep_duration) > expiration_timestamp {
                return Err(CommonError::ExpiredSubintent);
            }

            // Mock it to return TransactionStatus on third call
            let mock_success = delay_duration > DELAY_INCREMENT * 2;

            let response = match gateway_client
                .get_pre_authorization_status(intent_hash.clone(), mock_success)
                .await
            {
                Ok(response) => response,
                Err(_) => {
                    delays.push(delay_duration);
                    async_std::task::sleep(sleep_duration).await;
                    continue;
                }
            };

            // Note: This logic is equal to the one under poll_transaction_status_with_delays()
            // I am not spending any time in refactoring it, as we still don't know how the actual polling
            // status endpoint will look like, and we are mocking the response for now.
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

    /// Returns the timestamp when the subintent should expire/timeout.
    fn expiration_timestamp(
        &self,
        expiration: Option<DappToWalletInteractionSubintentExpiration>,
    ) -> Timestamp {
        match expiration {
            Some(expiration) => match expiration {
                DappToWalletInteractionSubintentExpiration::AtTime(at_time) => {
                    at_time.unix_timestamp_seconds
                }
                DappToWalletInteractionSubintentExpiration::AfterDelay(
                    delay,
                ) => Timestamp::now_utc()
                    .add(Duration::from_secs(delay.expire_after_seconds)),
            },
            // If there is no expiration, we manually set it to expire after the timeout
            None => Timestamp::now_utc().add(Self::MAX_EXPIRATION_TIMEOUT),
        }
    }

    // 1 hour
    const MAX_EXPIRATION_TIMEOUT: Duration = Duration::from_secs(60 * 60);
}

#[cfg(test)]
mod poll_pre_authorization_status_with_delays {
    use super::*;
    use actix_rt::time::timeout;
    use std::{future::Future, time::Duration};

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonOS;
    #[allow(clippy::upper_case_acronyms)]
    type TSR = TransactionStatusResponse;

    // Uncomment once test actually use Networking Driver
    /*
    #[actix_rt::test]
    async fn success_on_third_poll() {
        // This test will simulate the case where the first two polls return an empty result,
        // while the third one returns a committed success.
        let result = simulate_poll_status(
            vec![
                TSR::sample_empty(),
                TSR::sample_empty(),
                TSR::sample_committed_success(),
            ],
            None,
        )
        .await
            .unwrap();

        // The result corresponds to the expected TX
        assert_eq!(result.0, TransactionStatus::Success);

        // and there should have been a delay of 2s after first call, and 3s after the second call
        assert_eq!(result.1, vec![2, 3]);
    }

    #[actix_rt::test]
    async fn pending_then_failure() {
        // This test will simulate the case where the first response is empty (no payload status),
        // the second is `Pending` and the third is a `CommittedFailure`
        let result = simulate_poll_status(
            vec![
                TSR::sample_empty(),
                TSR::sample_pending(),
                TSR::sample_committed_failure(None),
            ],
            None,
        )
        .await
            .unwrap();

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
    async fn unknown_then_permanently_rejected() {
        // This test will simulate the case where the first response is `Unknown`,
        // while the second response is a `PermanentlyRejected`
        let result = simulate_poll_status(
            vec![
                TSR::sample_unknown(),
                TSR::sample_permanently_rejected(Some(
                    "AssertionFailed".to_owned(),
                )),
            ],
            None,
        )
        .await
            .unwrap();

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
    async fn commit_pending_outcome_unknown_then_temporarily_rejected(
    ) {
        // This test will simulate the case where the first response is `Unknown`,
        // while the second response is a `PermanentlyRejected`
        let result = simulate_poll_status(
            vec![
                TSR::sample_commit_pending_outcome_unknown(),
                TSR::sample_temporarily_rejected(),
            ],
            None,
        )
        .await
            .unwrap();

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
    async fn error_then_success() {
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

        let result = os
            .poll_pre_authorization_status_with_delays(SubintentHash::sample(), None)
            .await
            .unwrap();

        // Result should be `Success`
        assert_eq!(result.0, TransactionStatus::Success);
        // and there should have been a delay of 2s after first call
        assert_eq!(result.1, vec![2]);
    }
     */

    #[actix_rt::test]
    async fn expired_subintent() {
        // This test will simulate the subintent has expired at the moment of polling.
        let timestamp = Timestamp::now_utc().sub(Duration::from_secs(100));
        let expiration = DappToWalletInteractionSubintentExpiration::AtTime(
            timestamp.into(),
        );

        let result = simulate_poll_status(vec![], Some(expiration))
            .await
            .expect_err("Expected an error");

        assert_eq!(result, CommonError::ExpiredSubintent);
    }

    // Creates a `MockNetworkingDriver` that returns the given list of responses sequentially,
    // and then call `poll_pre_authorization_status_with_delays` with the given Expiration to get the result.
    async fn simulate_poll_status(
        responses: Vec<TransactionStatusResponse>,
        expiration: Option<DappToWalletInteractionSubintentExpiration>,
    ) -> Result<(TransactionStatus, Vec<u64>)> {
        let mock_driver = MockNetworkingDriver::with_responses(responses);

        let req = SUT::boot_test_with_networking_driver(Arc::new(mock_driver));

        let os =
            actix_rt::time::timeout(SARGON_OS_TEST_MAX_ASYNC_DURATION, req)
                .await
                .unwrap()
                .unwrap();

        os.poll_pre_authorization_status_with_delays(
            SubintentHash::sample(),
            expiration,
        )
        .await
    }
}

#[cfg(test)]
mod expiration_timestamp_tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonOS;

    #[actix_rt::test]
    async fn test_expiration_timestamp_at_time() {
        // This test will simulate the case where the expiration is set to a specific time
        let timestamp = Timestamp::now_utc().add(Duration::from_secs(100));
        let expiration = DappToWalletInteractionSubintentExpiration::AtTime(
            timestamp.into(),
        );

        let os = boot().await;
        let result = os.expiration_timestamp(Some(expiration));

        // The result should be the same as the timestamp
        assert_eq!(result, timestamp);
    }

    #[actix_rt::test]
    async fn test_expiration_timestamp_after_delay() {
        // This test will simulate the case where the expiration is set to a delay after the current time
        let delay = 100;
        let expiration = DappToWalletInteractionSubintentExpiration::AfterDelay(
            delay.into(),
        );

        let os = boot().await;
        let result = os.expiration_timestamp(Some(expiration));

        // The result should be the current time plus the delay, which we will verify
        // by checking that the diff is less than 0.1 s
        let expected = Timestamp::now_utc().add(Duration::from_secs(delay));
        let diff = result.duration_since(expected);
        assert!(diff < Duration::from_millis(100));
    }

    #[actix_rt::test]
    async fn test_expiration_timestamp_no_expiration() {
        // This test will simulate the case where there is no expiration set
        let os = boot().await;
        let result = os.expiration_timestamp(None);

        // The result should be the current time plus the default expiration timeout
        let expected = Timestamp::now_utc().add(SUT::MAX_EXPIRATION_TIMEOUT);
        let diff = result.duration_since(expected);
        assert!(diff < Duration::from_millis(100));
    }

    async fn boot() -> Arc<SargonOS> {
        let req = SUT::boot_test_with_networking_driver(Arc::new(
            MockNetworkingDriver::new_always_failing(),
        ));

        actix_rt::time::timeout(SARGON_OS_TEST_MAX_ASYNC_DURATION, req)
            .await
            .unwrap()
            .unwrap()
    }
}
