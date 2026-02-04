use crate::prelude::*;
use std::time::Duration;

#[async_trait::async_trait]
pub trait OSCreateSubintent {
    async fn create_subintent(
        &self,
        intent_discriminator: IntentDiscriminator,
        subintent_manifest: SubintentManifest,
        expiration: DappToWalletInteractionSubintentExpiration,
        message: Option<String>,
        header: Option<DappToWalletInteractionSubintentHeader>,
    ) -> Result<Subintent>;

    fn expiry_time_from_now_in_seconds(
        &self,
        expiration: DappToWalletInteractionSubintentExpiration,
    ) -> u64;

    fn calculate_end_ranges(
        &self,
        current_epoch: Epoch,
        expiry_time_from_now_in_seconds: u64,
    ) -> (Epoch, Instant);
}

// ==================
// Create Subintent
// ==================
#[async_trait::async_trait]
impl OSCreateSubintent for SargonOS {
    /// Creates a Subintent either from the provided header directly or
    /// given its discriminator, manifest and expiration if the header is absent.
    async fn create_subintent(
        &self,
        intent_discriminator: IntentDiscriminator,
        subintent_manifest: SubintentManifest,
        expiration: DappToWalletInteractionSubintentExpiration,
        message: Option<String>,
        header: Option<DappToWalletInteractionSubintentHeader>,
    ) -> Result<Subintent> {
        let header = if let Some(provided_header) = header {
            // If a header is provided, use it.
            IntentHeaderV2 {
                network_id: NetworkID::try_from(provided_header.network_id)?,
                start_epoch_inclusive: Epoch::from(
                    provided_header.start_epoch_inclusive,
                ),
                end_epoch_exclusive: Epoch::from(
                    provided_header.end_epoch_exclusive,
                ),
                min_proposer_timestamp_inclusive: provided_header
                    .min_proposer_timestamp_inclusive
                    .map(Instant::from),
                max_proposer_timestamp_exclusive: provided_header
                    .max_proposer_timestamp_exclusive
                    .map(Instant::from),
                intent_discriminator: IntentDiscriminator::from(
                    provided_header.intent_discriminator,
                ),
            }
        } else {
            // Calculate the seconds until the expiration of the subintent.
            let expiry_time_from_now_in_seconds =
                self.expiry_time_from_now_in_seconds(expiration);
            if expiry_time_from_now_in_seconds == 0 {
                return Err(CommonError::SubintentExpired);
            }

            // Get current epoch
            let (gateway_client, network_id) = self.gateway_client_on()?;
            let current_epoch = gateway_client.current_epoch().await?;

            // Calculate header ranges
            let end_ranges = self.calculate_end_ranges(
                current_epoch,
                expiry_time_from_now_in_seconds,
            );

            // Build header
            IntentHeaderV2 {
                network_id,
                start_epoch_inclusive: current_epoch,
                end_epoch_exclusive: end_ranges.0,
                min_proposer_timestamp_inclusive: None,
                max_proposer_timestamp_exclusive: Some(end_ranges.1),
                intent_discriminator,
            }
        };

        // Build subintent
        Subintent::new(header, subintent_manifest, message.into())
    }

    /// Returns the seconds until the expiration of the subintent.
    fn expiry_time_from_now_in_seconds(
        &self,
        expiration: DappToWalletInteractionSubintentExpiration,
    ) -> u64 {
        match expiration {
            DappToWalletInteractionSubintentExpiration::AtTime(at_time) => {
                let now = seconds_since_unix_epoch();
                let expiration = at_time.unix_timestamp_seconds;
                expiration.saturating_sub(now)
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
        let epoch_diff = MAX_EPOCH_DIFF.min(
            MIN_EPOCH_DIFF
                + (expiry_time_from_now_in_seconds / EPOCH_DURATION_IN_SECONDS),
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
    use std::time::Duration;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonOS;

    #[actix_rt::test]
    async fn create_subintent_expired() {
        // Test the case where the subintent is already expired
        let os = boot_success().await;

        let expiration =
            DappToWalletInteractionSubintentExpiration::AtTime(100.into());

        let result = os
            .create_subintent(
                IntentDiscriminator::sample(),
                SubintentManifest::sample(),
                expiration,
                None,
                None,
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
        let message = "Hello Radix!".to_string();

        let result = os
            .create_subintent(
                intent_discriminator,
                manifest.clone(),
                expiration,
                Some(message.clone()),
                None,
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
        assert_eq!(result.message, MessageV2::plain_text(message));
    }

    #[actix_rt::test]
    async fn expiry_time() {
        let os = boot_always_failing().await;

        // Check for AtTime expiration
        let now = seconds_since_unix_epoch();
        let pending_time = 32423;

        let mut expiration = DappToWalletInteractionSubintentExpiration::AtTime(
            (now + pending_time).into(),
        );

        let mut result = os.expiry_time_from_now_in_seconds(expiration);
        let diff = pending_time.abs_diff(result);
        assert!(diff <= 1); // Less than 1s difference

        // Check for AfterDelay expiration
        expiration = DappToWalletInteractionSubintentExpiration::AfterDelay(
            DappToWalletInteractionSubintentExpireAfterDelay::from(
                pending_time,
            ),
        );
        result = os.expiry_time_from_now_in_seconds(expiration);
        assert_eq!(result, pending_time);
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
