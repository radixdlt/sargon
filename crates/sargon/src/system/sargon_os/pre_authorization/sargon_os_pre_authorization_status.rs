use std::time::Duration;
use crate::prelude::*;

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
        self
            .poll_transaction_status(transaction_intent_hash)
            .await
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
                return Ok((transaction_intent_hash, delays));
            } else {
                // Increase delay by 1 second on subsequent calls
                delay_duration += DELAY_INCREMENT;
                let sleep_duration = Duration::from_millis(delay_duration);

                delays.push(delay_duration);
                async_std::task::sleep(sleep_duration).await;
            }
        }
    }

    fn expiration_timestamp(&self, expiration: Option<DappToWalletInteractionSubintentExpiration>) -> Timestamp {
        match expiration {
            Some(expiration) => {
                match expiration {
                    DappToWalletInteractionSubintentExpiration::AtTime(at_time) => {
                        at_time.unix_timestamp_seconds
                    }
                    DappToWalletInteractionSubintentExpiration::AfterDelay(delay) => {
                        Timestamp::now_utc().add(Duration::from_secs(delay.expire_after_seconds))
                    }
                }
            },
            // If there is no expiation, we manually set it to expire after the max interval
            None => Timestamp::now_utc().add(Self::MAX_EXPIRATION_INTERVAL)
        }
    }

    // 1 hour
    const MAX_EXPIRATION_INTERVAL: Duration = Duration::from_secs(60 * 60);
}