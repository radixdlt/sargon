use crate::prelude::*;
use std::time::Duration;

// ==================
// Poll PreAuthorization Status (Public)
// ==================
impl SargonOS {
    /// Polls the state of a `PreAuthorization` until we can determine the parent Transaction's status.
    /// This means, we will first poll the subintent status, and once it has been submitted we
    /// will continue polling the
    pub async fn poll_pre_authorization_status(
        &self,
        intent_hash: SubintentHash,
        expiration: Option<DappToWalletInteractionSubintentExpiration>,
    ) -> Result<TransactionStatus> {
        // Poll until the subintent is submitted within a Transaction
        let (transaction_intent_hash, _) = self
            .poll_subintent_status_with_delays(intent_hash, expiration)
            .await?;

        // Poll the state of the Transaction
        self.poll_transaction_status(transaction_intent_hash).await
    }
}

// ==================
// Poll PreAuthorization Status (Internal)
// ==================
impl SargonOS {
    /// Polls the state of a Subintent until it is submitted
    ///
    /// It returns the `TransactionIntentHash`, but also the list of delays between each poll.
    async fn poll_subintent_status_with_delays(
        &self,
        intent_hash: SubintentHash,
        expiration: Option<DappToWalletInteractionSubintentExpiration>,
    ) -> Result<(TransactionIntentHash, Vec<u64>)> {
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
            // Check if the subinent hasn't expired already
            if Timestamp::now_utc() > expiration_timestamp {
                return Err(CommonError::ExpiredSubintent);
            }

            // Mock it to return TX intent on third call
            let mock = delay_duration > DELAY_INCREMENT * 2;

            let response = gateway_client
                .get_pre_authorization_status(intent_hash.clone(), mock)
                .await?;

            if let Some(transaction_intent_hash) = response {
                let result = TransactionIntentHash::from_bech32(
                    &transaction_intent_hash,
                )
                .map_err(|_| {
                    CommonError::FailedToDecodeTransactionHash {
                        bad_value: transaction_intent_hash,
                    }
                })?;
                return Ok((result, delays));
            } else {
                // Increase delay by 1 second on subsequent calls
                delay_duration += DELAY_INCREMENT;
                let sleep_duration = Duration::from_millis(delay_duration);

                delays.push(delay_duration);
                async_std::task::sleep(sleep_duration).await;
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
mod poll_pre_authorization_status_tests {
    use super::*;
    use actix_rt::time::timeout;
    use std::{future::Future, time::Duration};

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonOS;

    #[actix_rt::test]
    async fn test_subintent_submitted_and_tx_succeeds() {
        // This test will simulate the case where the subintent is submitted and the transaction is successful
        let os = mock_sargon_os(
            vec![Some("transaction_hash".to_owned())],
            TransactionStatusResponse::sample_committed_success(),
        );

        let result = os
            .await
            .poll_pre_authorization_status(SubintentHash::sample(), None)
            .await
            .unwrap();

        // TX result should be `Success`
        assert_eq!(result, TransactionStatus::Success);
    }

    #[actix_rt::test]
    async fn test_subintent_submitted_and_tx_fails() {
        // This test will simulate the case where the subintent is submitted but the transaction fails
        let os = mock_sargon_os(
            vec![Some("transaction_hash".to_owned())],
            TransactionStatusResponse::sample_permanently_rejected(None),
        );

        let result = os
            .await
            .poll_pre_authorization_status(SubintentHash::sample(), None)
            .await
            .unwrap();

        // TX result should be `PermanentlyRejected`
        assert_eq!(
            result,
            TransactionStatus::PermanentlyRejected {
                reason: TransactionStatusReason::Unknown
            }
        );
    }

    #[actix_rt::test]
    async fn test_subintent_expired() {
        // This test will simulate the case where the subintent expires and TX isn't submitted
        let os =
            mock_sargon_os(vec![], TransactionStatusResponse::sample_empty());

        // Set an expiration in the past
        let timestamp = Timestamp::now_utc().sub(Duration::from_secs(200));
        let expiration = DappToWalletInteractionSubintentExpiration::AtTime(
            timestamp.into(),
        );

        let result = os
            .await
            .poll_pre_authorization_status(
                SubintentHash::sample(),
                Some(expiration),
            )
            .await
            .expect_err("Expected an error");

        // TX result should be `Success`
        assert_eq!(result, CommonError::ExpiredSubintent);
    }

    // Returns a SargonOS with a `MockNetworkingDriver` that returns the given list of subintent status responses sequentially,
    // and then the corresponding TransactionStatusResponse.
    async fn mock_sargon_os(
        poll_subintent_responses: Vec<Option<String>>,
        poll_transaction_response: TransactionStatusResponse,
    ) -> Arc<SargonOS> {
        let mut responses: Vec<BagOfBytes> = poll_subintent_responses
            .iter()
            .map(|r| r.as_ref().map_or(BagOfBytes::default(), to_bag_of_bytes))
            .collect();

        responses.push(to_bag_of_bytes(poll_transaction_response));

        let mock_driver = MockNetworkingDriver::new_with_bodies(200, responses);

        let req = SUT::boot_test_with_networking_driver(Arc::new(mock_driver));

        let os =
            actix_rt::time::timeout(SARGON_OS_TEST_MAX_ASYNC_DURATION, req)
                .await
                .unwrap()
                .unwrap();

        os
    }

    fn to_bag_of_bytes<T>(value: T) -> BagOfBytes
    where
        T: Serialize,
    {
        BagOfBytes::from(serde_json::to_vec(&value).unwrap())
    }
}

#[cfg(test)]
mod poll_subintent_status_with_delays {
    use super::*;
    use actix_rt::time::timeout;
    use std::{future::Future, time::Duration};

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonOS;

    #[actix_rt::test]
    async fn tx_available_on_third_poll() {
        // This test will simulate the case where the first two polls don't return a result,
        // while the third one returns one
        let expected_tx = TransactionIntentHash::sample();

        let result = simulate_poll_status(
            vec![None, None, Some(expected_tx.clone().bech32_encoded_tx_id)],
            None,
        )
        .await
        .unwrap();

        // The result corresponds to the expected TX
        assert_eq!(result.0, expected_tx);

        // and there should have been a delay of 2s after first call, and 3s after the second call
        assert_eq!(result.1, vec![2, 3]);
    }

    #[actix_rt::test]
    async fn tx_wrong_hash() {
        // This test will simulate the case where the poll response has a wrong hash, that cannot be parsed
        // as a TransactionIntentHash

        let result =
            simulate_poll_status(vec![Some("wrong_tx_hash".to_owned())], None)
                .await
                .expect_err("Expected an error");

        // The result is an error with the wrong hash
        assert_eq!(
            result,
            CommonError::FailedToDecodeTransactionHash {
                bad_value: "wrong_tx_hash".to_owned()
            }
        );
    }

    #[actix_rt::test]
    async fn poll_status_error() {
        // This test will simulate the case where we fail to get subintent status from gateway.
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
    // and then call `poll_subintent_status_with_delays` with the given Expiration to get the result.
    async fn simulate_poll_status(
        responses: Vec<Option<String>>,
        expiration: Option<DappToWalletInteractionSubintentExpiration>,
    ) -> Result<(TransactionIntentHash, Vec<u64>)> {
        let mock_driver = MockNetworkingDriver::with_responses(responses);

        let req = SUT::boot_test_with_networking_driver(Arc::new(mock_driver));

        let os =
            actix_rt::time::timeout(SARGON_OS_TEST_MAX_ASYNC_DURATION, req)
                .await
                .unwrap()
                .unwrap();

        os.poll_subintent_status_with_delays(
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
