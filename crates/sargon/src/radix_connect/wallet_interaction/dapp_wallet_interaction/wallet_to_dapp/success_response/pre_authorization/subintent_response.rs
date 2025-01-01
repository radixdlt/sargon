use crate::prelude::*;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq)]
pub struct WalletToDappInteractionSubintentResponseItem {
    /// A signed subintent
    pub signed_subintent: SignedSubintent,

    /// The timestamp at which the subintent expires
    pub expiration_timestamp: Instant,
}

impl WalletToDappInteractionSubintentResponseItem {
    pub fn new(
        signed_subintent: SignedSubintent,
        expiration_timestamp: Instant,
    ) -> Self {
        Self {
            signed_subintent,
            expiration_timestamp,
        }
    }

    fn encoded_signed_partial_transaction(&self) -> String {
        let bytes = self.signed_subintent.compiled();
        hex_encode(&bytes)
    }

    fn subintent_hash(&self) -> SubintentHash {
        self.signed_subintent.subintent.hash()
    }
}

impl From<SignedSubintent> for WalletToDappInteractionSubintentResponseItem {
    fn from(value: SignedSubintent) -> Self {
        let expiration_timestamp = value
            .subintent
            .header
            .max_proposer_timestamp_exclusive
            .expect(
                "A SignedSubintent must have max_proposer_timestamp_exclusive",
            );
        Self::new(value, expiration_timestamp)
    }
}

impl Serialize for WalletToDappInteractionSubintentResponseItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct(
            "WalletToDappInteractionSubintentResponseItem",
            3,
        )?;
        state.serialize_field(
            "signedPartialTransaction",
            &self.encoded_signed_partial_transaction(),
        )?;
        state.serialize_field(
            "subintentHash",
            &self.subintent_hash().to_string(),
        )?;
        state.serialize_field(
            "expirationTimestamp",
            &self.expiration_timestamp.seconds_since_unix_epoch,
        )?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for WalletToDappInteractionSubintentResponseItem {
    fn deserialize<D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Self, D::Error> {
        #[derive(Deserialize, Serialize)]
        struct Wrapper {
            #[serde(rename = "signedPartialTransaction")]
            encoded_signed_partial_transaction: String,

            #[serde(rename = "expirationTimestamp")]
            expiration_timestamp_seconds: i64,
        }
        let wrapped = Wrapper::deserialize(deserializer)?;
        let decoded = hex_decode(wrapped.encoded_signed_partial_transaction)
            .map_err(de::Error::custom)?;
        let expiration_timestamp =
            Instant::from(wrapped.expiration_timestamp_seconds);
        SignedSubintent::decompiling(decoded)
            .map_err(de::Error::custom)
            .map(|s| Self::new(s, expiration_timestamp))
    }
}

impl HasSampleValues for WalletToDappInteractionSubintentResponseItem {
    fn sample() -> Self {
        Self::new(SignedSubintent::sample(), Instant::sample())
    }

    fn sample_other() -> Self {
        Self::new(SignedSubintent::sample_other(), Instant::sample_other())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = WalletToDappInteractionSubintentResponseItem;

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
    fn json_success() {
        assert_json_roundtrip(&SUT::sample());

        assert_eq_after_json_roundtrip(
            &SUT::sample_other(),
            r#"
        {
            "signedPartialTransaction": "4d220e03210221012105210607f20a00000000000000000a0a000000000000002200002200000ab168de3a00000000202000220000202000202200202100202200202000",
            "subintentHash": "subtxid_sim1kdwxe9mkpgn2n5zplvh4kcu0d69k5qcz679xhxfa8ulcjtjqsvtq799xkn",
            "expirationTimestamp": 1703438036
        }
        "#,
        );
    }

    #[test]
    fn json_failures() {
        // Test without signedPartialTransaction
        let json = r#"
        {
            "subintentHash": "subtxid_sim1kdwxe9mkpgn2n5zplvh4kcu0d69k5qcz679xhxfa8ulcjtjqsvtq799xkn",
            "expirationTimestamp": 1730999831257
        }
        "#;
        let result = serde_json::from_str::<SUT>(json);
        assert!(result.is_err());

        // Test with invalid signedPartialTransaction
        let json = r#"
        {
            "signedPartialTransaction": "invalid",
            "subintentHash": "subtxid_sim1kdwxe9mkpgn2n5zplvh4kcu0d69k5qcz679xhxfa8ulcjtjqsvtq799xkn",
            "expirationTimestamp": 1730999831257
        }
        "#;
        let result = serde_json::from_str::<SUT>(json);
        assert!(result.is_err());

        // Test without expirationTimestamp
        let json = r#"
        {
            "signedPartialTransaction": "4d220e03210221012105210607f20a00000000000000000a0a000000000000002200002200000ab168de3a00000000202000220000202000202200202100202200202000",
            "subintentHash": "subtxid_sim1kdwxe9mkpgn2n5zplvh4kcu0d69k5qcz679xhxfa8ulcjtjqsvtq799xkn"
        }
        "#;
        let result = serde_json::from_str::<SUT>(json);
        assert!(result.is_err());
    }

    #[test]
    fn from() {
        let signed_subintent = SignedSubintent::sample();
        let result = SUT::from(signed_subintent.clone());
        assert_eq!(result.signed_subintent, signed_subintent);
        assert_eq!(
            result.expiration_timestamp,
            signed_subintent
                .subintent
                .header
                .max_proposer_timestamp_exclusive
                .unwrap()
        );
    }

    #[test]
    #[should_panic(
        expected = "A SignedSubintent must have max_proposer_timestamp_exclusive"
    )]
    fn from_without_max_proposer_timestamp() {
        let signed_subintent = SignedSubintent::sample_other();
        let _ = SUT::from(signed_subintent);
    }
}
