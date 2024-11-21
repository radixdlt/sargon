use crate::prelude::*;
use std::time::Duration;

// ==================
// Poll PreAuthorization Status (Public)
// ==================
impl SargonOS {
    /// Polls the status of a `SubintentHash` until it is either `Success` or `Expired`.
    pub async fn poll_pre_authorization_status(
        &self,
        intent_hash: SubintentHash,
        expiration: DappToWalletInteractionSubintentExpiration,
    ) -> Result<PreAuthorizationStatus> {
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
    /// It returns the `PreAuthorizationStatus`, but also the list of delays between each poll.
    async fn poll_pre_authorization_status_with_delays(
        &self,
        intent_hash: SubintentHash,
        expiration: DappToWalletInteractionSubintentExpiration,
    ) -> Result<(PreAuthorizationStatus, Vec<u64>)> {
        let network_id = self.current_network_id()?;
        let gateway_client = GatewayClient::new(
            self.clients.http_client.driver.clone(),
            network_id,
        );

        // We are going to play safe and leave an extra second to make sure we check the status one second after it has theoretically expired.
        // This is to avoid considering expired a subintent that got committed in the last instant.
        let seconds_until_expiration =
            self.seconds_until_expiration(expiration) + 1;
        let mut delays: Vec<u64> = vec![];

        let mut delay_duration = POLLING_DELAY_INCREMENT;

        loop {
            // Check the subintent status
            let response = gateway_client
                .subintent_status(SubintentStatusRequest::new(
                    intent_hash.to_string(),
                ))
                .await?;

            match response.subintent_status {
                SubintentStatus::CommittedSuccess => {
                    // If it has been committed, we consider it a `Success`.
                    let transaction_intent_hash = response
                        .finalized_at_transaction_intent_hash
                        .ok_or(CommonError::Unknown)?;
                    return Ok((
                        PreAuthorizationStatus::Success(
                            transaction_intent_hash,
                        ),
                        delays,
                    ));
                }
                SubintentStatus::Unknown => {
                    // If it is unknown, we need to determine whether it has expired, or if we need to add a delay and try again.

                    let accumulated_delay = delays.iter().sum::<u64>();
                    let has_expired =
                        accumulated_delay >= seconds_until_expiration;

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
                            delay_duration + POLLING_DELAY_INCREMENT;
                        let remaining_time =
                            seconds_until_expiration - accumulated_delay;
                        delay_duration = tentative_delay.min(remaining_time);
                        delays.push(delay_duration);

                        #[cfg(test)]
                        let sleep_duration =
                            Duration::from_millis(delay_duration);
                        #[cfg(not(test))]
                        let sleep_duration =
                            Duration::from_secs(delay_duration);

                        async_std::task::sleep(sleep_duration).await;
                    }
                }
            }
        }
    }

    /// Returns the remaining seconds until the subintent expires.
    fn seconds_until_expiration(
        &self,
        expiration: DappToWalletInteractionSubintentExpiration,
    ) -> u64 {
        match expiration {
            DappToWalletInteractionSubintentExpiration::AtTime(at_time) => {
                let current_time = seconds_since_unix_epoch();
                if at_time.unix_timestamp_seconds > current_time {
                    at_time.unix_timestamp_seconds - current_time
                } else {
                    0 // Avoid overflow in case we check after expiration
                }
            }
            DappToWalletInteractionSubintentExpiration::AfterDelay(delay) => {
                delay.expire_after_seconds
            }
        }
    }
}

#[cfg(test)]
mod poll_pre_authorization_status_with_delays {
    use super::*;
    use actix_rt::time::timeout;
    use std::{future::Future, time::Duration};

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonOS;
    #[allow(clippy::upper_case_acronyms)]
    type SSR = SubintentStatusResponse;

    #[actix_rt::test]
    async fn success_on_third_poll() {
        // This test will simulate the case where the first two polls return `Unknown`,
        // while the third one returns `CommittedSuccess`.
        let result = simulate_poll_status(
            2,
            SSR::sample_committed_success(),
            expiration_after_delay(10),
        )
        .await
        .unwrap();

        // The result is Success with the mocked TransactionIntentHash
        assert_eq!(
            result.0,
            PreAuthorizationStatus::Success(
                TransactionIntentHash::sample().to_string()
            )
        );

        // and there should have been a delay of 2s after first call, and 3s after the second call
        assert_eq!(result.1, vec![2, 3]);
    }

    #[actix_rt::test]
    async fn success_exactly_at_expiration() {
        // This test will simulate the case where the subintent expires in 9 seconds, and we only get
        // the `CommittedSuccess` on the fourth request (after 9 seconds of delay).
        let result = simulate_poll_status(
            3,
            SSR::sample_committed_success(),
            expiration_after_delay(9),
        )
        .await
        .unwrap();

        // The result is Success with the mocked TransactionIntentHash
        assert_eq!(
            result.0,
            PreAuthorizationStatus::Success(
                TransactionIntentHash::sample().to_string()
            )
        );

        // and delays should have been incrementing as expected until the last one (which is reduced to the remaining time)
        assert_eq!(result.1, vec![2, 3, 4]);
    }

    #[actix_rt::test]
    async fn success_immediately_after_expiration() {
        // This test will simulate the case where the subintent expires in 10 seconds, and we only get
        // the `CommittedSuccess` on the fifth request (after 11 seconds of delay).
        let result = simulate_poll_status(
            4,
            SSR::sample_committed_success(),
            expiration_after_delay(10),
        )
        .await
        .unwrap();

        // The result is Success with the mocked TransactionIntentHash
        assert_eq!(
            result.0,
            PreAuthorizationStatus::Success(
                TransactionIntentHash::sample().to_string()
            )
        );

        // and delays should have been incrementing as expected until the last one (which is reduced to the remaining time)
        assert_eq!(result.1, vec![2, 3, 4, 2]);
    }

    #[actix_rt::test]
    async fn expired() {
        // This test will simulate the case where the subintent expires in 10 seconds, and we only get
        // the `CommittedSuccess` on the sixth request (that is never made).
        let result = simulate_poll_status(
            5,
            SSR::sample_committed_success(),
            expiration_after_delay(10),
        )
        .await
        .unwrap();

        // The result is Success with the mocked TransactionIntentHash
        assert_eq!(result.0, PreAuthorizationStatus::Expired);

        // and delays should have been incrementing as expected until the last one (which is reduced to the remaining time)
        assert_eq!(result.1, vec![2, 3, 4, 2]);
    }

    #[actix_rt::test]
    async fn corrupt_success() {
        // This test will simulate the case where the GW returns a corrupted `CommittedSuccess` response
        // that is missing the TX id.
        let result = simulate_poll_status(
            0,
            SSR::sample_committed_success_corrupt(),
            expiration_after_delay(10),
        )
        .await
        .expect_err("Expected an error");

        // The result an Unknown error
        assert_eq!(result, CommonError::Unknown);
    }

    #[actix_rt::test]
    async fn failure() {
        // This test will simulate the case where the GW returns a failure response
        let mock_driver = MockNetworkingDriver::new_always_failing();
        let req = SUT::boot_test_with_networking_driver(Arc::new(mock_driver));

        let os =
            actix_rt::time::timeout(SARGON_OS_TEST_MAX_ASYNC_DURATION, req)
                .await
                .unwrap()
                .unwrap();

        let result = os
            .poll_pre_authorization_status_with_delays(
                SubintentHash::sample(),
                expiration_after_delay(10),
            )
            .await
            .expect_err("Expected an error");

        // The result an Unknown error
        assert_eq!(result, CommonError::NetworkResponseBadCode { code: 500 });
    }

    // Creates a `MockNetworkingDriver` that returns Unknown `SubintentStatusResponse` for the first `unknown_count` calls,
    // and then returns the given `last` response.
    // Also, calls `poll_pre_authorization_status_with_delays` with the given Expiration to get the result.
    async fn simulate_poll_status(
        unknown_count: u64,
        last: SubintentStatusResponse,
        expiration: DappToWalletInteractionSubintentExpiration,
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
            expiration,
        )
        .await
    }

    fn expiration_after_delay(
        seconds: u64,
    ) -> DappToWalletInteractionSubintentExpiration {
        DappToWalletInteractionSubintentExpiration::AfterDelay(seconds.into())
    }
}

#[cfg(test)]
mod seconds_until_expiration_tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonOS;

    #[actix_rt::test]
    async fn at_time() {
        let os = boot().await;

        // Test the case where the expiration is set to a specific time in the past
        let unix_seconds = 100;
        let expiration = DappToWalletInteractionSubintentExpiration::AtTime(
            unix_seconds.into(),
        );
        let result = os.seconds_until_expiration(expiration);
        assert_eq!(result, 0);

        // Test the case where the expiration is set to a specific time in the future
        let now = seconds_since_unix_epoch();
        let diff = 50;
        let expiration = DappToWalletInteractionSubintentExpiration::AtTime(
            (now + diff).into(),
        );

        let result = os.seconds_until_expiration(expiration);
        assert_eq!(result, diff);
    }

    #[actix_rt::test]
    async fn after_delay() {
        // This test will simulate the case where the expiration is set to a delay after the current time
        let delay = 100;
        let expiration = DappToWalletInteractionSubintentExpiration::AfterDelay(
            delay.into(),
        );

        let os = boot().await;
        let result = os.seconds_until_expiration(expiration);

        // The result should be the same as the delay
        assert_eq!(result, delay);
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
