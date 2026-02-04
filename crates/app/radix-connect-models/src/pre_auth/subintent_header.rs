use crate::prelude::*;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DappToWalletInteractionSubintentHeader {
    pub network_id: u8,
    pub start_epoch_inclusive: u64,
    pub end_epoch_exclusive: u64,
    pub min_proposer_timestamp_inclusive: Option<i64>,
    pub max_proposer_timestamp_exclusive: Option<i64>,
    pub intent_discriminator: u64,
}

impl DappToWalletInteractionSubintentHeader {
    pub fn new(
        network_id: u8,
        start_epoch_inclusive: u64,
        end_epoch_exclusive: u64,
        min_proposer_timestamp_inclusive: Option<i64>,
        max_proposer_timestamp_exclusive: Option<i64>,
        intent_discriminator: u64,
    ) -> Self {
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

impl HasSampleValues for DappToWalletInteractionSubintentHeader {
    fn sample() -> Self {
        Self::new(
            NetworkID::Mainnet.discriminant(),
            0,
            1,
            Some(1694448356),
            Some(1703438036),
            123456789,
        )
    }

    fn sample_other() -> Self {
        Self::new(
            NetworkID::Stokenet.discriminant(),
            1,
            2,
            Some(1694448356),
            Some(1703438036),
            123456790,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DappToWalletInteractionSubintentHeader;

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
    fn json_roundtrip() {
        assert_eq_after_json_roundtrip(
            &SUT::sample(),
            r#"
            {
              "networkId": 1,
              "startEpochInclusive": 0,
              "endEpochExclusive": 1,
              "minProposerTimestampInclusive": 1694448356,
              "maxProposerTimestampExclusive": 1703438036,
              "intentDiscriminator": 123456789
            }
        "#,
        );

        assert_eq_after_json_roundtrip(
            &SUT::sample_other(),
            r#"
            {
              "networkId": 2,
              "startEpochInclusive": 1,
              "endEpochExclusive": 2,
              "minProposerTimestampInclusive": 1694448356,
              "maxProposerTimestampExclusive": 1703438036,
              "intentDiscriminator": 123456790
            }
        "#,
        );
    }
}
