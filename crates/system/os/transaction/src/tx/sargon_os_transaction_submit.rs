use crate::prelude::*;

#[async_trait::async_trait]
pub trait OsTxSubmitting {
    async fn submit_transaction(
        &self,
        notarized_transaction: NotarizedTransaction,
    ) -> Result<TransactionIntentHash>;
}

// ==================
// Submit Transaction
// ==================
#[async_trait::async_trait]
impl OsTxSubmitting for SargonOS {
    /// Submits a notarized transaction payload to the network.
    async fn submit_transaction(
        &self,
        notarized_transaction: NotarizedTransaction,
    ) -> Result<TransactionIntentHash> {
        let gateway_client = self.gateway_client()?;

        gateway_client
            .submit_notarized_transaction(notarized_transaction)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonOS;

    #[actix_rt::test]
    async fn submit_transaction_success() {
        let notarized_transaction = NotarizedTransaction::sample();
        let response = TransactionSubmitResponse { duplicate: false };
        let body = serde_json::to_vec(&response).unwrap();

        let mock_driver =
            MockNetworkingDriver::with_spy(200, body, |request, _| {
                // Verify the body sent matches the expected one
                let sent_request = TransactionSubmitRequest::new(
                    NotarizedTransaction::sample(),
                );
                let sent_body = serde_json::to_vec(&sent_request).unwrap();

                assert_eq!(request.body.to_vec(), sent_body);
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

        let expected_result = notarized_transaction
            .signed_intent()
            .intent()
            .transaction_intent_hash();

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

        assert_eq!(result, CommonError::NetworkResponseBadCode { code: 500 });
    }
}
