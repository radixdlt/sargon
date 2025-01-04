use crate::prelude::*;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(tag = "discriminator")]
pub enum DappToWalletInteractionItems {
    #[serde(rename = "unauthorizedRequest")]
    UnauthorizedRequest(DappToWalletInteractionUnauthorizedRequestItems),

    #[serde(rename = "authorizedRequest")]
    AuthorizedRequest(DappToWalletInteractionAuthorizedRequestItems),

    #[serde(rename = "transaction")]
    Transaction(DappToWalletInteractionTransactionItems),

    #[serde(rename = "preAuthorizationRequest")]
    PreAuthorization(DappToWalletInteractionPreAuthorizationItems),
}

impl HasSampleValues for DappToWalletInteractionItems {
    fn sample() -> Self {
        Self::UnauthorizedRequest(
            DappToWalletInteractionUnauthorizedRequestItems::sample(),
        )
    }

    fn sample_other() -> Self {
        Self::Transaction(DappToWalletInteractionTransactionItems::sample())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DappToWalletInteractionItems;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }
}
