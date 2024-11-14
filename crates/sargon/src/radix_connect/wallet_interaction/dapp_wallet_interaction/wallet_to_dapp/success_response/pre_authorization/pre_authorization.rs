use crate::prelude::*;
use radix_transactions::model::TransactionPayload;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WalletToDappInteractionPreAuthorizationResponseItems {
    pub response: WalletToDappInteractionSubintentResponseItem,
}

impl WalletToDappInteractionPreAuthorizationResponseItems {
    pub fn new(signed_subintent: SignedSubintent) -> Self {
        Self {
            response: WalletToDappInteractionSubintentResponseItem::new(
                signed_subintent,
            ),
        }
    }
}

impl HasSampleValues for WalletToDappInteractionPreAuthorizationResponseItems {
    fn sample() -> Self {
        Self {
            response: WalletToDappInteractionSubintentResponseItem::sample(),
        }
    }

    fn sample_other() -> Self {
        Self {
            response:
                WalletToDappInteractionSubintentResponseItem::sample_other(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = WalletToDappInteractionPreAuthorizationResponseItems;

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
