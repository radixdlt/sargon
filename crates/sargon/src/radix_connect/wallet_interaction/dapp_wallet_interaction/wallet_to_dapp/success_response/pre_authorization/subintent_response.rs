use crate::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct WalletToDappInteractionSubintentResponseItem {
    /// A signed subintent
    pub signed_subintent: SignedSubintent,
}

impl WalletToDappInteractionSubintentResponseItem {
    pub fn new(signed_subintent: SignedSubintent) -> Self {
        Self { signed_subintent }
    }

    fn encoded_signed_partial_transaction(&self) -> String {
        let bytes = self.signed_subintent.compiled();
        hex_encode(&bytes)
    }

    fn subintent_hash(&self) -> SubintentHash {
        self.signed_subintent.subintent.hash()
    }
}

impl Serialize for WalletToDappInteractionSubintentResponseItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct(
            "WalletToDappInteractionSubintentResponseItem",
            2,
        )?;
        state.serialize_field(
            "signedPartialTransaction",
            &self.encoded_signed_partial_transaction(),
        )?;
        state.serialize_field(
            "subintentHash",
            &self.subintent_hash().to_string(),
        )?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for WalletToDappInteractionSubintentResponseItem {
    #[cfg(not(tarpaulin_include))] // false negative
    fn deserialize<D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Self, D::Error> {
        #[derive(Deserialize, Serialize)]
        struct Wrapper {
            #[serde(rename = "signedPartialTransaction")]
            encoded_signed_partial_transaction: String,
        }
        let wrapped = Wrapper::deserialize(deserializer)?;
        let decoded = hex_decode(wrapped.encoded_signed_partial_transaction)
            .map_err(de::Error::custom)?;
        SignedSubintent::decompiling(decoded)
            .map_err(de::Error::custom)
            .map(Self::new)
    }
}

impl HasSampleValues for WalletToDappInteractionSubintentResponseItem {
    fn sample() -> Self {
        Self::new(SignedSubintent::sample())
    }

    fn sample_other() -> Self {
        Self::new(SignedSubintent::sample_other())
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
    fn json_roundtrip() {
        assert_json_roundtrip(&SUT::sample());
        assert_json_roundtrip(&SUT::sample_other());
    }

    #[test]
    fn json_failures() {
        // Test without signedPartialTransaction
        let json = r#"
        {
            "subintentHash": "subtxid_sim1kdwxe9mkpgn2n5zplvh4kcu0d69k5qcz679xhxfa8ulcjtjqsvtq799xkn"
        }
        "#;
        let result = serde_json::from_str::<SUT>(json);
        assert!(result.is_err());

        // Test with invalid signedPartialTransaction
        let json = r#"
        {
            "signedPartialTransaction": "invalid",
            "subintentHash": "subtxid_sim1kdwxe9mkpgn2n5zplvh4kcu0d69k5qcz679xhxfa8ulcjtjqsvtq799xkn"
        }
        "#;
        let result = serde_json::from_str::<SUT>(json);
        assert!(result.is_err());
    }
}
