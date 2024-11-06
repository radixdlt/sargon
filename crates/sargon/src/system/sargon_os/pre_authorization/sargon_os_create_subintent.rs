use crate::prelude::*;
use std::time::Duration;

// ==================
// Create Subintent
// ==================
impl SargonOS {
    /// Creates a Subintent given its discriminator, manifest and expiration.
    pub async fn create_subintent(
        &self,
        intent_discriminator: IntentDiscriminator,
        subintent_manifest: SubintentManifest,
        expiration: DappToWalletInteractionSubintentExpiration,
    ) -> Result<Subintent> {
        let network_id = self.current_network_id()?;
        let gateway_client = GatewayClient::new(
            self.clients.http_client.driver.clone(),
            network_id,
        );
        let current_epoch = gateway_client.current_epoch().await?;

        // Calculate the seconds until the expiration of the subintent.
        let expiry_time_from_now_in_seconds =
            self.expiry_time_from_now_in_seconds(expiration);
        if expiry_time_from_now_in_seconds == 0 {
            return Err(CommonError::SubintentExpired);
        }

        // Calculate header ranges
        let end_ranges = self.calculate_end_ranges(
            current_epoch,
            expiry_time_from_now_in_seconds,
        );

        // Build header
        let header = IntentHeaderV2 {
            network_id,
            start_epoch_inclusive: current_epoch,
            end_epoch_exclusive: end_ranges.0,
            min_proposer_timestamp_inclusive: None,
            max_proposer_timestamp_exclusive: Some(end_ranges.1),
            intent_discriminator,
        };

        // Build subintent
        Subintent::new(header, subintent_manifest, MessageV2::None)
    }

    /// Returns the seconds until the expiration of the subintent.
    fn expiry_time_from_now_in_seconds(
        &self,
        expiration: DappToWalletInteractionSubintentExpiration,
    ) -> u64 {
        match expiration {
            DappToWalletInteractionSubintentExpiration::AtTime(at_time) => {
                at_time
                    .unix_timestamp_seconds
                    .duration_since(Timestamp::now_utc())
                    .as_seconds_f64() as u64
            }
            DappToWalletInteractionSubintentExpiration::AfterDelay(delay) => {
                delay.expire_after_seconds
            }
        }
    }

    /// Given the current epoch and seconds until expiration, returns the end epoch and the max proposer timestamp.
    fn calculate_end_ranges(
        &self,
        current_epoch: Epoch,
        expiry_time_from_now_in_seconds: u64,
    ) -> (Epoch, Instant) {
        // 5 minutes
        const EXPECTED_EPOCH_DURATION_IN_SECONDS: u64 = 300;

        // As per the transaction validation configuration, epoch diff should be less than 1 month.
        const MAX_EPOCH_DIFF: u64 =
            30 * 24 * 60 * 60 / EXPECTED_EPOCH_DURATION_IN_SECONDS;

        // 1 epoch for the fact that it's min_inclusive and max_exclusive; 1 more for the fact that we might be very close to the end of the epoch already
        const MIN_EPOCH_DIFF: u64 = 2;

        let epoch_diff = MAX_EPOCH_DIFF.min(
            MIN_EPOCH_DIFF
                + (expiry_time_from_now_in_seconds
                    / EXPECTED_EPOCH_DURATION_IN_SECONDS),
        );
        let end_epoch_exclusive = current_epoch.adding(epoch_diff);
        let max_proposer_timestamp_exclusive = Timestamp::now_utc()
            .add(Duration::from_secs(expiry_time_from_now_in_seconds));
        (end_epoch_exclusive, max_proposer_timestamp_exclusive.into())
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
    async fn create_subintent_expired() {
        // Test the case where the subintent is already expired
        let os = boot_success().await;

        let timestamp = Timestamp::now_utc().sub(Duration::from_secs(100));
        let expiration = DappToWalletInteractionSubintentExpiration::AtTime(
            timestamp.into(),
        );

        let result = os
            .create_subintent(
                IntentDiscriminator::sample(),
                SubintentManifest::sample(),
                expiration,
            )
            .await
            .expect_err("Expected an error");

        assert_eq!(result, CommonError::SubintentExpired);
    }

    #[actix_rt::test]
    async fn create_subintent_valid() {
        // Test the case where the subintent is valid
        let os = boot_success().await;

        let expiration = DappToWalletInteractionSubintentExpiration::AfterDelay(
            DappToWalletInteractionSubintentExpireAfterDelay::from(600),
        );

        let intent_discriminator = IntentDiscriminator::sample();
        let manifest = SubintentManifest::sample();

        let result = os
            .create_subintent(
                intent_discriminator,
                manifest.clone(),
                expiration,
            )
            .await
            .expect("Expected a valid subintent");

        let max_timestamp = Timestamp::now_utc().add(Duration::from_secs(600));

        assert_eq!(result.header.network_id, NetworkID::Mainnet);
        assert_eq!(result.header.start_epoch_inclusive, Epoch(41965));
        assert_eq!(result.header.end_epoch_exclusive, Epoch(41969));
        assert_eq!(result.header.min_proposer_timestamp_inclusive, None);
        assert_eq!(
            result.header.max_proposer_timestamp_exclusive,
            Some(max_timestamp.into())
        );
        assert_eq!(result.header.intent_discriminator, intent_discriminator);
        assert_eq!(result.manifest, manifest);
        assert_eq!(result.message, MessageV2::None);
    }

    #[actix_rt::test]
    async fn expiry_time() {
        let os = boot_always_failing().await;

        // Check for AtTime expiration
        let seconds = 32423;
        let timestamp = Timestamp::now_utc().add(Duration::from_secs(seconds));
        let mut expiration = DappToWalletInteractionSubintentExpiration::AtTime(
            DappToWalletInteractionSubintentExpireAtTime::from(timestamp),
        );

        let mut result = os.expiry_time_from_now_in_seconds(expiration);
        let mut diff = seconds.abs_diff(result);
        assert!(diff <= 1); // Less than 1s difference

        // Check for AfterDelay expiration
        expiration = DappToWalletInteractionSubintentExpiration::AfterDelay(
            DappToWalletInteractionSubintentExpireAfterDelay::from(seconds),
        );
        result = os.expiry_time_from_now_in_seconds(expiration);
        assert_eq!(result, seconds);
    }

    #[actix_rt::test]
    async fn end_ranges_epoch() {
        let os = boot_always_failing().await;

        // Test the case where expiration is in 30 seconds (less than 1 epoch)
        let current_epoch = Epoch::from(1);
        let mut expiry_time = 30;
        let mut result = os.calculate_end_ranges(current_epoch, expiry_time);
        assert_eq!(result.0, Epoch::from(3));

        // Test the case where expiration is in 15 minutes
        expiry_time = 15 * 60;
        result = os.calculate_end_ranges(current_epoch, expiry_time);
        assert_eq!(result.0, Epoch::from(6));

        // Test the case where expiration is in 2 months
        let one_month_in_seconds = 30 * 24 * 60 * 60;
        let one_month_epoch = one_month_in_seconds / 300;
        expiry_time = 2 * one_month_in_seconds;
        result = os.calculate_end_ranges(current_epoch, expiry_time);
        assert_eq!(result.0, current_epoch.adding(one_month_epoch));
    }

    #[actix_rt::test]
    async fn end_ranges_instant() {
        let os = boot_always_failing().await;

        let current_epoch = Epoch::from(1);
        let expiry_time = 30;
        let result = os.calculate_end_ranges(current_epoch, expiry_time);
        let expected: Instant = Timestamp::now_utc()
            .add(Duration::from_secs(expiry_time))
            .into();
        let diff = (result.1.seconds_since_unix_epoch
            - expected.seconds_since_unix_epoch)
            .abs();
        assert!(diff <= 1); // Less than 1s difference
    }

    /// Boots a SargonOS that returns `Epoch(41965)` when asking for current epoch.
    async fn boot_success() -> Arc<SargonOS> {
        let response = TransactionConstructionResponse {
            ledger_state: LedgerState::sample_stokenet(),
        };
        let mock_driver = MockNetworkingDriver::with_responses(vec![response]);

        let req = SUT::boot_test_with_networking_driver(Arc::new(mock_driver));

        timeout(SARGON_OS_TEST_MAX_ASYNC_DURATION, req)
            .await
            .unwrap()
            .unwrap()
    }

    /// Boots the SargonOS with a MockNetworkingDriver that always fails.
    async fn boot_always_failing() -> Arc<SargonOS> {
        let req = SUT::boot_test_with_networking_driver(Arc::new(
            MockNetworkingDriver::new_always_failing(),
        ));

        timeout(SARGON_OS_TEST_MAX_ASYNC_DURATION, req)
            .await
            .unwrap()
            .unwrap()
    }
}
