use crate::prelude::*;
use radix_transactions::manifest::DefaultTestExecutionConfigType::System;
use std::time::Duration;

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
        let expiry_time_from_now_in_seconds =
            self.expiry_time_from_now_in_seconds(expiration);
        let end_ranges = self.calculate_end_ranges(
            current_epoch,
            expiry_time_from_now_in_seconds,
        );

        let header = IntentHeaderV2 {
            network_id,
            start_epoch_inclusive: current_epoch,
            end_epoch_exclusive: end_ranges.0,
            min_proposer_timestamp_inclusive: None,
            max_proposer_timestamp_exclusive: Some(end_ranges.1),
            intent_discriminator,
        };

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
        // As per the transaction validation configuration, epoch diff should be less than 1 month.
        const MAX_EPOCH_DIFF: u64 = 12 * 24 * 30;

        // 1 epoch for the fact that it's min_inclusive and max_exclusive; 1 more for the fact that we might be very close to the end of the epoch already
        const MIN_EPOCH_DIFF: u64 = 2;

        // 5 minutes
        const EXPECTED_EPOCH_DURATION_IN_SECONDS: u64 = 300;

        let epoch_diff = MAX_EPOCH_DIFF.min(
            MIN_EPOCH_DIFF
                + (expiry_time_from_now_in_seconds
                    / EXPECTED_EPOCH_DURATION_IN_SECONDS),
        );
        let end_epoch_exclusive = current_epoch.adding(epoch_diff);
        let seconds_since_unix_epoch = Timestamp::now_utc()
            .duration_since(Timestamp::UNIX_EPOCH)
            .as_seconds_f64() as i64;
        let max_proposer_timestamp_exclusive = Instant::from(
            seconds_since_unix_epoch + expiry_time_from_now_in_seconds as i64,
        );
        (end_epoch_exclusive, max_proposer_timestamp_exclusive)
    }
}
