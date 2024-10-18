use crate::prelude::*;

// ==================
// Submit Transaction
// ==================
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

#[cfg(test)]
mod tests {
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
