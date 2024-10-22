use crate::prelude::*;

/// Represents the header of an intent in V2, containing network ID,
/// epoch range, optional proposer timestamps, and an intent discriminator.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    uniffi::Record,
)]
#[display("{} intent_discriminator: {}", network_id, intent_discriminator)]
pub struct IntentHeaderV2 {
    pub network_id: NetworkID,
    pub start_epoch_inclusive: Epoch,
    pub end_epoch_exclusive: Epoch,
    pub min_proposer_timestamp_inclusive: Option<Instant>,
    pub max_proposer_timestamp_exclusive: Option<Instant>,

    /// This field is intended to enable a network user to generate an identical intent with
    /// a new hash. Users can simply set this randomly if they wish to. A u64 is large
    /// enough to avoid any risk of collision over the course of a single epoch anyway.
    ///
    /// This field's name intent_discriminator is the new name for what was the nonce field in
    /// IntentV1. This was poorly named, as it caused confusion with an Ethereum-style nonce.
    pub intent_discriminator: IntentDiscriminator,
}

impl IntentHeaderV2 {
    /// Creates a new `IntentHeaderV2`
    ///
    /// # Panics
    /// Panics if `end_epoch_exclusive` is less than `start_epoch_inclusive` or if
    /// `max_proposer_timestamp_exclusive` is less than `min_proposer_timestamp_inclusive`.
    pub fn new(
        network_id: NetworkID,
        start_epoch_inclusive: impl Into<Epoch>,
        end_epoch_exclusive: impl Into<Epoch>,
        min_proposer_timestamp_inclusive: Option<Instant>,
        max_proposer_timestamp_exclusive: Option<Instant>,
        intent_discriminator: impl Into<IntentDiscriminator>,
    ) -> Self {
        let start_epoch_inclusive = start_epoch_inclusive.into();
        let end_epoch_exclusive = end_epoch_exclusive.into();
        let intent_discriminator = intent_discriminator.into();
        assert!(
            end_epoch_exclusive >= start_epoch_inclusive,
            "End epoch MUST be greater than or equal start epoch."
        );

        if let (Some(min_ts), Some(max_ts)) = (
            min_proposer_timestamp_inclusive,
            max_proposer_timestamp_exclusive,
        ) {
            assert!(
                max_ts >= min_ts,
                "Max proposer timestamp MUST be greater than or equal min proposer timestamp."
            );
            assert!(
                min_ts.seconds_since_unix_epoch
                    >= start_epoch_inclusive.0 as i64,
                "Min proposer timestamp MUST be within the epoch window."
            );
            assert!(
                max_ts.seconds_since_unix_epoch <= end_epoch_exclusive.0 as i64,
                "Max proposer timestamp MUST be within the epoch window."
            );
        }
        Self {
            network_id,
            start_epoch_inclusive,
            end_epoch_exclusive,
            min_proposer_timestamp_inclusive,
            max_proposer_timestamp_exclusive,
            intent_discriminator,
        }
    }
}

impl From<IntentHeaderV2> for ScryptoIntentHeaderV2 {
    fn from(value: IntentHeaderV2) -> Self {
        Self {
            network_id: value.network_id.into(),
            start_epoch_inclusive: value.start_epoch_inclusive.into(),
            end_epoch_exclusive: value.end_epoch_exclusive.into(),
            min_proposer_timestamp_inclusive: value
                .min_proposer_timestamp_inclusive
                .map(Into::into),
            max_proposer_timestamp_exclusive: value
                .max_proposer_timestamp_exclusive
                .map(Into::into),
            intent_discriminator: value.intent_discriminator.into(),
        }
    }
}

impl TryFrom<ScryptoIntentHeaderV2> for IntentHeaderV2 {
    type Error = crate::CommonError;

    fn try_from(value: ScryptoIntentHeaderV2) -> Result<Self, Self::Error> {
        let network_id: NetworkID = value.network_id.try_into()?;
        Ok(Self {
            network_id,
            start_epoch_inclusive: value.start_epoch_inclusive.into(),
            end_epoch_exclusive: value.end_epoch_exclusive.into(),
            min_proposer_timestamp_inclusive: value
                .min_proposer_timestamp_inclusive
                .map(Into::into),
            max_proposer_timestamp_exclusive: value
                .max_proposer_timestamp_exclusive
                .map(Into::into),
            intent_discriminator: value.intent_discriminator.into(),
        })
    }
}

impl HasSampleValues for IntentHeaderV2 {
    fn sample() -> Self {
        Self::new(
            NetworkID::Mainnet,
            76935,
            76945,
            Some(76938.into()),
            Some(76940.into()),
            IntentDiscriminator::sample(),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            NetworkID::Simulator,
            0,
            10,
            None,
            None,
            IntentDiscriminator::sample_other(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = IntentHeaderV2;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn to_from_scrypto() {
        let roundtrip =
            |s: SUT| SUT::try_from(ScryptoIntentHeaderV2::from(s)).unwrap();
        roundtrip(SUT::sample());
        roundtrip(SUT::sample_other());
    }

    #[test]
    #[should_panic(
        expected = "End epoch MUST be greater than or equal start epoch."
    )]
    fn panics_if_end_epoch_is_smaller_than_start() {
        _ = SUT::new(NetworkID::Mainnet, 237, 236, None, None, 421337237)
    }

    #[test]
    #[should_panic(
        expected = "Max proposer timestamp MUST be greater than or equal min proposer timestamp."
    )]
    fn panics_if_max_proposer_is_smaller_than_min_proposer() {
        _ = SUT::new(
            NetworkID::Mainnet,
            237,
            247,
            Some(1728481000.into()),
            Some(1728480000.into()),
            421337237,
        )
    }

    #[test]
    #[should_panic(
        expected = "Min proposer timestamp MUST be within the epoch window."
    )]
    fn panics_if_min_proposer_is_not_within_epoch_window() {
        _ = SUT::new(
            NetworkID::Mainnet,
            237,
            247,
            Some(227.into()),
            Some(247.into()),
            421337237,
        )
    }

    #[test]
    #[should_panic(
        expected = "Max proposer timestamp MUST be within the epoch window."
    )]
    fn panics_if_max_proposer_is_not_within_epoch_window() {
        _ = SUT::new(
            NetworkID::Mainnet,
            237,
            247,
            Some(237.into()),
            Some(257.into()),
            421337237,
        )
    }
}
