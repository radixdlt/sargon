use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "discriminator")]
#[allow(clippy::large_enum_variant)]
pub enum WalletToDappInteractionResponseItems {
    #[serde(rename = "authorizedRequest")]
    AuthorizedRequest(WalletToDappInteractionAuthorizedRequestResponseItems),

    #[serde(rename = "unauthorizedRequest")]
    UnauthorizedRequest(
        WalletToDappInteractionUnauthorizedRequestResponseItems,
    ),

    #[serde(rename = "transaction")]
    Transaction(WalletToDappInteractionTransactionResponseItems),

    #[serde(rename = "preAuthorizationResponse")]
    PreAuthorization(WalletToDappInteractionPreAuthorizationResponseItems),
}

impl HasSampleValues for WalletToDappInteractionResponseItems {
    fn sample() -> Self {
        WalletToDappInteractionResponseItems::AuthorizedRequest(
            WalletToDappInteractionAuthorizedRequestResponseItems::sample(),
        )
    }
    fn sample_other() -> Self {
        WalletToDappInteractionResponseItems::Transaction(
            WalletToDappInteractionTransactionResponseItems::sample_other(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = WalletToDappInteractionResponseItems;

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
