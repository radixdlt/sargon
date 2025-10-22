use crate::prelude::*;
use std::time::Duration;

#[async_trait::async_trait]
pub trait OSPollPreAuthorizationStatus {
    async fn poll_pre_authorization_status(
        &self,
        intent_hash: SubintentHash,
        expiration_timestamp: Instant,
    ) -> Result<PreAuthorizationStatus>;
}

// ==================
// Poll PreAuthorization Status (Public)
// ==================
#[async_trait::async_trait]
impl OSPollPreAuthorizationStatus for SargonOS {
    /// Polls the status of a `SubintentHash` until it is either `Success` or `Expired`.
    async fn poll_pre_authorization_status(
        &self,
        intent_hash: SubintentHash,
        expiration_timestamp: Instant,
    ) -> Result<PreAuthorizationStatus> {
        let seconds_until_expiration =
            self.seconds_until_expiration(expiration_timestamp);
        self.poll_pre_authorization_status_with_delays(
            intent_hash,
            seconds_until_expiration,
        )
        .await
        .map(|(status, _)| status)
    }
}

#[async_trait::async_trait]
pub trait OSPollPreAuthorizationStatusWithDelays {
    fn seconds_until_expiration(&self, expiration_timestamp: Instant) -> u64;
    async fn poll_pre_authorization_status_with_delays(
        &self,
        intent_hash: SubintentHash,
        seconds_until_expiration: u64,
    ) -> Result<(PreAuthorizationStatus, Vec<u64>)>;
}

// ==================
// Poll PreAuthorization Status (Internal)
// ==================
#[async_trait::async_trait]
impl OSPollPreAuthorizationStatusWithDelays for SargonOS {
    /// Polls the state of a Subintent until it is submitted
    ///
    /// It returns the `PreAuthorizationStatus`, but also the list of delays between each poll.
    async fn poll_pre_authorization_status_with_delays(
        &self,
        intent_hash: SubintentHash,
        seconds_until_expiration: u64,
    ) -> Result<(PreAuthorizationStatus, Vec<u64>)> {
        let gateway_client = self.gateway_client()?;

        // We are going to play safe and leave an extra second to make sure we check the status one second after it has theoretically expired.
        // This is to avoid considering expired a subintent that got committed in the last instant.
        let seconds_until_expiration = seconds_until_expiration + 1;
        let mut delays = Vec::new();

        let mut delay_duration = POLLING_DELAY_INCREMENT_IN_SECONDS;

        loop {
            // Check the subintent status and default to `Unknown` if the request fails.
            let (subintent_status, transaction_intent_hash) =
                match gateway_client
                    .subintent_status(SubintentStatusRequest::new(
                        intent_hash.to_string(),
                    ))
                    .await
                {
                    Ok(response) => (
                        response.subintent_status,
                        response.finalized_at_transaction_intent_hash,
                    ),
                    Err(_) => (SubintentStatus::Unknown, None),
                };

            match subintent_status {
                SubintentStatus::CommittedSuccess => {
                    // If it has been committed, we consider it a `Success`.
                    let intent_hash =
                        match transaction_intent_hash {
                            Some(hash) => {
                                TransactionIntentHash::from_bech32(&hash)?
                            }
                            None => return Err(CommonError::Unknown {
                                error_message:
                                    "Failed mapping sub-intent transaction hash"
                                        .to_string(),
                            }),
                        };
                    return Ok((
                        PreAuthorizationStatus::Success { intent_hash },
                        delays,
                    ));
                }
                SubintentStatus::Unknown => {
                    // If it is unknown, we need to determine whether it has expired, or if we need to add a delay and try again.
                }
            }

            // Check the accumulated delay to determine if the subintent has expired
            let accumulated_delay = delays.iter().sum::<u64>();
            let has_expired = accumulated_delay >= seconds_until_expiration;

            if has_expired {
                // If it has expired, we return the corresponding status.
                return Ok((PreAuthorizationStatus::Expired, delays));
            } else {
                // Otherwise, we determine the delay before next call.
                // This will either be the default delay or the remaining time until expiration (whatever is less).
                // Example: We have already polled 4 times for a subintent that expires after 10 seconds.
                // Seconds until expiration = 10 + 1 = 11
                // Accumulated delay = 0 + 2 + 3 + 4 = 9
                // Next polling should be in 5 seconds, but instead we will make it in 2 to check immediately after expiration.

                let tentative_delay =
                    delay_duration + POLLING_DELAY_INCREMENT_IN_SECONDS;
                let remaining_time =
                    seconds_until_expiration - accumulated_delay;
                delay_duration = tentative_delay.min(remaining_time);
                delays.push(delay_duration);

                #[cfg(test)]
                let sleep_duration = Duration::from_millis(delay_duration);
                #[cfg(not(test))]
                let sleep_duration = Duration::from_secs(delay_duration);

                async_std::task::sleep(sleep_duration).await;
            }
        }
    }

    /// Returns the remaining seconds until the subintent expires.
    fn seconds_until_expiration(&self, expiration_timestamp: Instant) -> u64 {
        let expiration = expiration_timestamp.seconds_since_unix_epoch as u64;
        let now = seconds_since_unix_epoch();
        expiration.saturating_sub(now)
    }
}

#[cfg(test)]
mod poll_pre_authorization_status_with_delays {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonOS;

    #[allow(clippy::upper_case_acronyms)]
    type SSR = SubintentStatusResponse;

    #[actix_rt::test]
    async fn success_on_third_poll() {
        // This test will simulate the case where the first two polls return `Unknown`,
        // while the third one returns `CommittedSuccess`.

        let intent_hash = TransactionIntentHash::sample_other();
        let result = simulate_poll_status(
            2,
            SSR::committed_success(intent_hash.to_string()),
            10,
        )
        .await
        .unwrap();

        // The result is Success with the expected TransactionIntentHash
        assert_eq!(result.0, PreAuthorizationStatus::Success { intent_hash });

        // and there should have been a delay of 2s after first call, and 3s after the second call
        assert_eq!(result.1, vec![2, 3]);
    }

    #[actix_rt::test]
    async fn success_exactly_at_expiration() {
        // This test will simulate the case where the subintent expires in 9 seconds, and we only get
        // the `CommittedSuccess` on the fourth request (after 9 seconds of delay).
        let result =
            simulate_poll_status(3, SSR::sample_committed_success(), 9)
                .await
                .unwrap();

        // The result is Success
        assert!(matches!(result.0, PreAuthorizationStatus::Success { .. }));

        // and delays should have been incrementing as expected until the last one (which is reduced to the remaining time)
        assert_eq!(result.1, vec![2, 3, 4]);
    }

    #[actix_rt::test]
    async fn success_immediately_after_expiration() {
        // This test will simulate the case where the subintent expires in 10 seconds, and we only get
        // the `CommittedSuccess` on the fifth request (after 11 seconds of delay).
        let result =
            simulate_poll_status(4, SSR::sample_committed_success(), 10)
                .await
                .unwrap();

        // The result is Success
        assert!(matches!(result.0, PreAuthorizationStatus::Success { .. }));

        // and delays should have been incrementing as expected until the last one (which is reduced to the remaining time)
        assert_eq!(result.1, vec![2, 3, 4, 2]);
    }

    #[actix_rt::test]
    async fn expired() {
        // This test will simulate the case where the subintent expires in 10 seconds, and we only get
        // the `CommittedSuccess` on the sixth request (that is never made).
        let result =
            simulate_poll_status(5, SSR::sample_committed_success(), 10)
                .await
                .unwrap();

        // The result is Expired
        assert_eq!(result.0, PreAuthorizationStatus::Expired);

        // and delays should have been incrementing as expected until the last one (which is reduced to the remaining time)
        assert_eq!(result.1, vec![2, 3, 4, 2]);
    }

    #[actix_rt::test]
    async fn success_without_transaction_intent_hash() {
        // This test will simulate the case where the GW returns a corrupted `CommittedSuccess` response
        // that is missing the TX id.
        let result = simulate_poll_status(0, SSR::committed_success(None), 10)
            .await
            .expect_err("Expected an error");

        // The result an Unknown error
        assert_eq!(
            result,
            CommonError::Unknown {
                error_message: "Failed mapping sub-intent transaction hash"
                    .to_string()
            }
        );
    }

    #[actix_rt::test]
    async fn success_with_invalid_transaction_intent_hash() {
        // This test will simulate the case where the GW returns a corrupted `CommittedSuccess` response
        // that is missing the TX id.
        let result = simulate_poll_status(
            0,
            SSR::committed_success("not an intent hash".to_string()),
            10,
        )
        .await
        .expect_err("Expected an error");

        // The result an Unknown error
        assert_eq!(result, CommonError::FailedToBech32DecodeTransactionHashAfterHavingTestedAllNetworkID { bad_value: "not an intent hash".to_string() });
    }

    #[actix_rt::test]
    async fn failure_then_success() {
        // This test will simulate the case where the GW returns a failure response on first two polls,
        // while the third response is a `CommittedSuccess`
        let responses = vec![
            MockNetworkingDriverResponse::new_failing(),
            MockNetworkingDriverResponse::new_failing(),
            MockNetworkingDriverResponse::new_success(
                SSR::sample_committed_success(),
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
            .poll_pre_authorization_status_with_delays(
                SubintentHash::sample(),
                10,
            )
            .await
            .unwrap();

        // The result is Success
        assert!(matches!(result.0, PreAuthorizationStatus::Success { .. }));

        // and delays should have been 2 and 3 seconds
        assert_eq!(result.1, vec![2, 3]);
    }

    // Creates a `MockNetworkingDriver` that returns Unknown `SubintentStatusResponse` for the first `unknown_count` calls,
    // and then returns the given `last` response.
    // Also, calls `poll_pre_authorization_status_with_delays` with the given Expiration to get the result.
    async fn simulate_poll_status(
        unknown_count: u64,
        last: SubintentStatusResponse,
        seconds_until_expiration: u64,
    ) -> Result<(PreAuthorizationStatus, Vec<u64>)> {
        let mut responses = vec![SSR::sample_unknown(); unknown_count as usize];
        responses.push(last);
        let mock_driver = MockNetworkingDriver::with_responses(responses);

        let req = SUT::boot_test_with_networking_driver(Arc::new(mock_driver));

        let os =
            actix_rt::time::timeout(SARGON_OS_TEST_MAX_ASYNC_DURATION, req)
                .await
                .unwrap()
                .unwrap();

        os.poll_pre_authorization_status_with_delays(
            SubintentHash::sample(),
            seconds_until_expiration,
        )
        .await
    }
}

#[cfg(test)]
mod seconds_until_expiration_tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonOS;

    #[actix_rt::test]
    async fn seconds_until() {
        let os = boot().await;

        let seconds = 100;
        // Test the case where the expiration is set to a specific time in the past
        let expiration_timestamp =
            Timestamp::now_utc().sub(Duration::from_secs(seconds));
        let result = os.seconds_until_expiration(expiration_timestamp.into());
        assert_eq!(result, 0);

        // Test the case where the expiration is set to a specific time in the future
        let expiration_timestamp =
            Timestamp::now_utc().add(Duration::from_secs(seconds));
        let result = os.seconds_until_expiration(expiration_timestamp.into());
        assert!(seconds - result <= 1); // Less than 1s difference, needed since the test is not instant and 1s may have passed.
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
