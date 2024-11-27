use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WalletToDappInteractionSubintentResponseItem {
    /// A hex encoded signed partial transaction.
    #[serde(rename = "signedPartialTransaction")]
    pub encoded_signed_partial_transaction: String,

    /// The hash of the generated subintent.
    #[serde(rename = "subintentHash")]
    pub subintent_hash: String,
}

impl WalletToDappInteractionSubintentResponseItem {
    pub fn new(signed_subintent: SignedSubintent) -> Self {
        Self {
            encoded_signed_partial_transaction: hex_encode(
                signed_subintent.compiled(),
            ),
            subintent_hash: signed_subintent.subintent.hash().to_string(),
        }
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
    fn new() {
        let signed_subintent = SignedSubintent::sample();
        let sut = SUT::new(signed_subintent);
        assert_eq!(sut, SUT::sample());
    }
}
